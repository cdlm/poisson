extern crate poisson;

extern crate rand;

extern crate nalgebra as na;
pub type Vect = na::Vec3<f64>;

mod helper;
use helper::test_with_samples;

#[test]
fn test_3d_1_80_normal() {
    test_with_samples::<Vect>(1, 0.8, 1600, false);
}

#[test]
fn test_3d_1_80_perioditic() {
    test_with_samples::<Vect>(1, 0.8, 800, true);
}

#[test]
fn test_3d_10_80_normal() {
    test_with_samples::<Vect>(10, 0.8, 400, false);
}

#[test]
fn test_3d_10_80_perioditic() {
    test_with_samples::<Vect>(10, 0.8, 200, true);
}

#[test]
fn test_3d_100_80_normal() {
    test_with_samples::<Vect>(100, 0.8, 200, false);
}

#[test]
fn test_3d_100_80_perioditic() {
    test_with_samples::<Vect>(100, 0.8, 100, true);
}