use std::fs::OpenOptions;
use std::io::Read;

extern crate png;

use png::Png;

fn main() {
    let mut file_path = std::env::current_dir().unwrap();
    file_path.push("interesting.png");

    let mut file = OpenOptions::new().read(true).open(file_path).unwrap();

    let mut bytes = vec![];
    file.read_to_end(&mut bytes).unwrap();

    let png = Png::new(&bytes);

    dbg!(
        &png.pixels.len(),
        &png.bit_depth,
        &png.color_type,
        &png.bg_color,
        &png.chromaticities,
        &png.phys_dim,
        &png.texts
    );
}
