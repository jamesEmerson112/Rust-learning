def parse_age(text: str) -> int:
    """Parse string to age (0-255). Raises ValueError on failure."""
    n = int(text.strip())
    if n < 0 or n > 255:
        raise ValueError(f"invalid age: {text}")
    return n
