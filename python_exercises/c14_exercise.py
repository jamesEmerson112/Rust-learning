def count_numbers(nums: list[int]) -> dict[int, int]:
    counts: dict[int, int] = {}
    for n in nums:
        counts[n] = counts.get(n, 0) + 1
    return counts
