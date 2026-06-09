pub fn get_value(db: &sled::Db, key: &str) -> sled::Result<Option<String>> {
    // TODO: db.get(key) returns Result<Option<IVec>>. Map the bytes to a String
    // (e.g. String::from_utf8_lossy(&v).to_string()). Return None when absent.
    let _ = (db, key);
    Ok(None)
}

fn main() -> sled::Result<()> {
    let db = sled::open("c70_exercise_sled_db")?;
    db.insert("k", "hello".as_bytes())?;
    println!("{:?}", get_value(&db, "k")?);
    drop(db);
    std::fs::remove_dir_all("c70_exercise_sled_db").ok();
    Ok(())
}
