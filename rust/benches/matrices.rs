#[macro_use]
extern crate bencher;

use bencher::Bencher;
use libcomplex_polynomials::{complex::Complex, matrix::Matrix, polynomial::Polynomial};
use rand::thread_rng;

fn matrices_add_regular(bench: &mut Bencher) {
    let p1 = Polynomial::new(vec![
        Complex::new(1.0, 2.0),
        Complex::new(0.0, -2.0),
        Complex::new(1.5, 0.0),
        Complex::new(-30.0, 3.0),
    ]);
    let p2 = Polynomial::new(vec![
        Complex::new(1.5, 0.0),
        Complex::new(-30.0, 3.0),
        Complex::new(15.0, 1.3),
        Complex::new(-5.5, 12.0),
    ]);
    let p3 = Polynomial::new(vec![
        Complex::new(3.5, -1.0),
        Complex::new(0.0, -3.0),
        Complex::new(5.0, 1.2),
        Complex::new(-2.5, 18.0),
    ]);
    let p4 = Polynomial::new(vec![
        Complex::new(5.0, 1.2),
        Complex::new(-2.5, 18.0),
        Complex::new(3.5, -1.0),
        Complex::new(0.0, -3.0),
    ]);
    let m1 = Matrix::new(vec![p1.clone(), p2.clone(), p3.clone(), p4.clone()], 2, 2);
    let m2 = Matrix::new(vec![p2, p3, p4, p1], 2, 2);

    bench.iter(|| {
        Matrix::add(&m1, &m2);
    });
}

fn matrices_add_regular_big(bench: &mut Bencher) {
    let mut rand = thread_rng();

    let m1 = Matrix::random(8, 8, &mut rand);
    let m2 = Matrix::random(8, 8, &mut rand);

    bench.iter(|| {
        Matrix::add(&m1, &m2);
    });
}

fn matrices_add_in_ring(bench: &mut Bencher) {
    let p1 = Polynomial::new(vec![
        Complex::new(1.0, 2.0),
        Complex::new(0.0, -2.0),
        Complex::new(1.5, 0.0),
        Complex::new(-30.0, 3.0),
    ]);
    let p2 = Polynomial::new(vec![
        Complex::new(1.5, 0.0),
        Complex::new(-30.0, 3.0),
        Complex::new(15.0, 1.3),
        Complex::new(-5.5, 12.0),
    ]);
    let p3 = Polynomial::new(vec![
        Complex::new(3.5, -1.0),
        Complex::new(0.0, -3.0),
        Complex::new(5.0, 1.2),
        Complex::new(-2.5, 18.0),
    ]);
    let p4 = Polynomial::new(vec![
        Complex::new(5.0, 1.2),
        Complex::new(-2.5, 18.0),
        Complex::new(3.5, -1.0),
        Complex::new(0.0, -3.0),
    ]);
    let m1 = Matrix::new(vec![p1.clone(), p2.clone(), p3.clone(), p4.clone()], 2, 2);
    let m2 = Matrix::new(vec![p2, p3, p4, p1], 2, 2);

    bench.iter(|| {
        Matrix::add_in_ring(&m1, &m2, 2);
    });
}

fn matrices_mul_regular(bench: &mut Bencher) {
    let p1 = Polynomial::new(vec![
        Complex::new(1.0, 2.0),
        Complex::new(0.0, -2.0),
        Complex::new(1.5, 0.0),
        Complex::new(-30.0, 3.0),
    ]);
    let p2 = Polynomial::new(vec![
        Complex::new(1.5, 0.0),
        Complex::new(-30.0, 3.0),
        Complex::new(15.0, 1.3),
        Complex::new(-5.5, 12.0),
    ]);
    let p3 = Polynomial::new(vec![
        Complex::new(3.5, -1.0),
        Complex::new(0.0, -3.0),
        Complex::new(5.0, 1.2),
        Complex::new(-2.5, 18.0),
    ]);
    let p4 = Polynomial::new(vec![
        Complex::new(5.0, 1.2),
        Complex::new(-2.5, 18.0),
        Complex::new(3.5, -1.0),
        Complex::new(0.0, -3.0),
    ]);
    let m1 = Matrix::new(vec![p1.clone(), p2.clone(), p3.clone(), p4.clone()], 2, 2);
    let m2 = Matrix::new(vec![p2, p3, p4, p1], 2, 2);

    bench.iter(|| {
        Matrix::mul(&m1, &m2);
    });
}

fn matrices_mul_in_ring(bench: &mut Bencher) {
    let p1 = Polynomial::new(vec![
        Complex::new(1.0, 2.0),
        Complex::new(0.0, -2.0),
        Complex::new(1.5, 0.0),
        Complex::new(-30.0, 3.0),
    ]);
    let p2 = Polynomial::new(vec![
        Complex::new(1.5, 0.0),
        Complex::new(-30.0, 3.0),
        Complex::new(15.0, 1.3),
        Complex::new(-5.5, 12.0),
    ]);
    let p3 = Polynomial::new(vec![
        Complex::new(3.5, -1.0),
        Complex::new(0.0, -3.0),
        Complex::new(5.0, 1.2),
        Complex::new(-2.5, 18.0),
    ]);
    let p4 = Polynomial::new(vec![
        Complex::new(5.0, 1.2),
        Complex::new(-2.5, 18.0),
        Complex::new(3.5, -1.0),
        Complex::new(0.0, -3.0),
    ]);
    let m1 = Matrix::new(vec![p1.clone(), p2.clone(), p3.clone(), p4.clone()], 2, 2);
    let m2 = Matrix::new(vec![p2, p3, p4, p1], 2, 2);

    bench.iter(|| {
        Matrix::mul_in_ring(&m1, &m2, 2);
    });
}

benchmark_group!(
    benches,
    matrices_add_regular,
    matrices_add_regular_big,
    matrices_add_in_ring,
    matrices_mul_regular,
    matrices_mul_in_ring
);
benchmark_main!(benches);
