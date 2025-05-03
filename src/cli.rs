use crate::utils::FractalType;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Clone, Debug, ValueEnum)]
pub enum FractalTypeArg {
    Mandelbrot,
    Julia,
    BurningShip,
    Tricorn,
    Nova,
    Sin,
    Cos,
}

impl From<FractalTypeArg> for FractalType {
    fn from(value: FractalTypeArg) -> Self {
        match value {
            FractalTypeArg::Mandelbrot => FractalType::Mandelbrot,
            FractalTypeArg::Julia => FractalType::Julia,
            FractalTypeArg::BurningShip => FractalType::BurningShip,
            FractalTypeArg::Tricorn => FractalType::Tricorn,
            FractalTypeArg::Nova => FractalType::Nova,
            FractalTypeArg::Sin => FractalType::Sin,
            FractalTypeArg::Cos => FractalType::Cos,
        }
    }
}

#[derive(Parser)]
#[command(
    name = "mandelbrot",
    about = "Generate fractal zoom animation",
    version,
    long_about = None
)]
pub struct Cli {
    /// Directory where output images and gif will be saved
    #[arg(short, long, default_value = "results")]
    pub output_folder: PathBuf,

    /// Image dimensions in format WIDTHxHEIGHT (e.g., 1000x750)
    #[arg(long, default_value = "1024x1024")]
    pub pixels: String,

    /// Upper left corner coordinates in format REAL,IMAGINARY (e.g., -1.20,0.35)
    #[arg(short, long, default_value = "-2.0,-2.0")]
    pub upper_left: String,

    /// Lower right corner coordinates in format REAL,IMAGINARY (e.g., -1,0.20)
    #[arg(short, long, default_value = "2.0,2.0")]
    pub lower_right: String,

    /// Scaling factor for each frame (e.g., 0.9 for zoom in)
    #[arg(short, long, default_value_t = 0.95)]
    pub scale_factor: f64,

    /// Power of the fractal (e.g., 2 for standard Mandelbrot)
    #[arg(long, default_value_t = 2)]
    pub power: i32,

    /// Escape radius for the fractal set
    #[arg(short, long, default_value_t = 2.0)]
    pub escape_radius: f64,

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

    /// The type of fractal to generate
    #[arg(long, value_enum, default_value = "mandelbrot")]
    pub fractal_type: FractalTypeArg,

    /// Constant for Julia sets in format REAL,IMAGINARY (e.g., -0.8,0.156)
    #[arg(short, long, default_value = "-0.8,0.156")]
    pub julia_constant: String,
}
