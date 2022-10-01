# Poisson Benchmark

Numpy vs Rust vs Python using Py03 bindings

* `bench_poisson.py` (Python, numpy) (single threaded)
* `bench_poisson_rust.py` (Python calling Rust using Py03) (Multithreaded)
* `fish-rs/target/release/fish-rs` (Rust) (Multithreaded)

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
  Time (mean ± σ):     340.7 ms ±   2.3 ms    [User: 520.1 ms, System: 915.2 ms]
  Range (min … max):   338.3 ms … 344.0 ms    10 runs
 
Benchmark 2: python bench_poisson_rust.py
  Time (mean ± σ):     106.5 ms ±   3.1 ms    [User: 524.2 ms, System: 841.4 ms]
  Range (min … max):   104.0 ms … 116.6 ms    27 runs
 
Benchmark 3: ./fish-rs/target/release/fish-rs
  Time (mean ± σ):      20.3 ms ±   1.0 ms    [User: 256.1 ms, System: 26.7 ms]
  Range (min … max):    18.6 ms …  24.1 ms    137 runs
 
Summary
  './fish-rs/target/release/fish-rs' ran
    5.25 ± 0.30 times faster than 'python bench_poisson_rust.py'
   16.80 ± 0.82 times faster than 'python bench_poisson.py'
```
