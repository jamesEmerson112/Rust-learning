// THE VAULT RUN — Chapter 5: THE VAULT
// Raw byte shards are amateur hour. Structured intel — codename + grey-market
// value — encodes into the vault as JSON bytes and decodes back out intact.
// (The c74 capstone will import THIS file and reuse your codec. Build it well.)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Intel {
    pub codename: String,
    pub value: u32, // creds on the grey market
}

pub fn encode_intel(db: &sled::Db, intel: &Intel) -> anyhow::Result<()> {
    // TODO: Serialize the intel to JSON bytes with serde_json::to_vec, then
    // db.insert it under intel.codename.as_bytes(). anyhow's `?` unifies the
    // serde + sled error types.
    let _ = (db, intel);
    Ok(())
}

pub fn decode_intel(db: &sled::Db, codename: &str) -> anyhow::Result<Option<Intel>> {
    // TODO: db.get(codename)? gives Option<IVec>; deserialize the bytes with
    // serde_json::from_slice into an Intel. Return None when the key is absent.
    let _ = (db, codename);
    Ok(None)
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c71_exercise_vault")?;
    let jewel = Intel { codename: "GHOSTKEY".to_string(), value: 64000 };
    encode_intel(&db, &jewel)?;
    println!("[vault] GHOSTKEY -> {:?} (want Some(..))", decode_intel(&db, "GHOSTKEY")?);
    drop(db);
    std::fs::remove_dir_all("c71_exercise_vault").ok();
    Ok(())
}
