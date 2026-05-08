import time

nums = list(range(1, 10_000_001))

start = time.perf_counter()
result = [n * n for n in nums if n % 2 == 0]
elapsed = (time.perf_counter() - start) * 1000

print(f"{elapsed:.4f}")
print(f"len = {len(result)}", file=__import__('sys').stderr)
