// Read values back out of sled by key. get() returns Option<IVec> (a smart
// pointer to bytes) — None when the key is absent, just like HashMap::get.
fn get_value(db: &sled::Db, key: &str) -> sled::Result<Option<String>> {
    Ok(db.get(key)?.map(|v| String::from_utf8_lossy(&v).to_string()))
}

fn main() -> sled::Result<()> {
    let db = sled::open("c70_example_sled_db")?;
    db.insert("svc:gel", "4500".as_bytes())?;
    println!("svc:gel     -> {:?}", get_value(&db, "svc:gel")?);
    println!("svc:missing -> {:?}", get_value(&db, "svc:missing")?);
    drop(db);
    std::fs::remove_dir_all("c70_example_sled_db").ok();
    Ok(())
}
