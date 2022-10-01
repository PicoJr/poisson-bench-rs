# Poisson Benchmark

Important note: `fish-rs` is multithreaded using `rayon`, and so is the function called in `bench_poisson_rust.py` but for now it appears it only uses a single core despite using the same parallelized Rust implementation.

Numpy vs Rust vs Python using Py03 bindings

* `bench_poisson.py` (Python, numpy)
* `bench_poisson_rust.py` (Python calling Rust using Py03)
* `fish-rs/target/release/fish-rs` (Rust)

## Quickstart

Create a new virtualenv, install numpy (required by `bench_poisson.py`), install maturin (required by `bench_poisson_rust.py`).

```
virtualenv .
pip install numpy maturin
source bin/activate
```

Build the Python lib using `maturin` (`-r` means build in release mode).

```
maturin develop -r
```

Build `fish-rs` binary for reference:

```
cd ./fish-rs
cargo build --release
```

## Results

Time for generating `10_000_000` values with a Poisson distribution.

```
❯ hyperfine --warmup 2 "python bench_poisson.py" "python bench_poisson_rust.py" "./fish-rs/target/release/fish-rs"
Benchmark 1: python bench_poisson.py
  Time (mean ± σ):     340.2 ms ±   2.1 ms    [User: 519.9 ms, System: 924.3 ms]
  Range (min … max):   337.4 ms … 343.4 ms    10 runs
 
Benchmark 2: python bench_poisson_rust.py
  Time (mean ± σ):     301.8 ms ±   1.2 ms    [User: 473.0 ms, System: 118.1 ms]
  Range (min … max):   299.6 ms … 304.0 ms    10 runs
 
Benchmark 3: ./fish-rs/target/release/fish-rs
  Time (mean ± σ):      20.0 ms ±   0.7 ms    [User: 253.9 ms, System: 28.1 ms]
  Range (min … max):    18.5 ms …  22.0 ms    139 runs


Summary
  './fish-rs/target/release/fish-rs' ran
   15.07 ± 0.53 times faster than 'python bench_poisson_rust.py'
   16.98 ± 0.61 times faster than 'python bench_poisson.py'
```
