#[path = "../src/bin/c71_exercise.rs"]
#[allow(dead_code)]
mod c71_exercise;

use c71_exercise::{get_service, put_service, Service};

fn temp_db() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

#[test]
fn round_trips_a_struct() {
    let db = temp_db();
    let svc = Service { name: "Gel Manicure".to_string(), price: 4500 };
    put_service(&db, "svc:1", &svc).unwrap();
    assert_eq!(get_service(&db, "svc:1").unwrap(), Some(svc));
}

#[test]
fn missing_struct_is_none() {
    let db = temp_db();
    assert_eq!(get_service(&db, "svc:nope").unwrap(), None);
}
