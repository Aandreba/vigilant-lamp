use std::time::Instant;
use math::matrix::{Matrix2};

mod engine;
mod graph;
mod math;
mod extra;

fn main() {
    let start = Instant::now();

    let alpha = Matrix2::from_array([[1, 2], [3, 4]]);
    let beta = Matrix2::from_array([[5, 6], [7, 8]]);
    let result = alpha * beta;

    let end = Instant::now();
    let delta = end - start;

    println!();
    println!("{}", result);
    println!("{}", delta.as_nanos())
}