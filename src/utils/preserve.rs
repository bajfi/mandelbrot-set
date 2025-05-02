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
