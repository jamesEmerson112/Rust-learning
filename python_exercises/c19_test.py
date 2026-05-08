import pytest
from c19_exercise import safe_divide

def test_success():
    assert abs(safe_divide("10", "4") - 2.5) < 1e-9

def test_invalid_a():
    with pytest.raises(ValueError):
        safe_divide("x", "4")

def test_invalid_b():
    with pytest.raises(ValueError):
        safe_divide("4", "y")

def test_divide_by_zero():
    with pytest.raises(ValueError):
        safe_divide("4", "0")
