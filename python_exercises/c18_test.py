import pytest
from c18_exercise import parse_pair

def test_both_valid():
    assert parse_pair("3", "4") == (3, 4)

def test_first_invalid():
    with pytest.raises(ValueError):
        parse_pair("x", "4")

def test_second_invalid():
    with pytest.raises(ValueError):
        parse_pair("3", "y")

def test_negatives():
    assert parse_pair("-5", "10") == (-5, 10)
