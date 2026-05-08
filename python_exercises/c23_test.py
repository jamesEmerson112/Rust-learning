from c23_exercise import total

def test_basic():
    assert total([1, 2, 3, 4]) == 10

def test_empty():
    assert total([]) == 0

def test_negatives():
    assert total([-1, 2, -3]) == -2
