from c05_exercise import evens_up_to

def test_evens_up_to_six():
    assert evens_up_to(6) == [2, 4, 6]

def test_evens_up_to_zero():
    assert evens_up_to(0) == []

def test_evens_up_to_one():
    assert evens_up_to(1) == []
