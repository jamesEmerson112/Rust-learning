// sled is a pure-Rust embedded key-value database. No SQL, no schema, no server:
// open a directory, then insert/get byte keys and values. Here we just write.
fn put(db: &sled::Db, key: &str, value: &str) -> sled::Result<()> {
    db.insert(key, value.as_bytes())?; // keys and values are bytes
    Ok(())
}

fn main() -> sled::Result<()> {
    let db = sled::open("c69_example_sled_db")?; // creates the DB directory on disk
    put(&db, "svc:gel", "4500")?;
    put(&db, "svc:pedicure", "3500")?;
    db.flush()?; // persist to disk
    println!("stored {} entries", db.len());
    println!("has svc:gel? {}", db.contains_key("svc:gel")?);

    drop(db); // close the DB before removing its files
    std::fs::remove_dir_all("c69_example_sled_db").ok();
    Ok(())
}
