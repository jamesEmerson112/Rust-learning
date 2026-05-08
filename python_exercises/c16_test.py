from c16_exercise import word_count

def test_case_insensitive():
    assert word_count("Rust rust RUST").get("rust") == 3

def test_trims_punctuation():
    counts = word_count("hello, world! hello.")
    assert counts.get("hello") == 2
    assert counts.get("world") == 1

def test_empty():
    assert word_count("") == {}

def test_pure_punctuation():
    assert word_count("??? !!! ...") == {}
