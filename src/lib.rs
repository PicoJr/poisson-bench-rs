use rand_distr::{Poisson, Distribution};
use rayon::prelude::{IntoParallelIterator};
use rayon::iter::ParallelIterator;
use pyo3::prelude::*;

#[pyfunction]
fn single_bench_par(n: usize) -> PyResult<Vec<f32>> {
    let poi: Poisson<f32> = Poisson::new(1.0).unwrap();
    Ok((0..n).into_par_iter().map(
        |_| poi.sample(&mut rand::thread_rng())
    ).collect())
}

/// A Python module implemented in Rust.
#[pymodule]
fn fish(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(single_bench_par, m)?)?;
    Ok(())
}