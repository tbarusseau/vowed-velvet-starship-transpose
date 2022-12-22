#[macro_use]
extern crate bencher;

use bencher::Bencher;
use libcomplex_polynomials::{complex::Complex, polynomial::Polynomial};

fn polynomials_generation(bench: &mut Bencher) {
    bench.iter(|| {
        Polynomial::new(vec![
            Complex::new(1.0, 1.0),
            Complex::new(1.0, 1.0),
            Complex::new(1.0, 1.0),
            Complex::new(1.0, 1.0),
            Complex::new(1.0, 1.0),
        ]);
    });
}

fn polynomials_add_regular(bench: &mut Bencher) {
    let p1 = Polynomial::new(vec![
        Complex::new(1.0, 2.0),
        Complex::new(0.0, -2.0),
        Complex::new(1.5, 0.0),
        Complex::new(-30.0, 3.0),
        Complex::new(15.0, 1.3),
        Complex::new(-5.5, 12.0),
    ]);
    let p2 = Polynomial::new(vec![
        Complex::new(2.0, 2.0),
        Complex::new(3.0, -3.0),
        Complex::new(2.5, 1.0),
        Complex::new(-10.0, 1.0),
        Complex::new(25.0, 2.8),
        Complex::new(-2.5, 16.5),
    ]);

    bench.iter(|| {
        Polynomial::add(&p1, &p2);
    });
}

fn polynomials_add_in_ring(bench: &mut Bencher) {
    let p1 = Polynomial::new(vec![
        Complex::new(1.0, 2.0),
        Complex::new(0.0, -2.0),
        Complex::new(1.5, 0.0),
        Complex::new(-30.0, 3.0),
        Complex::new(15.0, 1.3),
        Complex::new(-5.5, 12.0),
    ]);
    let p2 = Polynomial::new(vec![
        Complex::new(2.0, 2.0),
        Complex::new(3.0, -3.0),
        Complex::new(2.5, 1.0),
        Complex::new(-10.0, 1.0),
        Complex::new(25.0, 2.8),
        Complex::new(-2.5, 16.5),
    ]);

    bench.iter(|| {
        Polynomial::add_in_ring(&p1, &p2, 3);
    });
}

fn polynomials_mul_regular(bench: &mut Bencher) {
    let p1 = Polynomial::new(vec![
        Complex::new(1.0, 2.0),
        Complex::new(0.0, -2.0),
        Complex::new(1.5, 0.0),
        Complex::new(-30.0, 3.0),
        Complex::new(15.0, 1.3),
        Complex::new(-5.5, 12.0),
    ]);
    let p2 = Polynomial::new(vec![
        Complex::new(2.0, 2.0),
        Complex::new(3.0, -3.0),
        Complex::new(2.5, 1.0),
        Complex::new(-10.0, 1.0),
        Complex::new(25.0, 2.8),
        Complex::new(-2.5, 16.5),
    ]);

    bench.iter(|| {
        Polynomial::mul(&p1, &p2);
    });
}

fn polynomials_mul_in_ring(bench: &mut Bencher) {
    let p1 = Polynomial::new(vec![
        Complex::new(1.0, 2.0),
        Complex::new(0.0, -2.0),
        Complex::new(1.5, 0.0),
        Complex::new(-30.0, 3.0),
        Complex::new(15.0, 1.3),
        Complex::new(-5.5, 12.0),
    ]);
    let p2 = Polynomial::new(vec![
        Complex::new(2.0, 2.0),
        Complex::new(3.0, -3.0),
        Complex::new(2.5, 1.0),
        Complex::new(-10.0, 1.0),
        Complex::new(25.0, 2.8),
        Complex::new(-2.5, 16.5),
    ]);

    bench.iter(|| {
        Polynomial::mul_in_ring(&p1, &p2, 3);
    });
}

benchmark_group!(
    benches,
    polynomials_generation,
    polynomials_add_regular,
    polynomials_add_in_ring,
    polynomials_mul_regular,
    polynomials_mul_in_ring,
);
benchmark_main!(benches);
