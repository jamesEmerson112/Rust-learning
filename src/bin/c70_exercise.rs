// RUST GENERAL HOSPITAL — Pharmacy
// A nurse looks a drug up before giving it. Read the record back out by its key. A drug the
// pharmacy does not stock should come back as None instead of crashing the program.

pub fn fetch(db: &sled::Db, key: &str) -> sled::Result<Option<String>> {
    // TODO: db.get(key) returns Result<Option<IVec>>. Convert the bytes to a String,
    // for example with String::from_utf8_lossy(&v).to_string(). Return None when the
    // key is absent.
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
