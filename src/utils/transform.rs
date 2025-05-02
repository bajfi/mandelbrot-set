use num::Complex;

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

/// Scale a point around a center point by a given factor.
///
/// # Arguments
/// * `original` - The original point to scale.
/// * `center` - The center point around which to scale.
/// * `factor` - The scaling factor. A value greater than 1.0 scales away from the center,
///
pub fn scale_point<T>(original: T, center: T, factor: f64) -> T
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<f64, Output = T>
        + std::ops::Div<f64, Output = T>,
{
    center + (original - center) * factor
}

#[test]
fn test_scale_point() {
    let center = Complex { re: 1.0, im: 1.0 };
    let factor = 2.0;

    let original = Complex { re: 2.0, im: 2.0 };
    let scaled = scale_point(original, center, factor);
    assert_eq!(scaled.re, 3.0);
    assert_eq!(scaled.im, 3.0);

    let original = Complex { re: 1.0, im: 1.0 };
    let scaled = scale_point(original, center, factor);
    assert_eq!(scaled.re, 1.0);
    assert_eq!(scaled.im, 1.0);

    let factor = 0.5;

    let original = Complex { re: 2.0, im: 2.0 };
    let scaled = scale_point(original, center, factor);
    assert_eq!(scaled.re, 1.5);
    assert_eq!(scaled.im, 1.5);

    let factor = -1.0;
    let original = Complex { re: 2.0, im: 2.0 };
    let scaled = scale_point(original, center, factor);
    assert_eq!(scaled.re, 0.0);
    assert_eq!(scaled.im, 0.0);
}
