pub mod complex;
pub mod fib;
pub mod matrix;
pub mod polynomial_solver;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polynomial_solver_linear_sol() {
        let a = complex::Complex::new(1f64, 0f64);
        let b = complex::Complex::new(2f64, 0f64);
        assert_eq!(
            polynomial_solver::solve_linear(a, b).unwrap(),
            complex::Complex::new(-2f64, 0f64)
        );
    }

    #[test]
    fn polynomial_solver_linear_nosol() {
        use crate::polynomial_solver::ErrorType;
        let a = complex::Complex::new(0f64, 0f64);
        let b = complex::Complex::new(1f64, 0f64);
        assert_eq!(
            polynomial_solver::solve_linear(a, b),
            Err(ErrorType::NoSolutions)
        );
    }
}
