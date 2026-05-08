from c06_exercise import fizzbuzz

def test_first_five():
    assert fizzbuzz(5) == ["1", "2", "Fizz", "4", "Buzz"]

def test_fifteen_rule():
    assert fizzbuzz(15)[14] == "FizzBuzz"

def test_zero_is_empty():
    assert fizzbuzz(0) == []
