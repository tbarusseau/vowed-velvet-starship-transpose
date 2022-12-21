use rand::{thread_rng, Rng};

use crate::{complex::Complex, polynomial::Polynomial};

#[no_mangle]
pub extern "C" fn polynomial_new(content: *mut Complex, len: usize) -> *mut Polynomial {
    // SAFETY: None, as we use data coming directly from the other side to create a Vec.
    unsafe {
        Box::into_raw(Box::new(Polynomial::new(Vec::from_raw_parts(
            content, len, len,
        ))))
    }
}

#[no_mangle]
pub extern "C" fn polynomial_free(poly: *mut Polynomial) {
    // SAFETY: None, as we drop an arbitrary address from the other side.
    unsafe {
        poly.drop_in_place();
    }
}

#[no_mangle]
pub extern "C" fn gen_random_polynomial() -> *mut Polynomial {
    let mut rand = thread_rng();

    let degree = rand.gen_range(1..8);

    Box::into_raw(Box::new(Polynomial::new(
        (0..=degree)
            .into_iter()
            .map(|i| {
                if rand.gen_bool(0.7) || i == degree {
                    Complex::random(&mut rand)
                } else {
                    Complex::ZERO
                }
            })
            .collect(),
    )))
}

#[no_mangle]
pub extern "C" fn polynomial_add(a: *mut Polynomial, b: *mut Polynomial) -> *mut Polynomial {
    // SAFETY: None, as we dereference raw pointers coming from the other side.
    unsafe {
        let p1 = Box::from_raw(a);
        let p2 = Box::from_raw(b);

        Box::into_raw(Box::new(Polynomial::add(&*p1, &*p2)))
    }
}

#[no_mangle]
pub extern "C" fn polynomial_mul(a: *mut Polynomial, b: *mut Polynomial) -> *mut Polynomial {
    // SAFETY: None, as we dereference raw pointers coming from the other side.
    unsafe {
        let p1 = Box::from_raw(a);
        let p2 = Box::from_raw(b);

        Box::into_raw(Box::new(Polynomial::mul(&*p1, &*p2)))
    }
}

#[no_mangle]
pub extern "C" fn polynomial_add_in_ring(
    a: *mut Polynomial,
    b: *mut Polynomial,
    ring_degree: usize,
) -> *mut Polynomial {
    // SAFETY: None, as we dereference raw pointers coming from the other side.
    unsafe {
        let p1 = Box::from_raw(a);
        let p2 = Box::from_raw(b);

        Box::into_raw(Box::new(Polynomial::add_in_ring(&*p1, &*p2, ring_degree)))
    }
}

#[no_mangle]
pub extern "C" fn polynomial_mul_in_ring(
    a: *mut Polynomial,
    b: *mut Polynomial,
    ring_degree: usize,
) -> *mut Polynomial {
    // SAFETY: None, as we dereference raw pointers coming from the other side.
    unsafe {
        let p1 = Box::from_raw(a);
        let p2 = Box::from_raw(b);

        Box::into_raw(Box::new(Polynomial::mul_in_ring(&*p1, &*p2, ring_degree)))
    }
}
