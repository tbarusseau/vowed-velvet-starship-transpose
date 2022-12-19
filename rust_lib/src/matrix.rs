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

    /// Add two matrices. The sizes of `a` and `b` must match.
    pub fn add(a: &Matrix, b: &Matrix) -> Matrix {
        assert_eq!(a.width, b.width);
        assert_eq!(a.height, b.height);

        let mut result = vec![Polynomial::zero(); b.width * a.height];

        for i in 0..a.height {
            for j in 0..b.width {
                let idx = j + i * b.width;

                result[idx] = Polynomial::add(&a.content[idx], &b.content[idx]);
            }
        }

        Matrix {
            width: b.width,
            height: a.height,
            content: result,
        }
    }

    /// Multiply two matrices. The width of `a` must match the height of `b`.
    pub fn mul(a: &Matrix, b: &Matrix) -> Matrix {
        assert_eq!(a.width, b.height);

        let mut result = vec![Polynomial::zero(); b.width * a.height];

        for i in 0..a.height {
            for j in 0..b.width {
                for k in 0..a.width {
                    let mul =
                        Polynomial::mul(&a.content[k + i * a.width], &b.content[j + k * b.width]);
                    let add = Polynomial::add(&result[j + i * b.width], &mul);

                    result[j + i * b.width] = add;
                }
            }
        }

        Matrix {
            width: b.width,
            height: a.height,
            content: result,
        }
    }

    /// Add two matrices while restricting the contained polynomials to the provided
    /// ring degree. `a` and `b` must have the same size.
    pub fn add_in_ring(a: &Matrix, b: &Matrix, ring: usize) -> Matrix {
        assert_eq!(a.width, b.width);
        assert_eq!(a.height, b.height);

        let mut result = vec![Polynomial::zero(); b.width * a.height];

        for i in 0..a.height {
            for j in 0..b.width {
                let idx = j + i * b.width;

                result[idx] = Polynomial::add_in_ring(&a.content[idx], &b.content[idx], ring);
            }
        }

        Matrix {
            width: b.width,
            height: a.height,
            content: result,
        }
    }

    /// Multiply two matrices while restricting the contained polynomials to the provided
    /// ring degree. The width of `a` must match the height of `b`.
    pub fn mul_in_ring(a: &Matrix, b: &Matrix, ring: usize) -> Matrix {
        assert_eq!(a.width, b.height);

        let mut result = vec![Polynomial::zero(); b.width * a.height];

        for i in 0..a.height {
            for j in 0..b.width {
                for k in 0..a.width {
                    let mul = Polynomial::mul_in_ring(
                        &a.content[k + i * a.width],
                        &b.content[j + k * b.width],
                        ring,
                    );
                    let add = Polynomial::add_in_ring(&result[j + i * b.width], &mul, ring);

                    result[j + i * b.width] = add;
                }
            }
        }

        Matrix {
            width: b.width,
            height: a.height,
            content: result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add at least a single test for sum/multiplication.
}
