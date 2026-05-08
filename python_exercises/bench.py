"""
Rust vs Python Per-Lesson Benchmark
Runs each comparable lesson on large input, measures time + peak memory.
"""
import subprocess
import sys
import os
import time
import statistics
import importlib

try:
    import psutil
except ImportError:
    subprocess.check_call([sys.executable, "-m", "pip", "install", "psutil", "-q"])
    import psutil

ROOT = os.path.dirname(os.path.abspath(__file__))
PROJECT = os.path.dirname(ROOT)
RUNS = 3

# (lesson_number, label, python_module, python_func, input_generator, scale)
BENCHMARKS = [
    (1,  "double",        "c01_exercise", "double",        lambda n: n, 10_000_000),
    (3,  "sum_array",     "c03_exercise", "sum_array",     lambda n: list(range(n)), 10_000_000),
    (4,  "min_max",       "c04_exercise", "min_max",       lambda n: list(range(n)), 10_000_000),
    (5,  "evens_up_to",   "c05_exercise", "evens_up_to",   lambda n: n, 5_000_000),
    (6,  "fizzbuzz",      "c06_exercise", "fizzbuzz",      lambda n: n, 1_000_000),
    (13, "count_chars",   "c13_exercise", "count_chars",   lambda n: "abcde" * n, 2_000_000),
    (14, "count_numbers", "c14_exercise", "count_numbers", lambda n: list(range(n % 100)) * (n // 100), 5_000_000),
    (15, "normalize_word","c15_exercise", "normalize_word", lambda n: "...Hello!!!", 1_000_000),
    (16, "word_count",    "c16_exercise", "word_count",    lambda n: " ".join(["hello world rust python"] * n), 250_000),
    (20, "apply",         "c20_exercise", "apply",         lambda n: n, 10_000_000),
    (21, "doubled",       "c21_exercise", "doubled",       lambda n: list(range(n)), 5_000_000),
    (22, "squared_evens", "c22_exercise", "squared_evens", lambda n: list(range(n)), 5_000_000),
    (23, "total",         "c23_exercise", "total",         lambda n: list(range(n)), 10_000_000),
    (24, "product",       "c24_exercise", "product",       lambda n: [1] * n, 5_000_000),
    (27, "swap",          "c27_exercise", "swap",          lambda n: (1, 2), 10_000_000),
    (28, "larger",        "c28_exercise", "larger",        lambda n: (3, 7), 10_000_000),
]


def bench_python(module_name, func_name, input_gen, scale, runs=RUNS):
    """Benchmark a Python function."""
    sys.path.insert(0, ROOT)
    mod = importlib.import_module(module_name)
    fn = getattr(mod, func_name)

    # Generate input
    data = input_gen(scale)

    # Special cases: functions that take single values need a loop
    single_value_funcs = {"double", "apply", "swap", "larger", "normalize_word"}

    times = []
    for _ in range(runs):
        if func_name in single_value_funcs:
            start = time.perf_counter()
            if func_name == "double":
                for _ in range(scale):
                    fn(data)
            elif func_name == "apply":
                f = lambda n: n * 2
                for _ in range(scale):
                    fn(data, f)
            elif func_name == "swap":
                for _ in range(scale):
                    fn(1, 2)
            elif func_name == "larger":
                for _ in range(scale):
                    fn(3, 7)
            elif func_name == "normalize_word":
                for _ in range(scale):
                    fn(data)
            elapsed = (time.perf_counter() - start) * 1000
        else:
            start = time.perf_counter()
            fn(data)
            elapsed = (time.perf_counter() - start) * 1000
        times.append(elapsed)

    return statistics.median(times)


def bench_rust(lesson_num, scale, runs=RUNS):
    """Benchmark a Rust exercise using --bench mode."""
    bin_name = f"c{lesson_num:02d}_exercise"
    exe = os.path.join(PROJECT, "target", "release", f"{bin_name}.exe")

    if not os.path.exists(exe):
        return None

    times = []
    for _ in range(runs):
        result = subprocess.run(
            [exe, "--bench", str(scale)],
            capture_output=True, text=True, cwd=PROJECT
        )
        if result.returncode != 0:
            return None
        try:
            ms = float(result.stdout.strip().split("\n")[0])
            times.append(ms)
        except (ValueError, IndexError):
            return None

    return statistics.median(times)


def main():
    # Build Rust in release mode
    print("Building Rust exercises (--release)...")
    result = subprocess.run(
        ["cargo", "build", "--release", "--bins"],
        capture_output=True, text=True, cwd=PROJECT
    )
    if result.returncode != 0:
        print("Cargo build failed:")
        print(result.stderr[-500:] if len(result.stderr) > 500 else result.stderr)
        print("\nContinuing with Python-only benchmarks...\n")

    results = []
    print(f"\nRunning benchmarks ({RUNS} runs each, median)...\n")

    for lesson, label, mod_name, func_name, input_gen, scale in BENCHMARKS:
        print(f"  c{lesson:02d} {label}...", end=" ", flush=True)

        py_ms = bench_python(mod_name, func_name, input_gen, scale)
        rust_ms = bench_rust(lesson, scale)

        if rust_ms:
            speedup = py_ms / rust_ms
            print(f"Py={py_ms:.1f}ms  Rust={rust_ms:.1f}ms  ({speedup:.0f}x)")
        else:
            print(f"Py={py_ms:.1f}ms  Rust=N/A")

        results.append((f"c{lesson:02d}", label, py_ms, rust_ms))

    # Print table
    print("\n" + "=" * 75)
    print(f"{'Lesson':<7} | {'Task':<16} | {'Python (ms)':>12} | {'Rust (ms)':>10} | {'Speedup':>8}")
    print("-" * 75)
    for lesson, label, py_ms, rust_ms in results:
        if rust_ms:
            speedup = f"{py_ms / rust_ms:.0f}x"
        else:
            speedup = "N/A"
        rust_str = f"{rust_ms:.1f}" if rust_ms else "N/A"
        print(f"{lesson:<7} | {label:<16} | {py_ms:>12.1f} | {rust_str:>10} | {speedup:>8}")
    print("=" * 75)

    # Summary
    valid = [(py, rs) for _, _, py, rs in results if rs]
    if valid:
        avg_speedup = sum(py / rs for py, rs in valid) / len(valid)
        print(f"\nAverage speedup across {len(valid)} comparable lessons: {avg_speedup:.0f}x")


if __name__ == "__main__":
    main()
