from c28_exercise import larger

def test_ints():
    assert larger(3, 7) == 7

def test_strings():
    assert larger("apple", "banana") == "banana"

def test_equal():
    assert larger(5, 5) == 5
