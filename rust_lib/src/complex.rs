use std::ops::{Add, Mul};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    pub fn new(re: f32, im: f32) -> Complex {
        Complex { re, im }
    }

    pub const ZERO: Complex = Complex { re: 0.0, im: 0.0 };
}

impl<'a, 'b> Add<&'b Complex> for &'a Complex {
    type Output = Complex;

    fn add(self, rhs: &'b Complex) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl<'a, 'b> Mul<&'b Complex> for &'a Complex {
    type Output = Complex;

    fn mul(self, rhs: &'b Complex) -> Self::Output {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_arithmetic() {
        let c1 = Complex::new(2.0, 5.0);
        let c2 = Complex::new(-1.0, 10.0);

        assert_eq!(Complex { re: 1.0, im: 15.0 }, c1 + c2);
        assert_eq!(
            Complex {
                re: -52.0,
                im: 15.0
            },
            c1 * c2
        );

        let c1 = Complex::new(0.0, 5.0);
        let c2 = Complex::new(3.0, 4.0);

        assert_eq!(Complex { re: 3.0, im: 9.0 }, c1 + c2);
        assert_eq!(
            Complex {
                re: -20.0,
                im: 15.0
            },
            c1 * c2
        );
    }
}
