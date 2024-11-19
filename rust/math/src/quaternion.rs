use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Quaternion<N> {
    a: N,
    b: N,
    c: N,
    d: N,
}

impl<T> Quaternion<T> {
    pub fn new(a: T, b: T, c: T, d: T) -> Self {
        Self { a, b, c, d }
    }
}

// Conjugates require negation
impl<T> Quaternion<T>
where
    T: Neg<Output = T>,
{
    #[inline]
    pub fn conjugate(self) -> Self {
        Self::new(self.a, -self.b, -self.c, -self.d)
    }
}

// Norms and Reciprocals
// Requires sqrt functions and those are not trait functions
// so their explicit methods must be used
macro_rules! impl_norm_recip {
    ($t:ident, $method:ident) => {
        impl Quaternion<$t> {
            /// I would personally also call this magnitude
            #[inline]
            pub fn norm(self) -> $t {
                $t::$method(self.a * self.a + self.b * self.b + self.c * self.c + self.d * self.d)
            }

            /// Returns the square of the norm
            /// Also see `norm`
            #[inline]
            pub fn norm2(self) -> $t {
                self.a * self.a + self.b * self.b + self.c * self.c + self.d * self.d
            }

            /// Returns the reciprocal
            #[inline]
            pub fn recip(self) -> Self {
                self.conjugate() / self.norm2()
            }
        }
    };
}

impl_norm_recip!(f32, sqrt);
impl_norm_recip!(f64, sqrt);
impl_norm_recip!(i16, isqrt);
impl_norm_recip!(i32, isqrt);
impl_norm_recip!(i64, isqrt);
impl_norm_recip!(i128, isqrt);

// Negation, Addition, Subtraction, Multiplication, Division

impl<T> Neg for Quaternion<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.a, -self.b, -self.c, -self.d)
    }
}

impl<T> Add for Quaternion<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.a + rhs.a,
            self.b + rhs.b,
            self.c + rhs.c,
            self.d + rhs.d,
        )
    }
}

impl<T> AddAssign for Quaternion<T>
where
    T: AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.a += rhs.a;
        self.b += rhs.b;
        self.c += rhs.c;
        self.d += rhs.d;
    }
}

impl<T> Sub for Quaternion<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.a - rhs.a,
            self.b - rhs.b,
            self.c - rhs.c,
            self.d - rhs.d,
        )
    }
}

impl<T> SubAssign for Quaternion<T>
where
    T: SubAssign,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.a -= rhs.a;
        self.b -= rhs.b;
        self.c -= rhs.c;
        self.d -= rhs.d;
    }
}

impl<T> Mul<T> for Quaternion<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.a * rhs, self.b * rhs, self.c * rhs, self.d * rhs)
    }
}

impl<T> MulAssign<T> for Quaternion<T>
where
    T: MulAssign + Copy,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.a *= rhs;
        self.b *= rhs;
        self.c *= rhs;
        self.d *= rhs;
    }
}

impl<T> Mul<Quaternion<T>> for Quaternion<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.a * rhs.a - self.b * rhs.b - self.c * rhs.c - self.d * rhs.d,
            self.a * rhs.b + self.b * rhs.a + self.c * rhs.d - self.d * rhs.c,
            self.a * rhs.c + self.c * rhs.a + self.d * rhs.b - self.b * rhs.d,
            self.a * rhs.d + self.d * rhs.a + self.b * rhs.c - self.c * rhs.b,
        )
    }
}

impl<T> MulAssign<Quaternion<T>> for Quaternion<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.a = self.a * rhs.a - self.b * rhs.b - self.c * rhs.c - self.d * rhs.d;
        self.b = self.a * rhs.b + self.b * rhs.a + self.c * rhs.d - self.d * rhs.c;
        self.c = self.a * rhs.c + self.c * rhs.a + self.d * rhs.b - self.b * rhs.d;
        self.d = self.a * rhs.d + self.d * rhs.a + self.b * rhs.c - self.c * rhs.b;
    }
}

impl<T> Div<T> for Quaternion<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.a / rhs, self.b / rhs, self.c / rhs, self.d / rhs)
    }
}

impl<T> DivAssign<T> for Quaternion<T>
where
    T: DivAssign + Copy,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.a /= rhs;
        self.b /= rhs;
        self.c /= rhs;
        self.d /= rhs;
    }
}

macro_rules! impl_default {
    ($($t:ty)*) => {
        $(
            impl Default for Quaternion<$t> {
                fn default() -> Self {
                    Self::new(
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                    )
                }
            }
        )*
    };
}

impl_default!(i8 i16 i32 i64 i128 f32 f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_types_defaults() {
        macro_rules! test_default {
            ($($t:ty)*) => ($(
                let x: Quaternion<$t> = Default::default();
                assert_eq!(
                    x,
                    Quaternion::new(
                        <$t>::default(),
                        <$t>::default(),
                        <$t>::default(),
                        <$t>::default()
                    )
                );
            )*)
        }

        test_default!(i8 i16 i32 i64 f32 f64);
    }
}
