use rand_distr::{Poisson, Distribution};
use rayon::prelude::{IntoParallelIterator};
use rayon::iter::ParallelIterator;

#[allow(unused)]
fn single_bench() -> Vec<f32> {
    let poi = Poisson::new(1.0).unwrap();
    (0..10_000_000).map(
        |_| poi.sample(&mut rand::thread_rng())
    ).collect()
}

fn single_bench_par() -> Vec<f32> {
    let poi: Poisson<f32> = Poisson::new(1.0).unwrap();
    (0..10_000_000).into_par_iter().map(
        |_| poi.sample(&mut rand::thread_rng())
    ).collect()
}

fn main() {
    single_bench_par();
}
