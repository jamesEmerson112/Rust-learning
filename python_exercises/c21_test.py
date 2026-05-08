from c21_exercise import doubled

def test_basic():
    assert doubled([1, 2, 3]) == [2, 4, 6]

def test_empty():
    assert doubled([]) == []

def test_negatives():
    assert doubled([-1, -2, 0]) == [-2, -4, 0]
