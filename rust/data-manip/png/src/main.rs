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

    dbg!(&png.pixels.len(), &png.bit_depth, &png.color_type);

    let mut sum: u128 = 0;
    for i in &png.pixels {
        sum += i.a as u128;
    }
    dbg!(sum as f32 / png.pixels.len() as f32);
}
