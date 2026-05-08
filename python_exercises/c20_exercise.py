from typing import Callable


def apply(x: int, f: Callable[[int], int]) -> int:
    return f(x)
