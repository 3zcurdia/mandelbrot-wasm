use num::complex::Complex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn mandelbrot_pixels(
    width: usize,
    height: usize,
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) -> Vec<u8> {
    calculate_mandelbrot(width, height, max_iters, x_min, x_max, y_min, y_max)
        .iter()
        .flat_map(|value| pixel_color(*value))
        .collect()
}

fn pixel_color(value: usize) -> Vec<u8> {
    match value {
        0..=2 => vec![236, 110, 65, 255],
        3..=5 => vec![236, 110, 65, 255],
        6..=10 => vec![233, 73, 52, 255],
        11..=30 => vec![223, 60, 57, 255],
        31..=100 => vec![202, 60, 81, 255],
        101..=200 => vec![184, 61, 102, 255],
        201..=400 => vec![160, 61, 130, 255],
        401..=700 => vec![124, 62, 171, 255],
        _ => vec![111, 43, 164, 255],
    }
}

#[wasm_bindgen]
pub fn calculate_mandelbrot(
    width: usize,
    height: usize,
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) -> Vec<usize> {
    let dx = x_max - x_min;
    let dy = y_max - y_min;
    let width_f64 = width as f64;
    let height_f64 = height as f64;

    let mut vector: Vec<usize> = Vec::with_capacity(width * height);
    for img_y in 0..height {
        for img_x in 0..width {
            let x_percent = img_x as f64 / width_f64;
            let y_percent = img_y as f64 / height_f64;
            let cx = x_min + dx * x_percent;
            let cy = y_min + dy * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            vector.push(escaped_at);
        }
    }
    vector
}

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters
}
