use std::fmt::Display;

use crate::complex::Complex;

#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct Polynomial {
    pub degree: usize,
    pub coefficients: Vec<Complex>,
}

impl Polynomial {
    pub fn zero() -> Polynomial {
        Polynomial {
            degree: 0,
            coefficients: vec![Complex::ZERO],
        }
    }

    /// Create a new complex polynomial, with its degree based on the number of coefficients.
    ///
    /// The coefficients are stored from lowest degree (0) to highest degree.
    pub fn new(coefficients: Vec<Complex>) -> Polynomial {
        if coefficients.len() == 0 {
            Polynomial::zero()
        } else {
            Polynomial {
                degree: coefficients.len() - 1,
                coefficients,
            }
        }
    }

    /// Trims a polynomial if needed, removing the highest degree terms with a 0 coefficient
    /// and adjusting the degree of the polynomial if needed.
    pub fn trim(self) -> Polynomial {
        if self.degree == 0 {
            return self;
        }

        let mut degree = self.degree;
        let mut coeff_clone = self.coefficients.clone();
        while let Some(last) = coeff_clone.pop() {
            if last != Complex::ZERO || degree == 0 {
                break;
            }

            degree -= 1;
        }

        Polynomial::new(self.coefficients.into_iter().take(degree + 1).collect())
    }

    /// Add two polynomials, summing both their coefficients one by one.
    ///
    /// The resulting polynomial is trimmed to the degree of its highest non-zero coefficient.
    pub fn add(a: &Polynomial, b: &Polynomial) -> Polynomial {
        let mut coefficients = vec![];

        let mut iter1 = a.coefficients.iter();
        let mut iter2 = b.coefficients.iter();

        // Make sure iter1 is the longest one
        if b.degree > a.degree {
            std::mem::swap(&mut iter1, &mut iter2);
        }

        // Pad the shortest one with zeroes
        let zero_iter = std::iter::repeat(&Complex::ZERO);
        let iter2 = zero_iter
            .take(b.degree.abs_diff(a.degree))
            .chain(iter2.rev());

        for (c1, c2) in iter1.rev().zip(iter2) {
            coefficients.push(c1 + c2);
        }

        let res = Polynomial::new(coefficients.into_iter().rev().collect());

        res.trim()
    }

    /// Multiply two polynomials, done using the "schoolbook" algorithm.
    ///
    /// The resulting polynomial is trimmed to the degree of its highest non-zero coefficient.
    pub fn mul(a: &Polynomial, b: &Polynomial) -> Polynomial {
        let len = match (a.coefficients.len() + b.coefficients.len()).checked_sub(1) {
            Some(len) => len,
            None => return Polynomial::zero(),
        };

        let mut coefficients = vec![Complex::ZERO; len];

        // "Schoolbook" algorithm
        for (i, c1) in a.coefficients.iter().enumerate() {
            for (j, c2) in b.coefficients.iter().enumerate() {
                coefficients[i + j] = coefficients[i + j] + c1 * c2;
            }
        }

        Polynomial::new(coefficients).trim()
    }

    /// Negates all the coefficients of a polynomial.
    pub fn neg(mut self: Polynomial) -> Polynomial {
        self.coefficients.iter_mut().for_each(|c| {
            *c = c.neg();
        });

        self
    }

    /// Reduce the provided polynomial to the provided ring degree.
    ///
    /// For example, reducing `-3x4 + 6x3 + 3x2 - 6x` with a ring of degree 4 results
    /// in the polynomial `6x3 + 3x2 - 6x -3`. We obtain this result by doing the
    /// euclidian division of `self` by `x^ring - 1`, and returning the remainder.
    pub fn reduce_to(&self, ring: usize) -> Polynomial {
        if self.degree < ring {
            return self.to_owned();
        }

        let mut denominator = vec![Complex::ZERO; ring + 1];
        denominator[0] = Complex::new(-1.0, 0.0);
        denominator[ring] = Complex::new(1.0, 0.0);

        let (_, remainder) = Self::euclidean_division(self, &Polynomial::new(denominator));

        remainder
    }

