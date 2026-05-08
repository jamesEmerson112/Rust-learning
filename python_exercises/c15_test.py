from c15_exercise import normalize_word

def test_lowercases():
    assert normalize_word("HELLO") == "hello"

def test_trims_punctuation():
    assert normalize_word("hello,") == "hello"
    assert normalize_word("...rust!!!") == "rust"

def test_preserves_mid_word():
    assert normalize_word("42nd") == "42nd"

def test_all_punctuation():
    assert normalize_word("???") == ""
