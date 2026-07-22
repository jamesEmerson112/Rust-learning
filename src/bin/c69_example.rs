// sled is a pure-Rust embedded key-value database — think a HashMap that lives on disk and
// survives restarts. No SQL, no schema, no server: open a directory, then insert/get byte
// keys and values. Coming from C: a B-tree-backed store you link into the binary, like
// Berkeley DB — no daemon to talk to.
//
// THE VAULT RUN — Chapter 5: THE VAULT. You're inside. Every data shard you rip from
// Aegis-9 goes straight into your own datavault — if the connection drops, the haul survives.
fn stash(db: &sled::Db, key: &str, shard: &str) -> sled::Result<()> {
    db.insert(key, shard.as_bytes())?; // keys and values are bytes
    Ok(())
}

fn main() -> sled::Result<()> {
    let db = sled::open("c69_example_vault")?; // creates the vault directory on disk
    stash(&db, "vault:blueprints", "aegis-9 tower schematics")?;
    stash(&db, "vault:payroll", "executive shell accounts")?;
    db.flush()?; // persist to disk — the haul survives a dropped uplink
    println!("[vault] {} shards stashed", db.len());
    println!("[vault] blueprints secured? {}", db.contains_key("vault:blueprints")?);

    drop(db); // close the vault before removing its files
    std::fs::remove_dir_all("c69_example_vault").ok();
    Ok(())
}
