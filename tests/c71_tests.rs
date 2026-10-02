#[path = "../src/bin/c71_exercise.rs"]
#[allow(dead_code)]
mod c71_exercise;

use c71_exercise::{Stock, decode_stock, encode_stock};

fn temp_db() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

#[test]
fn stock_record_round_trips() {
    let db = temp_db();
    let paracetamol = Stock { drug: "Paracetamol".to_string(), units: 6400 };
    encode_stock(&db, &paracetamol).unwrap();
    assert_eq!(decode_stock(&db, "Paracetamol").unwrap(), Some(paracetamol));
}

#[test]
fn unstocked_drug_is_none() {
    let db = temp_db();
    assert_eq!(decode_stock(&db, "Unknown").unwrap(), None);
}
