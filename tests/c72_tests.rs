#[path = "../src/bin/c72_exercise.rs"]
#[allow(dead_code)]
mod c72_exercise;

use c72_exercise::{all_services, Service};

fn temp_db() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

fn seed(db: &sled::Db, name: &str, price: u32) {
    let bytes = serde_json::to_vec(&Service { name: name.to_string(), price }).unwrap();
    db.insert(name, bytes).unwrap();
}

#[test]
fn iterates_all_entries() {
    let db = temp_db();
    seed(&db, "Gel Manicure", 4500);
    seed(&db, "Acrylic", 6000);
    seed(&db, "Pedicure", 3500);
    let all = all_services(&db).unwrap();
    assert_eq!(
        all,
        vec![
            Service { name: "Acrylic".to_string(), price: 6000 },
            Service { name: "Gel Manicure".to_string(), price: 4500 },
            Service { name: "Pedicure".to_string(), price: 3500 },
        ]
    );
}

#[test]
fn empty_db_is_empty_vec() {
    let db = temp_db();
    assert_eq!(all_services(&db).unwrap(), vec![]);
}
