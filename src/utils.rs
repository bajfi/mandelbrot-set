pub mod preserve;
pub mod transform;
use num::Complex;
use std::str::FromStr;

/// Enum representing different types of fractals
#[derive(Debug, Clone, Copy)]
pub enum FractalType {
    /// Standard Mandelbrot set: z = z^n + c
    Mandelbrot,
    /// Julia set: z = z^n + k where k is a constant
    Julia,
    /// Burning Ship fractal: z = (|Re(z)| + i|Im(z)|)^2 + c
    BurningShip,
    /// Tricorn/Mandelbar: z = conj(z)^n + c
    Tricorn,
    /// Nova fractal: z = z - (z^n - 1)/(n*z^(n-1)) + c
    Nova,
    /// Sin fractal: z = sin(z) + c
    Sin,
    /// Cos fractal: z = cos(z) + c
    Cos,
}

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

/// Try to determine if a point is in the fractal set, using at most `limit`
/// iterations to decide.
///
/// If the point is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for the calculation to exceed the escape radius.
/// If the point seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that it's not a member),
/// return `None`.
pub fn escape_time(
    c: Complex<f64>,
    limit: usize,
    power: i32,
    escape_radius: f64,
    fractal_type: FractalType,
    julia_constant: Option<Complex<f64>>,
) -> Option<usize> {
    assert!(limit > 0);
    assert!(escape_radius > 0.0);
    
    // Initial z value depends on the fractal type
    let mut z = match fractal_type {
        FractalType::Julia => c,                     // For Julia sets, z starts at the point coordinate
        _ => Complex { re: 0.0, im: 0.0 },           // For others, start at origin
    };
    
    for i in 0..limit {
        if z.norm_sqr() > escape_radius.powi(2) {
            return Some(i);
        }
        
        // Apply the appropriate formula based on the fractal type
        z = match fractal_type {
            FractalType::Mandelbrot => z.powi(power) + c,
            
            FractalType::Julia => {
                // Julia sets use a constant value k instead of c for the iteration
                let k = julia_constant.unwrap_or(Complex { re: -0.8, im: 0.156 });
                z.powi(power) + k
            },
            
            FractalType::BurningShip => {
                // Take absolute values of real and imaginary parts before squaring
                let re_abs = z.re.abs();
                let im_abs = z.im.abs();
                Complex { re: re_abs, im: im_abs }.powi(2) + c
            },
            
            FractalType::Tricorn => {
                // Take the complex conjugate before applying the power
                let z_conj = Complex { re: z.re, im: -z.im };
                z_conj.powi(power) + c
            },
            
            FractalType::Nova => {
                // Nova fractal: z = z - (z^n - 1)/(n*z^(n-1)) + c
                let p = power as f64;
                let numerator = z.powi(power) - Complex::new(1.0, 0.0);
                let denominator = p * z.powi(power - 1);
                z - (numerator / denominator) + c
            },
            
            FractalType::Sin => Complex::new(z.sin().re, z.sin().im) + c,
            
            FractalType::Cos => Complex::new(z.cos().re, z.cos().im) + c,
        };
    }
    
    None
}

/// Render a rectangle of the fractal set into a buffer of pixels.
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
    power: i32,
    escape_radius: f64,
    fractal_type: FractalType,
    julia_constant: Option<Complex<f64>>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = transform::pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] =
                match escape_time(point, u8::MAX as usize, power, escape_radius, fractal_type, julia_constant) {
                    None => 0,
                    Some(count) => u8::MAX - count as u8,
                };
        }
    }
}
