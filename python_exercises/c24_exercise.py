from functools import reduce


def product(nums: list[int]) -> int:
    return reduce(lambda acc, n: acc * n, nums, 1)
