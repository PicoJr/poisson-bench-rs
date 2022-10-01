import fish

def single_bench():
    s = fish.single_bench_par(10_000_000)
    return s

if __name__ == "__main__":
    single_bench()
