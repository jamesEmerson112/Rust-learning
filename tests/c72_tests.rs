#[path = "../src/bin/c72_exercise.rs"]
#[allow(dead_code)]
mod c72_exercise;

use c72_exercise::{Stock, full_inventory};

fn temp_db() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

fn seed(db: &sled::Db, drug: &str, units: u32) {
    let bytes = serde_json::to_vec(&Stock { drug: drug.to_string(), units }).unwrap();
    db.insert(drug, bytes).unwrap();
}

#[test]
fn stock_take_lists_every_drug() {
    let db = temp_db();
    seed(&db, "Paracetamol", 6400);
    seed(&db, "Amoxicillin", 4200);
    seed(&db, "Insulin", 1850);
    assert_eq!(
        full_inventory(&db).unwrap(),
        vec![
            Stock { drug: "Amoxicillin".to_string(), units: 4200 },
            Stock { drug: "Insulin".to_string(), units: 1850 },
            Stock { drug: "Paracetamol".to_string(), units: 6400 },
        ]
    );
}

#[test]
fn empty_pharmacy_lists_nothing() {
    let db = temp_db();
    assert_eq!(full_inventory(&db).unwrap(), vec![]);
}
