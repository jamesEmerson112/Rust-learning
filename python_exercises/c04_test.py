from c04_exercise import min_max

def test_min_max_positive():
    assert min_max([3, 1, 4, 1, 5]) == (1, 5)

def test_min_max_negative():
    assert min_max([-10, -5, 0, 5, 10]) == (-10, 10)

def test_min_max_all_same():
    assert min_max([7, 7, 7, 7, 7]) == (7, 7)
