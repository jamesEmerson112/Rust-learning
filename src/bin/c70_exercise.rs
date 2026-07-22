// THE VAULT RUN — Chapter 5: THE VAULT
// The fence wants proof before the meet. Pull shards back out by key —
// a missing key means the intel got burned in transit (that's a None, not a crash).

pub fn retrieve(db: &sled::Db, key: &str) -> sled::Result<Option<String>> {
    // TODO: db.get(key) returns Result<Option<IVec>>. Map the bytes to a String
    // (e.g. String::from_utf8_lossy(&v).to_string()). Return None when absent.
    let _ = (db, key);
    Ok(None)
}

fn main() -> sled::Result<()> {
    let db = sled::open("c70_exercise_vault")?;
    db.insert("vault:blueprints", "aegis-9 tower schematics".as_bytes())?;
    println!("[vault] blueprints -> {:?}", retrieve(&db, "vault:blueprints")?);
    println!("[vault] burned     -> {:?} (want None)", retrieve(&db, "vault:burned")?);
    drop(db);
    std::fs::remove_dir_all("c70_exercise_vault").ok();
    Ok(())
}
