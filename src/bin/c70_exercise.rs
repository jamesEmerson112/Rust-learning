// RUST GENERAL HOSPITAL — Pharmacy
// A nurse looks a drug up before giving it. Pull the record back out by key —
// a drug the pharmacy doesn't stock comes back as None, not a crash.

pub fn fetch(db: &sled::Db, key: &str) -> sled::Result<Option<String>> {
    // TODO: db.get(key) returns Result<Option<IVec>>. Map the bytes to a String
    // (e.g. String::from_utf8_lossy(&v).to_string()). Return None when absent.
    let _ = (db, key);
    Ok(None)
}

fn main() -> sled::Result<()> {
    let db = sled::open("c70_exercise_sled_db")?;
    db.insert("stock:paracetamol", "500 mg tablets, shelf A3".as_bytes())?;
    println!("[pharmacy] paracetamol -> {:?}", fetch(&db, "stock:paracetamol")?);
    println!("[pharmacy] unknown     -> {:?} (want None)", fetch(&db, "stock:unknown")?);
    drop(db);
    std::fs::remove_dir_all("c70_exercise_sled_db").ok();
    Ok(())
}
