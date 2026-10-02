#[path = "../src/bin/c74_exercise.rs"]
#[allow(dead_code)]
mod c74_exercise;

use c74_exercise::stock::{Stock, decode_stock, encode_stock};
use c74_exercise::{import_delivery, most_stocked};

fn temp_db() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

#[test]
fn your_c71_codec_is_wired_in() {
    let db = temp_db();
    let paracetamol = Stock { drug: "Paracetamol".to_string(), units: 6400 };
    encode_stock(&db, &paracetamol).unwrap();
    assert_eq!(
        decode_stock(&db, "Paracetamol").unwrap(),
        Some(paracetamol),
        "the capstone runs on YOUR c71 codec — finish c71 first"
    );
}

#[test]
fn imports_the_delivery_and_totals_the_stock() {
    let path = "test_c74_delivery.csv";
    std::fs::write(path, "drug,units\nAmoxicillin,4200\nInsulin,1850\nParacetamol,6400\n").unwrap();
    let db = temp_db();
    let total = import_delivery(&db, path).unwrap();
    std::fs::remove_file(path).unwrap();
    assert_eq!(total, 12_450);
    assert_eq!(db.len(), 3);
}

#[test]
fn most_stocked_is_the_largest_count() {
    let db = temp_db();
    for (drug, units) in [("Amoxicillin", 4200u32), ("Paracetamol", 6400), ("Insulin", 1850)] {
        encode_stock(&db, &Stock { drug: drug.to_string(), units }).unwrap();
    }
    assert_eq!(
        most_stocked(&db).unwrap(),
        Some(Stock { drug: "Paracetamol".to_string(), units: 6400 })
    );
}

#[test]
fn missing_delivery_note_is_err() {
    let db = temp_db();
    assert!(import_delivery(&db, "nonexistent_delivery.csv").is_err());
}
