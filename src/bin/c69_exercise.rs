pub fn put(db: &sled::Db, key: &str, value: &str) -> sled::Result<()> {
    // TODO: Insert key -> value into the sled database. Keys and values are
    // bytes, so pass value.as_bytes(). db.insert(...) returns a Result.
    let _ = (db, key, value);
    Ok(())
}

fn main() -> sled::Result<()> {
    let db = sled::open("c69_exercise_sled_db")?;
    put(&db, "svc:gel", "4500")?;
    println!("entries: {}", db.len());
    drop(db);
    std::fs::remove_dir_all("c69_exercise_sled_db").ok();
    Ok(())
}
