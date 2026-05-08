from c01_exercise import double

def test_double_positive():
    assert double(5) == 10

def test_double_negative():
    assert double(-3) == -6
