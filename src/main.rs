use std::env;
use image::GenericImageView;

const SYMBOLS: &[char] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
const PIXEL_SIZE: f64 = 25.0;
const X_STRETCH: f64 = 2.0;

fn main() -> Result<(), &'static str> {
    let filename: String;
    match env::args().into_iter().nth(1) {
        None => return Err("No filename provided."),
        Some(arg) => filename = arg,
    }

    let img = image::open(filename).unwrap();
    let (w, h) = img.dimensions();

    let a = u32::max(h, w) as f64 / PIXEL_SIZE;
    let w = (w as f64 / a * X_STRETCH) as u32;
    let h = (h as f64 / a) as u32;

    let resized = img.resize_exact(w, h, image::imageops::FilterType::Nearest);

    resized.pixels()
        .map(|(_, _, color)| {
            let sum: u16 = color.0.iter().take(3).map(|&v| v as u16).sum::<u16>();
            let normalized = sum as f64 / (3.0 * 255.0);
            let idx = (normalized * SYMBOLS.len() as f64).round() as usize;

            SYMBOLS[idx]
        })
        .collect::<Vec<_>>()
        .chunks(w as usize)
        .for_each(|chunk| {
            println!("{}", chunk.iter().collect::<String>())
        });

    Ok(())
}
