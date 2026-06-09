#[path = "../src/bin/c73_exercise.rs"]
#[allow(dead_code)]
mod c73_exercise;

use c73_exercise::{names_over, services_over, Service};

fn temp_db() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

fn seed(db: &sled::Db, name: &str, price: u32) {
    let bytes = serde_json::to_vec(&Service { name: name.to_string(), price }).unwrap();
    db.insert(name, bytes).unwrap();
}

#[test]
fn filters_by_min_price() {
    let db = temp_db();
    seed(&db, "Gel Manicure", 4500);
    seed(&db, "Pedicure", 3500);
    seed(&db, "Acrylic", 6000);
    let over = services_over(&db, 4000).unwrap();
    assert_eq!(
        over,
        vec![
            Service { name: "Acrylic".to_string(), price: 6000 },
            Service { name: "Gel Manicure".to_string(), price: 4500 },
        ]
    );
}

#[test]
fn projects_names_only() {
    let db = temp_db();
    seed(&db, "Gel Manicure", 4500);
    seed(&db, "Pedicure", 3500);
    seed(&db, "Acrylic", 6000);
    assert_eq!(
        names_over(&db, 4000).unwrap(),
        vec!["Acrylic".to_string(), "Gel Manicure".to_string()]
    );
}
