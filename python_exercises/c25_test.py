from c25_exercise import debug_string

def test_basic():
    assert debug_string([1, 2, 3]) == "[1, 2, 3]"

def test_empty():
    assert debug_string([]) == "[]"

def test_single():
    assert debug_string([42]) == "[42]"
