def safe_divide(a: str, b: str) -> float:
    """Parse two strings as floats and divide. Raises ValueError on failure."""
    left = float(a)
    right = float(b)
    if right == 0.0:
        raise ValueError("cannot divide by zero")
    return left / right
