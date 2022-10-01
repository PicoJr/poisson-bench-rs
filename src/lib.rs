use numpy::pyo3::Python;
use numpy::PyArray1;
use pyo3::prelude::*;
use rand_distr::{Distribution, Poisson};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

#[pyfunction]
fn single_bench_par(py: Python<'_>, n: usize) -> PyResult<&PyArray1<f32>> {
    let poi: Poisson<f32> = Poisson::new(1.0).unwrap();
    let v = (0..n)
        .into_par_iter()
        .map(|_| poi.sample(&mut rand::thread_rng()))
        .collect::<Vec<f32>>();
    Ok(PyArray1::from_vec(py, v))
}

/// A Python module implemented in Rust.
#[pymodule]
fn fish(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(single_bench_par, m)?)?;
    Ok(())
}
