use serde::{Deserialize, Serialize};

// Store whole structs: serialize to JSON bytes with serde (c49), insert, then
// deserialize on the way back. anyhow (c38) unifies serde + sled errors so one
// `?` handles both error types.
//
// THE VAULT RUN: raw shards are amateur hour. Structured intel — codename plus
// grey-market value — encodes into the vault and decodes back out intact.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Intel {
    codename: String,
    value: u32, // creds on the grey market
}

fn encode_intel(db: &sled::Db, intel: &Intel) -> anyhow::Result<()> {
    let bytes = serde_json::to_vec(intel)?;
    db.insert(intel.codename.as_bytes(), bytes)?;
    Ok(())
}

fn decode_intel(db: &sled::Db, codename: &str) -> anyhow::Result<Option<Intel>> {
    match db.get(codename)? {
        Some(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
        None => Ok(None),
    }
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c71_example_vault")?;
    let jewel = Intel { codename: "GHOSTKEY".to_string(), value: 64000 };
    encode_intel(&db, &jewel)?;
    println!("[vault] GHOSTKEY -> {:?}", decode_intel(&db, "GHOSTKEY")?);
    println!("[vault] BURNED   -> {:?}", decode_intel(&db, "BURNED")?);
    drop(db);
    std::fs::remove_dir_all("c71_example_vault").ok();
    Ok(())
}
