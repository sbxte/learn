use color::{Color, write_color};

mod color;
mod vec3;

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width as f64 - 1.),
                j as f64 / (image_height as f64 - 1.),
                0.,
            );
            write_color(&mut std::io::stdout(), pixel_color);
        }
    }
    eprintln!("Done!");
}
