use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(f: &mut dyn std::io::Write, pixel_color: Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rb = (255.999 * r) as u64;
    let gb = (255.999 * g) as u64;
    let bb = (255.999 * b) as u64;

    let _ = writeln!(f, "{} {} {}", rb, gb, bb);
}
