# Mandelbrot Set Generator

A high-performance Mandelbrot set visualization tool written in Rust that generates detailed zoom animations of the Mandelbrot set fractal.

## Features

- Generate high-resolution Mandelbrot set visualizations
- Create smooth zoom animations as GIF files
- Multi-threaded rendering for optimal performance
- Customizable parameters (resolution, coordinates, zoom factor, etc.)
- Command-line interface with sensible defaults

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2021 Edition or newer)
- Cargo (comes with Rust)

### Building from Source

Clone the repository and build with Cargo:

```bash
# Clone the repository
git clone https://github.com/yourusername/mandelbrot.git
cd mandelbrot

# Build in release mode for optimal performance
cargo build --release
```

The compiled binary will be available at `./target/release/mandelbrot`.

## Usage

Run the program with the default settings:

```bash
./target/release/mandelbrot
```

### Command Line Options

```bash
USAGE:
    mandelbrot [OPTIONS]

OPTIONS:
    -r, --result-folder <FOLDER>     Directory where output images and gif will be saved [default: results]
    -p, --pixels <WIDTHxHEIGHT>      Image dimensions [default: 1024x1024]
    -u, --upper-left <REAL,IMAG>     Upper left corner coordinates [default: -1.0,-1.0]
    -l, --lower-right <REAL,IMAG>    Lower right corner coordinates [default: 1.0,1.0]
    -s, --scale-factor <FACTOR>      Scaling factor for each frame (e.g., 0.9 for zoom in) [default: 0.9]
    -n, --n-frames <COUNT>           Number of frames to generate [default: 10]
    -d, --delay <DELAY>              Delay between frames in hundredths of a second [default: 20]
    -t, --threads <COUNT>            Number of threads to use for rendering [default: 8]
    -h, --help                       Print help information
    -V, --version                    Print version information
```

### Examples

Generate a 50-frame zooming animation at 800x600 resolution:

```bash
./target/release/mandelbrot --pixels 800x600 --n-frames 50
```

Zoom into a specific interesting region:

```bash
./target/release/mandelbrot --upper-left -0.7436,-0.1262 --lower-right -0.7396,-0.1222 --n-frames 100 --scale-factor 0.95
```

Use more threads on a powerful system:

```bash
./target/release/mandelbrot --threads 16
```

## How It Works

The Mandelbrot set is a complex mathematical fractal defined by the equation:

```math
z_{n+1} = z_n^2 + c
```

For each pixel in the image, we determine whether the corresponding complex number $c$ results in a bounded sequence when iteratively applying the equation. The rendering is done in parallel using multiple threads to maximize performance.

## Performance

The application uses the following optimizations:

- Parallel rendering with Rust's crossbeam library
- Division of the image into bands for thread workload balancing
- Release builds for maximum performance

## Dependencies

- [crossbeam](https://crates.io/crates/crossbeam) - Parallelization
- [image](https://crates.io/crates/image) - Image processing
- [gif](https://crates.io/crates/gif) - GIF creation
- [num](https://crates.io/crates/num) - Complex number operations
- [clap](https://crates.io/crates/clap) - Command-line argument parsing
- [indicatif](https://crates.io/crates/indicatif) - Progress bars

## License

This project is licensed under the MIT License - see the LICENSE file for details.
