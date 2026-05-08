from c13_exercise import count_chars

def test_count_basic():
    counts = count_chars("aab")
    assert counts.get("a") == 2
    assert counts.get("b") == 1

def test_count_empty():
    assert count_chars("") == {}

def test_count_repeated():
    counts = count_chars("aaaa")
    assert counts.get("a") == 4
    assert len(counts) == 1
