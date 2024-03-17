use image::GenericImageView;
use std::{fmt, fs::File, io, path::PathBuf};

mod cli;

const SYMBOLS: &[char] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

fn main() -> Result<(), &'static str> {
    let matches = cli::new().get_matches();

    let input_file: &PathBuf = matches.get_one("input").unwrap();
    let x_stretch: f64 = *matches.get_one("x-stretch").unwrap();
    let size: u32 = *matches.get_one("size").unwrap();

    let img = image::open(input_file).unwrap();
    let (w, h) = img.dimensions();

    let scale = size as f64 / u32::max(h, w) as f64;
    let w = (w as f64 * scale * x_stretch) as u32;
    let h = (h as f64 * scale) as u32;

    let resized = img.resize_exact(w, h, image::imageops::FilterType::Nearest);

    let data = resized
        .pixels()
        .map(|(_, _, color)| {
            let sum: u16 = color.0.iter().take(3).map(|&v| v as u16).sum::<u16>();
            let normalized = sum as f64 / (3.0 * 255.0);
            let idx = (normalized * SYMBOLS.len() as f64).round() as usize;

            SYMBOLS[idx]
        })
        .collect::<Vec<_>>()
        .chunks(w as usize)
        .fold(
            String::with_capacity((size as usize).pow(2)),
            |mut s, line| {
                fmt::Write::write_str(&mut s, &line.iter().chain(&['\n']).collect::<String>())
                    .unwrap();
                s
            },
        );

    match matches.get_one::<PathBuf>("output") {
        None => print!("{data}"),
        Some(path) => {
            let mut file = File::create(path).unwrap_or_else(|_| {
                panic!(
                    "Failed to open output file: '{}'",
                    path.to_str().unwrap_or_default()
                )
            });
            io::Write::write(&mut file, data.as_bytes()).unwrap();
        }
    }

    Ok(())
}
