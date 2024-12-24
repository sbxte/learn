fn main() {
    let height = 256;
    let width = 256;

    println!("P3");
    println!("{} {}", height, width);
    println!("255");

    for j in 0..height {
        for i in 0..width {
            let r = (i as f64) / (height as f64);
            let g = (j as f64) / (width as f64);
            let b = 0.0;

            const MAX: f64 = 255.999;
            let ir = (MAX * r) as u64;
            let ig = (MAX * g) as u64;
            let ib = (MAX * b) as u64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
