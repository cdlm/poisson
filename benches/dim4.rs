#![feature(test)]

extern crate test;
use test::{Bencher, black_box};

extern crate poisson;
use poisson::PoissonDisk;

extern crate rand;
use rand::{SeedableRng, XorShiftRng};

extern crate nalgebra as na;
pub type Vect = na::Vec4<f64>;

#[bench]
fn bench_4d_1_80_normal(b: &mut Bencher) {
    let rand = XorShiftRng::from_seed([1, 2, 3, 4]);
    let mut poisson = PoissonDisk::new(rand.clone()).build_samples::<Vect>(1, 0.8);
    b.iter(|| {
        let mut vecs = vec![];
        poisson.generate(&mut vecs);
        black_box(vecs);
    });
}

#[bench]
fn bench_4d_10_80_normal(b: &mut Bencher) {
    let rand = XorShiftRng::from_seed([1, 2, 3, 4]);
    let mut poisson = PoissonDisk::new(rand.clone()).build_samples::<Vect>(10, 0.8);
    b.iter(|| {
        let mut vecs = vec![];
        poisson.generate(&mut vecs);
        black_box(vecs);
    });
}

#[bench]
fn bench_4d_100_80_normal(b: &mut Bencher) {
    let rand = XorShiftRng::from_seed([1, 2, 3, 4]);
    let mut poisson = PoissonDisk::new(rand.clone()).build_samples::<Vect>(100, 0.8);
    b.iter(|| {
        let mut vecs = vec![];
        poisson.generate(&mut vecs);
        black_box(vecs);
    });
}
