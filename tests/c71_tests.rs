#[path = "../src/bin/c71_exercise.rs"]
#[allow(dead_code)]
mod c71_exercise;

use c71_exercise::{Intel, decode_intel, encode_intel};

fn temp_vault() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

#[test]
fn intel_round_trips() {
    let db = temp_vault();
    let jewel = Intel { codename: "GHOSTKEY".to_string(), value: 64000 };
    encode_intel(&db, &jewel).unwrap();
    assert_eq!(decode_intel(&db, "GHOSTKEY").unwrap(), Some(jewel));
}

#[test]
fn burned_intel_is_none() {
    let db = temp_vault();
    assert_eq!(decode_intel(&db, "BURNED").unwrap(), None);
}
