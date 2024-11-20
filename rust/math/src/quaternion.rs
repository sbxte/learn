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

// Norm
// Requires sqrt functions and those are not trait functions
// so their explicit methods must be used
macro_rules! impl_norm {
    ( $method:ident, $($t:ident)+) => {
        $(
            impl Quaternion<$t> {
                /// Returns the norm
                /// I would personally also call this magnitude
                #[inline]
                pub fn norm(self) -> $t {
                    $t::$method(self.a * self.a + self.b * self.b + self.c * self.c + self.d * self.d)
                }

                /// Returns the normalized
                /// Norm of 1
                #[inline]
                pub fn normalize(self) -> Self {
                    self / self.norm()
                }
            }
        )+
    };
}

impl_norm!(sqrt, f32 f64);
// Integer quaternions are so cursed I advice never using it
impl_norm!(isqrt, i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 isize usize);

// Norm2 and reciprocals do not require sqrt functions
impl<T> Quaternion<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    /// Returns the square of the norm
    /// Also see `norm`
    #[inline]
    pub fn norm2(self) -> T {
        self.a * self.a + self.b * self.b + self.c * self.c + self.d * self.d
    }
}

impl<T> Quaternion<T>
where
    T: Mul<Output = T> + Add<Output = T> + Neg<Output = T> + Div<Output = T> + Copy,
{
    /// Returns the reciprocal
    #[inline]
    pub fn recip(self) -> Self {
        self.conjugate() / self.norm2()
    }
}

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

// All because negative trait bounds are not allowed... bruh
//
// impl<T, F> From<Quaternion<F>> for Quaternion<T>
// where
//     T: From<F> + !From<T>,
// {
//     fn from(value: Quaternion<F>) -> Self {
//         Self::new(
//             From::from(value.a),
//             From::from(value.b),
//             From::from(value.c),
//             From::from(value.d),
//         )
//     }
// }

macro_rules! impl_from {
    ($t:ty, $($f:ty)+) => {
        $(
            impl From<Quaternion<$f>> for Quaternion<$t> {
                fn from(value: Quaternion<$f>) -> Quaternion<$t> {
                    Self::new(
                        From::from(value.a),
                        From::from(value.b),
                        From::from(value.c),
                        From::from(value.d)
                    )
                }
            }
        )+
    }
}

impl_from!(f64, f32 u32 u16 u8);
impl_from!(f32, u16 u8);
impl_from!(u64, u32 u16 u8);
impl_from!(u32, u16 u8);
impl_from!(u16, u8);
impl_from!(i64, i32 i16 i8);
impl_from!(i32, i16 i8);
impl_from!(i16, i8);

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

    #[test]
    fn interior_numeric_conversion() {
        assert_eq!(
            <Quaternion<f64> as From<_>>::from(Quaternion::<f32>::new(0.0, 0.0, 0.0, 0.0)),
            Quaternion::<f64>::new(0.0, 0.0, 0.0, 0.0)
        );
        assert_eq!(
            <Quaternion<i64> as From<_>>::from(Quaternion::<i32>::new(0, 0, 0, 0)),
            Quaternion::<i64>::new(0, 0, 0, 0)
        );
    }
}
