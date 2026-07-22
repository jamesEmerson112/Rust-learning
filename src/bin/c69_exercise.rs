// THE VAULT RUN — Chapter 5: THE VAULT
// You're inside. Every shard you rip goes into your OWN datavault on disk —
// if the uplink drops mid-run, the haul survives. sled: a HashMap that lives on disk.

pub fn stash(db: &sled::Db, key: &str, shard: &str) -> sled::Result<()> {
    // TODO: Insert key -> shard into the datavault. Keys and values are bytes,
    // so pass shard.as_bytes(). db.insert(...) returns a Result.
    let _ = (db, key, shard);
    Ok(())
}

fn main() -> sled::Result<()> {
    let db = sled::open("c69_exercise_vault")?;
    stash(&db, "vault:blueprints", "aegis-9 tower schematics")?;
    println!("[vault] shards stashed: {} (want 1)", db.len());
    drop(db);
    std::fs::remove_dir_all("c69_exercise_vault").ok();
    Ok(())
}
