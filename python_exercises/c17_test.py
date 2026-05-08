import pytest
from c17_exercise import parse_age

def test_valid():
    assert parse_age("25") == 25

def test_invalid():
    with pytest.raises(ValueError):
        parse_age("abc")

def test_overflow():
    with pytest.raises(ValueError):
        parse_age("300")
