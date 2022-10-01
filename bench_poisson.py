import numpy as np

def single_bench():
    rng = np.random.default_rng()
    s = rng.poisson(1.0, 10_000_000)
    return s

if __name__ == "__main__":
    single_bench()
