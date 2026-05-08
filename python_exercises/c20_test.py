from c20_exercise import apply

def test_double():
    assert apply(5, lambda n: n * 2) == 10

def test_add_one():
    assert apply(41, lambda n: n + 1) == 42

def test_captures_scope():
    factor = 3
    assert apply(5, lambda n: n * factor) == 15
