use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub price: u32,
}

pub fn put_service(db: &sled::Db, key: &str, svc: &Service) -> anyhow::Result<()> {
    // TODO: Serialize svc to JSON bytes with serde_json::to_vec, then db.insert
    // it under `key`. anyhow's `?` unifies the serde + sled error types.
    let _ = (db, key, svc);
    Ok(())
}

pub fn get_service(db: &sled::Db, key: &str) -> anyhow::Result<Option<Service>> {
    // TODO: db.get(key)? gives Option<IVec>; deserialize the bytes with
    // serde_json::from_slice into a Service. Return None when the key is absent.
    let _ = (db, key);
    Ok(None)
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c71_exercise_sled_db")?;
    let svc = Service { name: "Gel".to_string(), price: 4500 };
    put_service(&db, "svc:1", &svc)?;
    println!("{:?}", get_service(&db, "svc:1")?);
    drop(db);
    std::fs::remove_dir_all("c71_exercise_sled_db").ok();
    Ok(())
}
