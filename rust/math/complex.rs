use std::ops;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Complex {
    real: f64,
    imaginary: f64,
}
impl Complex {
    pub fn new(real: f64, imaginary: f64) -> Complex {
        Complex { real, imaginary }
    }

    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imaginary * self.imaginary).sqrt()
    }

    pub fn is_zero(&self) -> bool {
        self.real == 0.0 && self.imaginary == 0.0
    }
}
impl ::std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if -1e-10 < self.imaginary && self.imaginary < 1e-10 {
            write!(f, "{}", self.real)
        } else if -1e-10 < self.real && self.real < 1e-10 {
            write!(f, "{}i", self.imaginary)
        } else {
            write!(f, "({} + {}i)", self.real, self.imaginary)
        }
    }
}
impl ops::Add<Self> for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.to_owned();
        result.real += rhs.real;
        result.imaginary += rhs.imaginary;

        result
    }
}
impl ops::Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut result = self.to_owned();
        result.real = -result.real;
        result.imaginary = -result.imaginary;

        result
    }
}
impl ops::Sub<Self> for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.to_owned();
        result.real = self.real - rhs.real;
        result.imaginary = self.imaginary - rhs.imaginary;
        result
    }
}
impl ops::Mul<Self> for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = self.to_owned();
        result.real = self.real * rhs.real - self.imaginary * rhs.imaginary;
        result.imaginary = self.real * rhs.imaginary + self.imaginary * rhs.real;
        result
    }
}
impl ops::Mul<f64> for Complex {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut result = self.to_owned();
        result.real *= rhs;
        result.imaginary *= rhs;
        result
    }
}
impl ops::Div<Complex> for Complex {
    type Output = Self;

    fn div(self, rhs: Complex) -> Self::Output {
        let mut result = self.to_owned();
        let d = rhs.real * rhs.real + rhs.imaginary * rhs.imaginary;

        let real = self.real * rhs.real + self.imaginary * rhs.imaginary;
        let imaginary = - self.real * rhs.imaginary + rhs.real * self.imaginary;

        result.real = real / d;
        result.imaginary = imaginary / d;
        result
    }
}

