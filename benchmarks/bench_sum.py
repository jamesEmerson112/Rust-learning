import time

nums = list(range(1, 10_000_001))

start = time.perf_counter()
total = sum(nums)
elapsed = (time.perf_counter() - start) * 1000

print(f"{elapsed:.4f}")
print(f"sum = {total}", file=__import__('sys').stderr)
