pub type Float = f32;

pub const PI2: Float = std::f32::consts::PI * 2.;

/// Takes in the time-domain samples
/// and returns frequency-domain results
pub fn my_dft(samples: &[Float]) -> Vec<(Float, Float)> {
    let mut result = Vec::with_capacity(samples.len());

    for f in 0..(samples.len() >> 1) {
        // Calculate both sin and cos
        let t_samples = (0..samples.len())
            .map(|x| Float::sin_cos(PI2 * (f as Float) * (x as Float) / samples.len() as Float));

        // Normalize
        let (mag_s, mag_c) = t_samples.clone().fold((0., 0.), |(acc_s, acc_c), (s, c)| {
            (s.mul_add(s, acc_s), c.mul_add(c, acc_c))
        });
        let (mag_s, mag_c) = (
            if mag_s != 0. { mag_s } else { 1. },
            if mag_c != 0. { mag_c } else { 1. },
        );
        let t_samples = t_samples.map(|(s, c)| (s / mag_s, c / mag_c));

        // Dot product
        let (dot_s, dot_c) = t_samples
            .zip(samples.iter())
            .fold((0., 0.), |(acc_s, acc_c), ((s, c), x)| {
                (x.mul_add(s, acc_s), x.mul_add(c, acc_c))
            });

        let ampl = (dot_s * dot_s + dot_c * dot_c).sqrt();
        let phase = dot_c.atan2(dot_s);

        result.push((ampl, phase));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pure_sines() {
        let amplitudes = [1., 1., 2., 1.5, 1., 2., 0.5];
        let freqs = [50., 100., 250., 400., 450., 500., 600.];
        let phases = [0., 30., 45., 60., 90., 180., 270.].map(|x| x / 360. * PI2);
        let n = freqs.iter().fold(0., |acc, e| Float::max(acc, *e)) as usize * 2 + 2;

        let sine = |x, a, f, p| a * Float::sin(PI2 * f * (x as Float) / n as Float + p);

        let sample = |x| {
            freqs
                .iter()
                .zip(phases)
                .zip(amplitudes)
                .fold(0., |acc, ((f, p), a)| acc + sine(x, a, f, p))
        };

        let samples: Vec<_> = (0..n).map(|x| sample(x)).collect();
        let result = my_dft(&samples);

        println!("Freq, Ampl, Phase");
        for (f, (a, p)) in result.iter().enumerate() {
            if a.abs() > 0.01 {
                println!("{} {:.2} {:.2}", f, a, p / PI2 * 360.);
            }
        }
        assert!(result
            .iter()
            .enumerate()
            .map(|(f, (a, p))| (f, *a, *p))
            .filter(|(_, a, _)| a.abs() > 0.01)
            .enumerate()
            .all(|(i, (f, a, p))| (freqs[i] - f as f32).abs() < 0.1
                && (amplitudes[i] - a).abs() < 0.1
                && (phases[i] - (p / PI2 * 360.).abs() < 0.1)))
    }
}
