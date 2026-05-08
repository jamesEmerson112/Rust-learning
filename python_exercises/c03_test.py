from c03_exercise import sum_array

def test_sum_positive():
    assert sum_array([1, 2, 3, 4]) == 10

def test_sum_mixed():
    assert sum_array([-5, 5, -10, 10]) == 0
