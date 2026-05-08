from c22_exercise import squared_evens

def test_squares_evens():
    assert squared_evens([1, 2, 3, 4]) == [4, 16]

def test_empty():
    assert squared_evens([]) == []

def test_no_evens():
    assert squared_evens([1, 3, 5, 7]) == []
