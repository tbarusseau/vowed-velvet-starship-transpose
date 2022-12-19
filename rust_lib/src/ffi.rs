use crate::{complex::Complex, matrix::Matrix, polynomial::Polynomial};

#[no_mangle]
pub extern "C" fn matrix_new(content: *mut Polynomial, width: usize, height: usize) -> *mut Matrix {
    unsafe {
        Box::into_raw(Box::new(Matrix::new(
            Vec::from_raw_parts(content, width * height, width * height),
            width,
            height,
        )))
    }
}

#[no_mangle]
pub extern "C" fn polynomial_new(content: *mut Complex, len: usize) -> *mut Polynomial {
    unsafe {
        Box::into_raw(Box::new(Polynomial::new(Vec::from_raw_parts(
            content, len, len,
        ))))
    }
}

#[no_mangle]
pub extern "C" fn complex_new(real: f32, imaginary: f32) -> *mut Complex {
    Box::into_raw(Box::new(Complex::new(real, imaginary)))
}

#[no_mangle]
pub extern "C" fn complex_add(c1: *mut Complex, c2: *mut Complex) -> *mut Complex {
    unsafe {
        let b1 = Box::from_raw(c1);
        let b2 = Box::from_raw(c2);

        Box::into_raw(Box::new(*b1 + *b2))
    }
}

#[no_mangle]
pub extern "C" fn complex_mul(c1: *mut Complex, c2: *mut Complex) -> *mut Complex {
    unsafe {
        let b1 = Box::from_raw(c1);
        let b2 = Box::from_raw(c2);

        Box::into_raw(Box::new(*b1 * *b2))
    }
}
