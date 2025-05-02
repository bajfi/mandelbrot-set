use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "mandelbrot",
    about = "Generate Mandelbrot set zoom animation",
    version,
    long_about = None
)]
pub struct Cli {
    /// Directory where output images and gif will be saved
    #[arg(short, long, default_value = "results")]
    pub result_folder: PathBuf,

    /// Image dimensions in format WIDTHxHEIGHT (e.g., 1000x750)
    #[arg(long, default_value = "1024x1024")]
    pub pixels: String,

    /// Upper left corner coordinates in format REAL,IMAGINARY (e.g., -1.20,0.35)
    #[arg(short, long, default_value = "-2.0,-1.0")]
    pub upper_left: String,

    /// Lower right corner coordinates in format REAL,IMAGINARY (e.g., -1,0.20)
    #[arg(short, long, default_value = "1.0,1.0")]
    pub lower_right: String,

    /// Scaling factor for each frame (e.g., 0.9 for zoom in)
    #[arg(short, long, default_value_t = 0.95)]
    pub scale_factor: f64,

    /// Number of frames to generate
    #[arg(short, long, default_value_t = 100)]
    pub n_frames: usize,

    /// Delay between frames in hundredths of a second (e.g., 10 = 0.1 seconds)
    #[arg(short, long, default_value_t = 15)]
    pub delay: u16,

    /// Number of threads to use for rendering
    #[arg(long, default_value_t = 8)]
    pub threads: usize,

    /// The scale pointer (0, 0) is the upper left corner and (1, 1) is the lower right corner
    #[arg(short, long, default_value = "-1.4002,0.0")]
    pub pointer: String,

    /// Whether to save the frames results
    #[arg(long, default_value_t = false)]
    pub no_frames: bool,
}
