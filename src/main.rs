mod cli;
mod utils;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use tempfile::TempDir;
use utils::FractalType;

fn main() {
    let cli = cli::Cli::parse();

    // Create the output directory if it doesn't exist
    if !cli.output_folder.exists() {
        std::fs::create_dir_all(&cli.output_folder).expect("Error creating output directory");
    }

    // Determine where to store frames
    // If no_frames is true, use a temporary directory
    // Otherwise, use a subdirectory in the result folder
    let frames_dir = if cli.no_frames {
        // Create temporary directory that will be automatically deleted when dropped
        let dir = TempDir::new().expect("Error creating temporary directory");
        dir.path().to_path_buf()
    } else {
        // Use the frames directory in the result folder
        let dir = cli.output_folder.join("frames");
        if !dir.exists() {
            std::fs::create_dir_all(&dir).expect("Error creating frames directory");
        }
        dir
    };

    // Parse the arguments from the command line interface
    let bounds = utils::parse_pair(&cli.pixels, 'x').expect("Error parsing image dimensions");
    let mut upper_left =
        utils::parse_complex(&cli.upper_left).expect("Error parsing upper left corner point");
    let mut lower_right =
        utils::parse_complex(&cli.lower_right).expect("Error parsing lower right corner point");
    let scale_factor = cli.scale_factor;
    let power = cli.power;
    let escape_radius = cli.escape_radius;
    let scale_pointer = utils::parse_complex(&cli.pointer).expect("Error parsing scale pointer");
    let n_frames = cli.n_frames;

    // Get the fractal type from CLI
    let fractal_type: FractalType = cli.fractal_type.into();

    // Parse Julia set constant if needed
    let julia_constant = match fractal_type {
        FractalType::Julia => {
            Some(utils::parse_complex(&cli.julia_constant).expect("Error parsing Julia constant"))
        }
        _ => None,
    };

    // The size of the pixel buffer is width * height
    let mut pixels = vec![0; bounds.0 * bounds.1];

    // Collect frame paths for later GIF creation
    let mut frame_paths: Vec<String> = Vec::with_capacity(n_frames);

    // Get fractal name for file naming
    let fractal_name = match fractal_type {
        FractalType::Mandelbrot => "mandelbrot",
        FractalType::Julia => "julia",
        FractalType::BurningShip => "burning_ship",
        FractalType::Tricorn => "tricorn",
        FractalType::Nova => "nova",
        FractalType::Sin => "sin",
        FractalType::Cos => "cos",
    };

    // Setup progress bar for frame generation
    let progress_bar = ProgressBar::new(n_frames as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{pos}/{len}] frames {bar:50.cyan/blue} [{elapsed_precise}] {msg}")
            .unwrap(),
    );

    let threads = cli.threads;
    let rows_per_band = bounds.1 / threads + 1;
    for i in 0..n_frames {
        {
            let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
            crossbeam::scope(|spawner| {
                for (band_idx, band) in bands.into_iter().enumerate() {
                    let top = rows_per_band * band_idx;
                    let height = band.len() / bounds.0;
                    let band_bounds = (bounds.0, height);
                    let band_upper_left =
                        utils::transform::pixel_to_point(bounds, (0, top), upper_left, lower_right);
                    let band_lower_right = utils::transform::pixel_to_point(
                        bounds,
                        (bounds.0, top + height),
                        upper_left,
                        lower_right,
                    );

                    // Clone the julia_constant for the current band
                    let band_julia_constant = julia_constant.clone();

                    spawner.spawn(move |_| {
                        utils::render(
                            band,
                            band_bounds,
                            band_upper_left,
                            band_lower_right,
                            power,
                            escape_radius,
                            fractal_type,
                            band_julia_constant,
                        );
                    });
                }
            })
            .unwrap();
        }

        // Write the image to a file in the appropriate directory
        let frame_name = format!("{}/{}-{:03}.png", frames_dir.display(), fractal_name, i + 1);
        utils::preserve::write_image(&frame_name, &pixels, bounds).expect("Error writing PNG file");

        // Add frame path to our collection for GIF creation
        frame_paths.push(frame_name);

        // Scale the view
        (upper_left, lower_right) = (
            utils::transform::scale_point(upper_left, scale_pointer, scale_factor),
            utils::transform::scale_point(lower_right, scale_pointer, scale_factor),
        );

        // Update progress bar
        progress_bar.inc(1);
        progress_bar.set_message(format!("Frame {}/{} complete", i + 1, n_frames));
    }

    // Finish progress bar
    progress_bar.finish_with_message("All frames rendered");

    // After generating all frames, create a GIF animation
    println!("Creating GIF from {} frames...", frame_paths.len());
    let gif_path = format!("{}/{}.gif", cli.output_folder.display(), fractal_name);

    // Add progress bar for GIF creation
    let gif_progress = ProgressBar::new_spinner();
    gif_progress.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    gif_progress.set_message("Creating GIF animation...");
    gif_progress.enable_steady_tick(std::time::Duration::from_millis(100));

    utils::preserve::make_gif(frame_paths, &gif_path, cli.delay).expect("Error creating GIF file");

    gif_progress.finish_with_message(format!("GIF created at: {}", gif_path));
}
