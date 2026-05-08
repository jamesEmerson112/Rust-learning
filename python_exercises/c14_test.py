from c14_exercise import count_numbers

def test_single_value():
    assert count_numbers([7, 7, 7]).get(7) == 3

def test_multiple_values():
    counts = count_numbers([1, 2, 2, 3, 3, 3])
    assert counts.get(1) == 1
    assert counts.get(2) == 2
    assert counts.get(3) == 3

def test_empty():
    assert count_numbers([]) == {}
