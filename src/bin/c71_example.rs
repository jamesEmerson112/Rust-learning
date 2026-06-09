use serde::{Deserialize, Serialize};

// Store whole structs: serialize to JSON bytes with serde (c49), insert, then
// deserialize on the way back. anyhow (c38) unifies serde + sled errors so one
// `?` handles both error types.
#[derive(Debug, Serialize, Deserialize)]
struct Service {
    name: String,
    price: u32,
}

fn put_service(db: &sled::Db, key: &str, svc: &Service) -> anyhow::Result<()> {
    let bytes = serde_json::to_vec(svc)?;
    db.insert(key, bytes)?;
    Ok(())
}

fn get_service(db: &sled::Db, key: &str) -> anyhow::Result<Option<Service>> {
    match db.get(key)? {
        Some(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
        None => Ok(None),
    }
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c71_example_sled_db")?;
    let svc = Service { name: "Gel Manicure".to_string(), price: 4500 };
    put_service(&db, "svc:gel", &svc)?;
    println!("{:?}", get_service(&db, "svc:gel")?);
    println!("{:?}", get_service(&db, "svc:missing")?);
    drop(db);
    std::fs::remove_dir_all("c71_example_sled_db").ok();
    Ok(())
}
