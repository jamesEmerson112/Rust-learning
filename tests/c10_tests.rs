#[path = "../src/bin/c10_exercise.rs"]
#[allow(dead_code)]
mod c10_exercise;

use c10_exercise::Rectangle;

#[test]
fn rectangle_area_is_width_times_height() {
    let rectangle = Rectangle {
        width: 5,
        height: 6,
    };
    assert_eq!(rectangle.area(), 30);
}

#[test]
fn larger_rectangle_can_hold_smaller() {
    let outer = Rectangle {
        width: 10,
        height: 8,
    };
    let inner = Rectangle {
        width: 9,
        height: 7,
    };

    assert!(outer.can_hold(&inner));
}

#[test]
fn equal_dimension_cannot_hold() {
    let outer = Rectangle {
        width: 10,
        height: 8,
    };
    let inner = Rectangle {
        width: 10,
        height: 7,
    };

    assert!(!outer.can_hold(&inner));
}
