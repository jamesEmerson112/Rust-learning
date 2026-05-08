from c27_exercise import swap

def test_integers():
    assert swap(1, 2) == (2, 1)

def test_strings():
    assert swap("a", "b") == ("b", "a")

def test_bools():
    assert swap(True, False) == (False, True)
