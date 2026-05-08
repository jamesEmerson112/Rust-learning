"""
Rust vs Python Benchmark Runner
Runs each benchmark 5 times, takes median time + peak memory.
"""
import subprocess
import sys
import os
import statistics

try:
    import psutil
except ImportError:
    print("Installing psutil...")
    subprocess.check_call([sys.executable, "-m", "pip", "install", "psutil", "-q"])
    import psutil

BENCHMARKS = [
    ("Sum 10M",        "bench_sum"),
    ("Filter+Map 10M", "bench_filter_map"),
    ("Word Count 1M",  "bench_wordcount"),
]

RUNS = 5
ROOT = os.path.dirname(os.path.abspath(__file__))
PROJECT = os.path.dirname(ROOT)


def compile_rust():
    """Compile all Rust benchmarks with optimizations."""
    print("Compiling Rust benchmarks (--release)...")
    for _, name in BENCHMARKS:
        src = os.path.join(ROOT, f"{name}.rs")
        out = os.path.join(ROOT, f"{name}.exe")
        result = subprocess.run(
            ["rustc", "-O", "-o", out, src],
            capture_output=True, text=True
        )
        if result.returncode != 0:
            print(f"  FAILED to compile {name}:")
            print(result.stderr)
            sys.exit(1)
    print("  Done.\n")


def measure(cmd, runs=RUNS):
    """Run a command `runs` times, return median time (ms) and peak memory (KB)."""
    times = []
    peak_mem = 0

    for _ in range(runs):
        proc = subprocess.Popen(
            cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
            cwd=ROOT
        )
        ps = psutil.Process(proc.pid)

        local_peak = 0
        while proc.poll() is None:
            try:
                mem = ps.memory_info().rss
                if mem > local_peak:
                    local_peak = mem
            except (psutil.NoSuchProcess, psutil.AccessDenied):
                break

        stdout, _ = proc.communicate()
        ms = float(stdout.decode().strip())
        times.append(ms)
        if local_peak > peak_mem:
            peak_mem = local_peak

    return statistics.median(times), peak_mem / 1024  # KB


def main():
    compile_rust()

    results = []

    for label, name in BENCHMARKS:
        rust_exe = os.path.join(ROOT, f"{name}.exe")
        py_file = os.path.join(ROOT, f"{name}.py")

        print(f"Benchmarking: {label}")
        print(f"  Rust ({RUNS} runs)...", end=" ", flush=True)
        rust_ms, rust_kb = measure([rust_exe])
        print(f"{rust_ms:.1f} ms")

        print(f"  Python ({RUNS} runs)...", end=" ", flush=True)
        py_ms, py_kb = measure([sys.executable, py_file])
        print(f"{py_ms:.1f} ms")
        print()

        results.append((label, rust_ms, py_ms, rust_kb, py_kb))

    # Print table
    print("=" * 85)
    print(f"{'Task':<18} | {'Rust (ms)':>10} | {'Python (ms)':>12} | {'Speedup':>8} | {'Rust Mem':>10} | {'Py Mem':>10}")
    print("-" * 85)
    for label, r_ms, p_ms, r_kb, p_kb in results:
        speedup = p_ms / r_ms if r_ms > 0 else float('inf')
        r_mem = f"{r_kb/1024:.1f} MB" if r_kb > 1024 else f"{r_kb:.0f} KB"
        p_mem = f"{p_kb/1024:.1f} MB" if p_kb > 1024 else f"{p_kb:.0f} KB"
        print(f"{label:<18} | {r_ms:>10.1f} | {p_ms:>12.1f} | {speedup:>7.0f}x | {r_mem:>10} | {p_mem:>10}")
    print("=" * 85)


if __name__ == "__main__":
    main()
