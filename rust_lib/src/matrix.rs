use std::ops::{Add, Mul};

use crate::polynomial::Polynomial;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub content: Vec<Polynomial>,
}

impl Matrix {
    pub fn new(content: Vec<Polynomial>, width: usize, height: usize) -> Matrix {
        assert!(content.len() % width == 0);
        assert!(content.len() / width == height);

        Matrix {
            width,
            height,
            content,
        }
    }
}

impl<'a, 'b> Add<&'b Matrix> for &'a Matrix {
    type Output = Matrix;

    fn add(self, rhs: &'b Matrix) -> Self::Output {
        assert_eq!(self.width, rhs.width);
        assert_eq!(self.height, rhs.height);

        let mut result = vec![Polynomial::ZERO; rhs.width * self.height];

        for i in 0..self.height {
            for j in 0..rhs.width {
                let idx = j + i * rhs.width;

                // result[i][j] = self.content[i][j] + rhs.content[i][j];
                result[idx] = &self.content[idx] + &rhs.content[idx];
            }
        }

        Matrix {
            width: rhs.width,
            height: self.height,
            content: result,
        }
    }
}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl<'a, 'b> Mul<&'b Matrix> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &'b Matrix) -> Self::Output {
        assert_eq!(self.width, rhs.height);

        let mut result = vec![Polynomial::ZERO; rhs.width * self.height];

        for i in 0..self.height {
            for j in 0..rhs.width {
                for k in 0..self.width {
                    // result[i][j] = result[i][j] + self.content[i][k] * rhs.content[k][j];
                    result[j + i * rhs.width] = &result[j + i * rhs.width]
                        + &(&self.content[k + i * self.width] * &rhs.content[j + k * rhs.width]);
                }
            }
        }

        Matrix {
            width: rhs.width,
            height: self.height,
            content: result,
        }
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // // Below: sanity checks on matrices sum/multiplication.
    // #[test]
    // fn test_matrices_arithmetic() {
    //     let m1 = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
    //     let m2 = Matrix::new(vec![8.0, 12.0, 13.0, 5.0], 2, 2);

    //     assert_eq!(2, m1.width);
    //     assert_eq!(2, m1.height);

    //     assert_eq!(Matrix::new(vec![9.0, 14.0, 16.0, 9.0], 2, 2), &m1 + &m2);
    //     assert_eq!(Matrix::new(vec![34.0, 22.0, 76.0, 56.0], 2, 2), &m1 * &m2);

    //     let m1 = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
    //     let m2 = Matrix::new(vec![3.0, 4.0, 5.0, 1.0, 2.0, 3.0], 3, 2);

    //     assert_eq!(
    //         Matrix::new(vec![5.0, 8.0, 11.0, 13.0, 20.0, 27.0], 3, 2),
    //         &m1 * &m2,
    //     );
    // }
}
