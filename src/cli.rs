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
    #[arg(short, long, default_value = "1024x1024")]
    pub pixels: String,

    /// Upper left corner coordinates in format REAL,IMAGINARY (e.g., -1.20,0.35)
    #[arg(short, long, default_value = "-1.0,-1.0")]
    pub upper_left: String,

    /// Lower right corner coordinates in format REAL,IMAGINARY (e.g., -1,0.20)
    #[arg(short, long, default_value = "1.0,1.0")]
    pub lower_right: String,

    /// Scaling factor for each frame (e.g., 0.9 for zoom in)
    #[arg(short, long, default_value_t = 0.9)]
    pub scale_factor: f64,

    /// Number of frames to generate
    #[arg(short, long, default_value_t = 10)]
    pub n_frames: usize,

    /// Delay between frames in hundredths of a second (e.g., 10 = 0.1 seconds)
    #[arg(short, long, default_value_t = 20)]
    pub delay: u16,

    /// Number of threads to use for rendering
    #[arg(short, long, default_value_t = 8)]
    pub threads: usize,
}
