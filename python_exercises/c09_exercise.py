import math
from dataclasses import dataclass


@dataclass
class Point:
    x: float
    y: float


def distance_from_origin(p: Point) -> float:
    return math.sqrt(p.x * p.x + p.y * p.y)
