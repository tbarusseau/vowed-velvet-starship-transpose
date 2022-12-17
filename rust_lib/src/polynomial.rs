use std::ops::{Add, Mul};

use crate::complex::Complex;

#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct Polynomial {
    pub degree: usize,
    pub coefficients: Vec<Complex>,
}

impl Polynomial {
    pub const ZERO: Polynomial = Polynomial {
        degree: 0,
        coefficients: vec![],
    };

    pub fn new(degree: usize, coefficients: Vec<Complex>) -> Polynomial {
        assert!(coefficients.len() == degree + 1);

        Polynomial {
            degree,
            coefficients,
        }
    }

    pub fn reduce_to(&self, degree: usize) -> Polynomial {
        Polynomial {
            degree,
            coefficients: self.coefficients[..degree + 1].to_vec(),
        }
    }
}

impl<'a, 'b> Add<&'b Polynomial> for &'a Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: &'b Polynomial) -> Self::Output {
        let mut coefficients = vec![];

        for (c1, c2) in self.coefficients.iter().zip(rhs.coefficients.iter()) {
            coefficients.push(c1 + c2)
        }

        Polynomial {
            degree: self.degree,
            coefficients,
        }
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl<'a, 'b> Mul<&'b Polynomial> for &'a Polynomial {
    type Output = Polynomial;

    fn mul(self, other: &'b Polynomial) -> Self::Output {
        let mut coefficients =
            vec![Complex::ZERO; self.coefficients.len() + other.coefficients.len() - 1];

        // "Schoolbook" algorithm
        for (i, c1) in self.coefficients.iter().enumerate() {
            for (j, c2) in other.coefficients.iter().enumerate() {
                coefficients[i + j] = coefficients[i + j] + c1 * c2;
            }
        }

        // ! FIXME: This result is WRONG.
        // Steps to fix it:
        // [ 1. Get remainder of self / x^n-1 (and other) ] -> Not in our case since self/other are already in R
        // 2. Compute the initial result: result = self * other = ax^n-1 + rest
        // 3. If result is in R, return this.
        // 4. If not, return the remainder of R / x^n-1

        Polynomial {
            degree: coefficients.len() - 1,
            coefficients,
        }
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomials_arithmetic() {
        let p1 = Polynomial::new(1, vec![Complex::new(3.0, 0.0), Complex::new(1.0, 0.0)]);
        let p2 = Polynomial::new(1, vec![Complex::new(1.0, 0.0), Complex::new(-1.0, 0.0)]);

        assert_eq!(
            Polynomial::new(1, vec![Complex::new(4.0, 0.0), Complex::new(0.0, 0.0)]),
            &p1 + &p2
        );

        let mul_res = &p1 * &p2;
        assert_eq!(
            Polynomial::new(
                2,
                vec![
                    Complex::new(3.0, 0.0),
                    Complex::new(-2.0, 0.0),
                    Complex::new(-1.0, 0.0)
                ],
            ),
            mul_res,
        );
        assert_eq!(
            Polynomial::new(1, vec![Complex::new(3.0, 0.0), Complex::new(-2.0, 0.0),]),
            mul_res.reduce_to(1),
        );

        let p1 = Polynomial::new(1, vec![Complex::new(3.0, -2.0), Complex::new(1.0, -3.0)]);
        let p2 = Polynomial::new(1, vec![Complex::new(1.0, 4.0), Complex::new(-1.0, 3.0)]);

        assert_eq!(
            Polynomial::new(1, vec![Complex::new(4.0, 2.0), Complex::new(0.0, 0.0)]),
            &p1 + &p2,
        );

        // ! FIXME: This is false!
        let mul_res = &p1 * &p2;
        assert_eq!(
            Polynomial::new(
                2,
                vec![
                    Complex::new(11.0, 10.0),
                    Complex::new(16.0, 12.0),
                    Complex::new(8.0, 6.0)
                ],
            ),
            mul_res,
        );
        assert_eq!(
            Polynomial::new(1, vec![Complex::new(11.0, 10.0), Complex::new(16.0, 12.0)]),
            mul_res.reduce_to(1)
        )
    }
}
