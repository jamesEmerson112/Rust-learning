from c24_exercise import product

def test_basic():
    assert product([1, 2, 3, 4]) == 24

def test_empty():
    assert product([]) == 1

def test_with_zero():
    assert product([5, 0, 3]) == 0
