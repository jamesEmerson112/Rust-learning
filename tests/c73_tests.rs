#[path = "../src/bin/c73_exercise.rs"]
#[allow(dead_code)]
mod c73_exercise;

use c73_exercise::{Stock, low_stock, low_stock_names};

fn temp_db() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

fn seed(db: &sled::Db, drug: &str, units: u32) {
    let bytes = serde_json::to_vec(&Stock { drug: drug.to_string(), units }).unwrap();
    db.insert(drug, bytes).unwrap();
}

fn stocked_pharmacy() -> sled::Db {
    let db = temp_db();
    seed(&db, "Paracetamol", 6400);
    seed(&db, "Amoxicillin", 4200);
    seed(&db, "Insulin", 1850);
    seed(&db, "Saline", 5000); // exactly at the threshold — NOT low
    db
}

#[test]
fn reorder_list_keeps_only_low_stock() {
    let db = stocked_pharmacy();
    assert_eq!(
        low_stock(&db, 5000).unwrap(),
        vec![
            Stock { drug: "Amoxicillin".to_string(), units: 4200 },
            Stock { drug: "Insulin".to_string(), units: 1850 },
        ]
    );
}

#[test]
fn names_project_just_the_drugs() {
    let db = stocked_pharmacy();
    assert_eq!(
        low_stock_names(&db, 5000).unwrap(),
        vec!["Amoxicillin".to_string(), "Insulin".to_string()]
    );
}