    /// Add two polynomials, after having reduced them to the provided ring degree.
    ///
    /// The result is a polynomial trimmed to the degree of its highest non-zero coefficient.
    pub fn add_in_ring(a: &Polynomial, b: &Polynomial, ring: usize) -> Polynomial {
        let a = a.reduce_to(ring);
        let b = b.reduce_to(ring);

        Self::add(&a, &b)
    }

    /// Multiply two polynomials, after having reduced them to the provided ring degree.
    ///
    /// The result is reduced to the provided ring degree. After this, the result is a
    /// polynomial trimmed to the degree of its highest non-zero coefficient.
    pub fn mul_in_ring(a: &Polynomial, b: &Polynomial, ring: usize) -> Polynomial {
        let a = a.reduce_to(ring);
        let b = b.reduce_to(ring);

        let res = Self::mul(&a, &b);

        res.reduce_to(ring)
    }

    /// Returns the (quotient, remainder) of the euclidean division of `numerator` by `denominator`.
    pub fn euclidean_division(
        numerator: &Polynomial,
        denominator: &Polynomial,
    ) -> (Polynomial, Polynomial) {
        // "Long division" methods for polynomials.
        // Take the numerator and the denominator.
        // While the numerator's degree is higher or equal to the denominator's:
        // 1. Divide the highest term by the denominator's highest term
        // 2. Push the result to the quotient
        // 3. Substract the result multiplied to the denominator from the numerator
        // The remainder is what's left from the numerator at the end of the iterations.

        if numerator.degree < denominator.degree {
            return (numerator.to_owned(), Polynomial::zero());
        }

        let quotient_len = numerator.degree - denominator.degree + 1;

        let mut quotient: Vec<Complex> = vec![Complex::ZERO; quotient_len];
        let mut remainder = numerator.to_owned();

        let divider_highest_term = denominator.coefficients.last().unwrap();
        let divider_highest_term_degree = denominator.degree;

        while remainder.degree >= denominator.degree {
            let rem_highest_term = remainder.coefficients.last().unwrap();
            let rem_highest_term_degree = remainder.degree;

            let res = divider_highest_term * rem_highest_term;
            let res_degree = rem_highest_term_degree - divider_highest_term_degree;

            // Update the quotient's current term
            let idx = res_degree;
            quotient[idx] = quotient[idx] + res;

            // Multiply `res` by the denominator
            let mut res_poly = Polynomial::new(vec![Complex::ZERO; res_degree + 1]);
            res_poly.coefficients[res_degree] = res;

            let mul_res = Polynomial::mul(&res_poly, denominator);
            let neg_res = mul_res.neg();

            remainder = Polynomial::add(&remainder, &neg_res);
        }

        let quotient = Polynomial::new(quotient);

        (quotient, remainder)
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, coefficient) in self.coefficients.iter().rev().enumerate() {
            if coefficient == &Complex::ZERO {
                continue;
            }

            if i != 0 {
                write!(f, " + ")?;
            }

            match self.degree - i {
                0 => write!(f, "{}", coefficient),
                1 => write!(f, "({})X", coefficient),
                i => write!(f, "({})X{}", coefficient, i),
            }?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomials_construction() {
        let p = Polynomial::new(vec![Complex::new(0.0, 1.0)]);
        assert_eq!(0, p.degree);

        let p = Polynomial::new(vec![Complex::new(0.0, 1.0), Complex::new(3.0, -1.0)]);
        assert_eq!(1, p.degree);
    }

    #[test]
    fn test_polynomials_add() {
        let p1 = Polynomial::new(vec![Complex::new(3.0, 0.0), Complex::new(1.0, 0.0)]);
        let p2 = Polynomial::new(vec![Complex::new(1.0, 0.0), Complex::new(-1.0, 0.0)]);

        assert_eq!(
            Polynomial::new(vec![Complex::new(4.0, 0.0)]),
            Polynomial::add(&p1, &p2),
        );

        assert_eq!(
            Polynomial::new(vec![Complex::new(4.0, 0.0)]),
            Polynomial::add_in_ring(&p1, &p2, 1)
        );

        let p1 = Polynomial::new(vec![Complex::new(3.0, -2.0), Complex::new(1.0, -3.0)]);
        let p2 = Polynomial::new(vec![Complex::new(1.0, 4.0), Complex::new(-1.0, 3.0)]);

        assert_eq!(
            Polynomial::new(vec![Complex::new(4.0, 2.0)]),
            Polynomial::add(&p1, &p2),
        );

        assert_eq!(
            Polynomial::new(vec![Complex::new(4.0, 2.0)]),
            Polynomial::add_in_ring(&p1, &p2, 2),
        );
    }

    #[test]
    fn test_euclidean_division() {
        // p = x5 - x2 + 1
        let p = Polynomial::new(vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(-1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
        ]);

        // p / (x2 - 1)
        let res = Polynomial::euclidean_division(
            &p,
            &Polynomial::new(vec![
                Complex::new(-1.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(1.0, 0.0),
            ]),
        );

        // quotient: x3 + x - 1
        // remainder: x
        assert_eq!(
            (
                Polynomial::new(vec![
                    Complex::new(-1.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                ]),
                Polynomial::new(vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),])
            ),
            res,
        );

        // p / (x3 - 1)
        let res = Polynomial::euclidean_division(
            &p,
            &Polynomial::new(vec![
                Complex::new(-1.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(1.0, 0.0),
            ]),
        );

        // quotient: x3 - 1
        // remainder: 1
        assert_eq!(
            (
                Polynomial::new(vec![
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                ]),
                Polynomial::new(vec![Complex::new(1.0, 0.0)]),
            ),
            res,
        );
    }

    #[test]
    fn test_polynomials_reduce() {
        let p = Polynomial::new(vec![
            Complex::new(0.0, 0.0),
            Complex::new(-1.0, 0.0),
            Complex::new(-16.0, 0.0),
            Complex::new(18.0, 0.0),
            Complex::new(16.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0),
        ]);

        assert_eq!(
            Polynomial::new(vec![
                Complex::new(16.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(-15.0, 0.0),
                Complex::new(18.0, 0.0),
            ]),
            p.reduce_to(4),
        );

        let p = Polynomial::new(vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(-1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(1.0, 0.0),
        ]);

        assert_eq!(
            Polynomial::new(vec![
                Complex::new(3.0, 0.0),
                Complex::new(1.0, 0.0),
                Complex::new(-1.0, 0.0),
            ]),
            p.reduce_to(4),
        );

        assert_eq!(
            Polynomial::new(vec![Complex::new(1.0, 0.0), Complex::new(2.0, 0.0),]),
            p.reduce_to(3),
        );

        assert_eq!(
            Polynomial::new(vec![Complex::new(2.0, 0.0), Complex::new(1.0, 0.0),]),
            p.reduce_to(2),
        );
    }

    #[test]
    fn test_polynomials_mul() {
        let p1 = Polynomial::new(vec![Complex::new(3.0, 0.0), Complex::new(1.0, 0.0)]);
        let p2 = Polynomial::new(vec![Complex::new(1.0, 0.0), Complex::new(-1.0, 0.0)]);

        assert_eq!(
            Polynomial::new(vec![Complex::new(2.0, 0.0), Complex::new(-2.0, 0.0),]),
            Polynomial::mul_in_ring(&p1, &p2, 2),
        );

        let p1 = Polynomial::new(vec![
            Complex::new(-3.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(3.0, 0.0),
        ]);
        let p2 = Polynomial::new(vec![
            Complex::new(0.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(-1.0, 0.0),
        ]);

        let res_ring4 = Polynomial::new(vec![
            Complex::new(-3.0, 0.0),
            Complex::new(-6.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(6.0, 0.0),
        ]);
        assert_eq!(res_ring4, Polynomial::mul_in_ring(&p1, &p2, 4));

        let res_ring3 = Polynomial::new(vec![
            Complex::new(6.0, 0.0),
            Complex::new(-9.0, 0.0),
            Complex::new(3.0, 0.0),
        ]);
        assert_eq!(res_ring3, Polynomial::mul_in_ring(&p1, &p2, 3));

        let res_ring2 = Polynomial::zero();
        assert_eq!(res_ring2, Polynomial::mul_in_ring(&p1, &p2, 2));

        let res_ring1 = Polynomial::zero();
        assert_eq!(res_ring1, Polynomial::mul_in_ring(&p1, &p2, 1));
    }
}
