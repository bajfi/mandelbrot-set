mod cli;
mod utils;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let cli = cli::Cli::parse();

    // Create the output directory if it doesn't exist
    if !cli.result_folder.exists() {
        std::fs::create_dir_all(&cli.result_folder).expect("Error creating output directory");
    }

    let bounds = utils::parse_pair(&cli.pixels, 'x').expect("Error parsing image dimensions");
    let mut upper_left =
        utils::parse_complex(&cli.upper_left).expect("Error parsing upper left corner point");
    let mut lower_right =
        utils::parse_complex(&cli.lower_right).expect("Error parsing lower right corner point");
    let scale_factor = cli.scale_factor;
    let n_frames = cli.n_frames;

    let mut pixels = vec![0; bounds.0 * bounds.1];

    // Collect frame paths for later GIF creation
    let mut frame_paths: Vec<String> = Vec::with_capacity(n_frames);

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
                        utils::pixel_to_point(bounds, (0, top), upper_left, lower_right);
                    let band_lower_right = utils::pixel_to_point(
                        bounds,
                        (bounds.0, top + height),
                        upper_left,
                        lower_right,
                    );

                    spawner.spawn(move |_| {
                        utils::render(band, band_bounds, band_upper_left, band_lower_right);
                    });
                }
            })
            .unwrap();
        }

        // Create frames directory if it doesn't exist
        let frames_dir = cli.result_folder.join("frames");
        if !frames_dir.exists() {
            std::fs::create_dir_all(&frames_dir).expect("Error creating frames directory");
        }

        // Write the image to a file
        let frame_name = format!(
            "{}/frames/frame-{:03}.png",
            cli.result_folder.display(),
            i + 1
        );
        utils::write_image(&frame_name, &pixels, bounds).expect("Error writing PNG file");

        // Add frame path to our collection for GIF creation
        frame_paths.push(frame_name);

        // Scale the view
        (upper_left, lower_right) = utils::scale(upper_left, lower_right, scale_factor);

        // Update progress bar
        progress_bar.inc(1);
        progress_bar.set_message(format!("Frame {}/{} complete", i + 1, n_frames));
    }

    // Finish progress bar
    progress_bar.finish_with_message("All frames rendered");

    // After generating all frames, create a GIF animation
    println!("Creating GIF from {} frames...", frame_paths.len());
    let gif_path = format!("{}/mandelbrot_zoom.gif", cli.result_folder.display());

    // Add progress bar for GIF creation
    let gif_progress = ProgressBar::new_spinner();
    gif_progress.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    gif_progress.set_message("Creating GIF animation...");
    gif_progress.enable_steady_tick(std::time::Duration::from_millis(100));

    utils::make_gif(frame_paths, &gif_path, cli.delay).expect("Error creating GIF file");

    gif_progress.finish_with_message(format!("GIF created at: {}", gif_path));
}
