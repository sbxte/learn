use rand::prelude::*;

fn main() {
    let data = [
        655.2873f32,
        657.1582f32,
        657.2038f32,
        656.8522f32,
        657.0474f32,
        656.532f32,
        656.0639f32,
        658.4237f32,
        654.4136f32,
        656.5536f32,
    ];

    let deviation = 0.1f32;

    for v in data {
        let v = v + random::<f32>() * deviation * 2.0 - deviation;
        println!("{v}");
    }
}
