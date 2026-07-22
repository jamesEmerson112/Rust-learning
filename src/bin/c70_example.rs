// Read values back out of sled by key. get() returns Option<IVec> (a smart
// pointer to bytes) — None when the key is absent, just like HashMap::get.
//
// THE VAULT RUN: the fence wants proof before the meet. Pull shards back out of
// the datavault by key — a missing key means the intel got burned in transit.
fn retrieve(db: &sled::Db, key: &str) -> sled::Result<Option<String>> {
    Ok(db.get(key)?.map(|v| String::from_utf8_lossy(&v).to_string()))
}

fn main() -> sled::Result<()> {
    let db = sled::open("c70_example_vault")?;
    db.insert("vault:blueprints", "aegis-9 tower schematics".as_bytes())?;
    println!("[vault] blueprints -> {:?}", retrieve(&db, "vault:blueprints")?);
    println!("[vault] burned     -> {:?}", retrieve(&db, "vault:burned")?);
    drop(db);
    std::fs::remove_dir_all("c70_example_vault").ok();
    Ok(())
}
