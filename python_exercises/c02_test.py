from c02_exercise import build_greeting

def test_greeting_matches():
    assert build_greeting("Sam", 20) == "Hello, Sam! You are 20 years old."

def test_greeting_empty_name():
    assert build_greeting("", 1) == "Hello, ! You are 1 years old."
