use crate::complex::Complex;

#[no_mangle]
pub extern "C" fn complex_new(real: f32, imaginary: f32) -> *mut Complex {
    Box::into_raw(Box::new(Complex::new(real, imaginary)))
}

#[no_mangle]
pub extern "C" fn complex_free(complex: *mut Complex) {
    // SAFETY: None, as we drop an arbitrary address from the other side.
    unsafe {
        complex.drop_in_place();
    }
}

#[no_mangle]
pub extern "C" fn complex_add(c1: *mut Complex, c2: *mut Complex) -> *mut Complex {
    // SAFETY: None, as we dereference raw pointers coming from the other side.
    unsafe {
        let b1 = Box::from_raw(c1);
        let b2 = Box::from_raw(c2);

        Box::into_raw(Box::new(*b1 + *b2))
    }
}

#[no_mangle]
pub extern "C" fn complex_mul(c1: *mut Complex, c2: *mut Complex) -> *mut Complex {
    // SAFETY: None, as we dereference raw pointers coming from the other side.
    unsafe {
        let b1 = Box::from_raw(c1);
        let b2 = Box::from_raw(c2);

        Box::into_raw(Box::new(*b1 * *b2))
    }
}
