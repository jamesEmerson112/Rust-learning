from c33_exercise import Gradebook

def test_average_known_student():
    gb = Gradebook()
    gb.add_score("Sam", 80)
    gb.add_score("Sam", 100)
    assert abs(gb.average("Sam") - 90.0) < 1e-9

def test_average_unknown():
    gb = Gradebook()
    assert gb.average("Unknown") is None

def test_average_precision():
    gb = Gradebook()
    gb.add_score("Ava", 89)
    gb.add_score("Ava", 90)
    assert round(gb.average("Ava") * 100) / 100 == 89.5
