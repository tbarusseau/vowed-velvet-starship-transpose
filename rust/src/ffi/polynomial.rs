use rand::thread_rng;

use crate::{complex::Complex, polynomial::Polynomial};

#[no_mangle]
/// # Safety
/// None, as we use data coming directly from the other side to create a Vec.
pub unsafe extern "C" fn polynomial_new(content: *mut Complex, len: usize) -> *mut Polynomial {
    unsafe {
        Box::into_raw(Box::new(Polynomial::new(Vec::from_raw_parts(
            content, len, len,
        ))))
    }
}

#[no_mangle]
/// # Safety
/// None, as we drop an arbitrary address from the other side.
pub unsafe extern "C" fn polynomial_free(poly: *mut Polynomial) {
    unsafe {
        poly.drop_in_place();
    }
}

#[no_mangle]
pub extern "C" fn gen_random_polynomial() -> *mut Polynomial {
    let mut rand = thread_rng();

    Box::into_raw(Box::new(Polynomial::random(&mut rand)))
}

#[no_mangle]
/// # Safety
/// None, as we dereference raw pointers coming from the other side.
pub unsafe extern "C" fn polynomial_add(a: *mut Polynomial, b: *mut Polynomial) -> *mut Polynomial {
    unsafe {
        let p1 = Box::from_raw(a);
        let p2 = Box::from_raw(b);

        Box::into_raw(Box::new(Polynomial::add(&p1, &p2)))
    }
}

#[no_mangle]
/// # Safety
/// None, as we dereference raw pointers coming from the other side.
pub unsafe extern "C" fn polynomial_mul(a: *mut Polynomial, b: *mut Polynomial) -> *mut Polynomial {
    unsafe {
        let p1 = Box::from_raw(a);
        let p2 = Box::from_raw(b);

        Box::into_raw(Box::new(Polynomial::mul(&p1, &p2)))
    }
}

#[no_mangle]
/// # Safety
/// None, as we dereference raw pointers coming from the other side.
pub unsafe extern "C" fn polynomial_add_in_ring(
    a: *mut Polynomial,
    b: *mut Polynomial,
    ring_degree: usize,
) -> *mut Polynomial {
    unsafe {
        let p1 = Box::from_raw(a);
        let p2 = Box::from_raw(b);

        Box::into_raw(Box::new(Polynomial::add_in_ring(&p1, &p2, ring_degree)))
    }
}

#[no_mangle]
/// # Safety
/// None, as we dereference raw pointers coming from the other side.
pub unsafe extern "C" fn polynomial_mul_in_ring(
    a: *mut Polynomial,
    b: *mut Polynomial,
    ring_degree: usize,
) -> *mut Polynomial {
    let p1 = Box::from_raw(a);
    let p2 = Box::from_raw(b);

    Box::into_raw(Box::new(Polynomial::mul_in_ring(&p1, &p2, ring_degree)))
}
