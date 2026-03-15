#[path = "../src/bin/c17_exercise.rs"]
#[allow(dead_code)]
mod c17_exercise;

use c17_exercise::{Describable, Item};

#[test]
fn describe_item_basic() {
    let item = Item {
        name: "Widget".to_string(),
        price: 9.99,
    };
    assert_eq!(item.describe(), "Widget: $9.99");
}

#[test]
fn describe_item_whole_number() {
    let item = Item {
        name: "Bolt".to_string(),
        price: 3.0,
    };
    assert_eq!(item.describe(), "Bolt: $3.00");
}
