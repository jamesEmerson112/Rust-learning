from c09_exercise import Point, distance_from_origin

def test_three_four_is_five():
    assert abs(distance_from_origin(Point(3.0, 4.0)) - 5.0) < 1e-9

def test_origin_is_zero():
    assert abs(distance_from_origin(Point(0.0, 0.0)) - 0.0) < 1e-9
