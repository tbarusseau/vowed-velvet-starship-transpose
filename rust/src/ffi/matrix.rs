use crate::{matrix::Matrix, polynomial::Polynomial};

#[no_mangle]
/// # Safety
/// None, as we use data coming directly from the other side to create a Vec.
pub unsafe extern "C" fn matrix_new(
    content: *mut Polynomial,
    width: usize,
    height: usize,
) -> *mut Matrix {
    Box::into_raw(Box::new(Matrix::new(
        Vec::from_raw_parts(content, width * height, width * height),
        width,
        height,
    )))
}

#[no_mangle]
/// # Safety
/// None, as we drop an arbitrary address from the other side.
pub unsafe extern "C" fn matrix_free(matrix: *mut Matrix) {
    matrix.drop_in_place();
}

#[no_mangle]
/// # Safety
/// None, as we dereference raw pointers coming from the other side.
pub unsafe extern "C" fn matrix_add(a: *mut Matrix, b: *mut Matrix) -> *mut Matrix {
    let p1 = Box::from_raw(a);
    let p2 = Box::from_raw(b);

    Box::into_raw(Box::new(Matrix::add(&p1, &p2)))
}

#[no_mangle]
/// # Safety
/// None, as we dereference raw pointers coming from the other side.
pub unsafe extern "C" fn matrix_mul(a: *mut Matrix, b: *mut Matrix) -> *mut Matrix {
    let p1 = Box::from_raw(a);
    let p2 = Box::from_raw(b);

    Box::into_raw(Box::new(Matrix::mul(&p1, &p2)))
}

#[no_mangle]
/// # Safety
/// None, as we dereference raw pointers coming from the other side.
pub unsafe extern "C" fn matrix_add_in_ring(
    a: *mut Matrix,
    b: *mut Matrix,
    ring_degree: usize,
) -> *mut Matrix {
    let p1 = Box::from_raw(a);
    let p2 = Box::from_raw(b);

    Box::into_raw(Box::new(Matrix::add_in_ring(&p1, &p2, ring_degree)))
}

#[no_mangle]
/// # Safety
/// None, as we dereference raw pointers coming from the other side.
pub unsafe extern "C" fn matrix_mul_in_ring(
    a: *mut Matrix,
    b: *mut Matrix,
    ring_degree: usize,
) -> *mut Matrix {
    let p1 = Box::from_raw(a);
    let p2 = Box::from_raw(b);

    Box::into_raw(Box::new(Matrix::mul_in_ring(&p1, &p2, ring_degree)))
}
