use crate::complex::Complex;

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    NoSolutions,
    FalseDiscriminant,
    WtfHappened,
}

pub fn solve_linear(a: Complex, b: Complex) -> Result<Complex, ErrorType> {
    if a.is_zero() {
        return Err(ErrorType::NoSolutions);
    }
    Ok(-b / a)
}

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Result<(Complex, Complex), ErrorType> {
    if a == 0.0 {
        return match solve_linear(Complex::new(b, 0.0), Complex::new(c, 0.0)) {
            Err(e) => Err(e),
            Ok(s) => Ok((s, s)),
        };
    }

    let k = -b / (a * 2.0);
    let d = k * k - c / a;
    if d < 0.0 {
        let f = (-d).sqrt();
        let i = f / (a * 2.0);
        Ok((Complex::new(k, i), Complex::new(k, -i)))
    } else {
        let f = d.sqrt();
        Ok((Complex::new(k - f, 0.0), Complex::new(k + f, 0.0)))
    }
}

// pub fn solve_cubic(a: f64, b: f64, c: f64, d: f64) -> Result<(Complex, Complex, Complex), ErrorType> {
//     if a == 0.0 {
//         return match solve_quadratic(b, c, d) {
//             Ok((r1, r2)) => Ok((r1, r2, r1)),
//             Err(e) => Err(e),
//         }
//     }

//     let k = -b/(3.0*a);
//     let p = (3.0*a*c - b*b)/(3.0*a*a);
//     let q = (2.0*b*b*b - 9.0*a*b*c + 27.0*a*a*d)/(27.0*a*a*a);

//     let p0 = -q*0.5;
//     let p1 = q*q*0.25 + p*p*p/27.0;

//     if -1e-10 < p1 && p1 < 1e-10 {
//         if -1e-10 < p0 && p0 < 1e-10 {
//             return Ok((k, k, k));
//         }
//         let m = p0.cbrt();
//         let n = -2.0*m;
//         if m > n {
//             return Ok((k - m, k - n, k - n));
//         }
//         return Ok((k - n, k - n, k - m));
//     } else if p1 < 0.0 {
//         let t = f64::atan2((-p1).sqrt(), p0)/3.0;
//         let m = (-p/3.0).sqrt();
//         let co = t.cos()*m;
//         let si = t.sin()*m;
//         return Ok((k - si*ROOT_3 - co, k + si*ROOT_3 - co, k + 2.0*co));
//     } else {
//         let m = (p0 + p1.sqrt()).cbrt();
//         let z = k + m - p/(3.0*m);
//         return Ok((z, z, z));
//     }
// }

pub fn run_examples() {
    // Linear
    println!(
        "Solution of 4x + 1 = 0 is {}!",
        solve_linear(Complex::new(4.0, 0.0), Complex::new(1.0, 0.0)).unwrap()
    );
    println!(
        "Solution of 5x - 10 = 0 is {}!",
        solve_linear(Complex::new(5.0, 0.0), Complex::new(-10.0, 0.0)).unwrap()
    );
    match solve_linear(Complex::new(0.0, 0.0), Complex::new(3.0, 0.0)) {
        Ok(s) => println!("Solution of 0x - 3 = 0 is {}!", s),
        Err(e) => println!("Solution of 0x - 3 = 0 is {:?}!", e),
    }

    // Quadratic
    println!(
        "Solution of x^2 - 2x -3 = 0 is {:?}!",
        solve_quadratic(1.0, -2.0, -3.0).unwrap()
    );
    println!(
        "Solution of x^2 - 4x + 3 = 0 is {:?}!",
        solve_quadratic(1.0, -4.0, 3.0).unwrap()
    );
}
