use num::Complex;
/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius 2 centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`.
pub fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

use std::str::FromStr;
/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are
/// both strings that can be parsed by `T::from_str`. `separator` must be an
/// ASCII character.
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse
/// correctly, return `None`.
pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}
#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// Given the row and column of a pixel in the output image, return the
/// Parse a pair of floating-point numbers separated by a comma as a complex number.
pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    parse_pair(s, ',').map(|(re, im)| Complex { re, im })
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("0.5,1.5"), Some(Complex { re: 0.5, im: 1.5 }));
    assert_eq!(parse_complex("0.5,1.5x"), None);
}

/// corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex
/// plane designating the area our image covers.
pub fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64, // Why subtraction here? pixel.1 increases as we go down,
                                                                       // but the imaginary component increases as we go up.
    }
}
#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-
/// left and lower-right corners of the pixel buffer.
pub fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

use image::{ImageBuffer, Luma};
/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
pub fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    // Make sure the folder is created
    let path = std::path::Path::new(filename);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create directory: {}", e),
            )
        })?;
    }
    // Create an image buffer from the pixel data
    if let Some(img) =
        ImageBuffer::<Luma<u8>, _>::from_raw(bounds.0 as u32, bounds.1 as u32, pixels.to_vec())
    {
        // Save the image, converting any errors to std::io::Error
        img.save(filename)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    }

    Ok(())
}

pub fn scale(
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    factor: f64,
) -> (Complex<f64>, Complex<f64>) {
    let center = (upper_left + lower_right) / 2.0;
    let width = (lower_right.re - upper_left.re) * factor;
    let height = (upper_left.im - lower_right.im) * factor;
    (
        Complex {
            re: center.re - width / 2.0,
            im: center.im + height / 2.0,
        },
        Complex {
            re: center.re + width / 2.0,
            im: center.im - height / 2.0,
        },
    )
}
#[test]
fn test_scale() {
    assert_eq!(
        scale(
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 },
            2.0
        ),
        (Complex { re: -2.0, im: 2.0 }, Complex { re: 2.0, im: -2.0 })
    );
}

use gif::{Encoder, Frame, Repeat};
/// Create a GIF from a series of PNG images.
///
/// # Arguments
/// * `frames` - A vector of file paths to the PNG images to include in the GIF
/// * `output` - The file path for the output GIF
/// * `delay` - The delay between frames in hundredths of a second (e.g., 10 = 0.1 seconds)
///
/// # Returns
/// * `Ok(())` if the GIF was created successfully
/// * `Err(std::io::Error)` if there was an error creating or writing the GIF
///
/// # Example
/// ```
/// let frames = vec!["frame1.png".to_string(), "frame2.png".to_string()];
/// make_gif(frames, "animation.gif", 10)?;
/// ```
pub fn make_gif(frames: Vec<String>, output: &str, delay: u16) -> Result<(), std::io::Error> {
    // Check if we have any frames
    if frames.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No frames provided for GIF creation",
        ));
    }

    // Create output file and encoder
    let file = std::fs::File::create(output)?;

    // Open the first image to get dimensions
    let first_img = image::open(&frames[0])
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    let (width, height) = (first_img.width(), first_img.height());

    // Create a grayscale palette with 256 shades
    let mut palette = Vec::with_capacity(768); // 256 colors * 3 channels
    for i in 0..256 {
        palette.push(i as u8); // R
        palette.push(i as u8); // G
        palette.push(i as u8); // B
    }

    // Create the GIF encoder with our grayscale palette
    let mut encoder = Encoder::new(file, width as u16, height as u16, &palette)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Configure the GIF settings
    encoder
        .set_repeat(Repeat::Infinite)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Process each frame
    for (i, frame_path) in frames.iter().enumerate() {
        // Load the image
        let img = image::open(frame_path).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to open frame {}: {}", frame_path, e),
            )
        })?;

        // Convert to luma8 (grayscale)
        let img = img.to_luma8();

        // Check dimensions match the first frame
        if i > 0 && (img.width() != width || img.height() != height) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Frame dimensions mismatch: {} has different size than the first frame",
                    frame_path
                ),
            ));
        }

        // Get the raw pixel data - this contains grayscale values
        let buffer = img.into_raw();

        // Create a GIF frame
        let mut frame = Frame::default();
        frame.width = width as u16;
        frame.height = height as u16;
        frame.delay = delay;

        // The buffer already contains the palette indices (grayscale values 0-255)
        frame.buffer = std::borrow::Cow::Owned(buffer);

        // Write the frame to the GIF
        encoder
            .write_frame(&frame)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    }

    Ok(())
}
