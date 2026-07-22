// THE VAULT RUN — Chapter 5: THE VAULT
// The fence only moves premium goods. Shortlist intel above the value threshold —
// no SQL, just filter/map over your own store.
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Intel {
    pub codename: String,
    pub value: u32,
}

pub fn shortlist(db: &sled::Db, min_value: u32) -> anyhow::Result<Vec<Intel>> {
    // TODO: Load every Intel (iterate db.iter(), from_slice each value), then
    // .filter(|i| i.value >= min_value).collect(). Sort by codename.
    let _ = (db, min_value);
    Ok(Vec::new())
}

pub fn shortlist_codenames(db: &sled::Db, min_value: u32) -> anyhow::Result<Vec<String>> {
    // TODO: Same filter as shortlist, then .map(|i| i.codename).collect() to
    // project just the codenames. Sort them.
    let _ = (db, min_value);
    Ok(Vec::new())
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c73_exercise_vault")?;
    let intel = Intel { codename: "GHOSTKEY".to_string(), value: 64000 };
    db.insert("GHOSTKEY", serde_json::to_vec(&intel)?)?;
    println!("[fence] premium: {:?} (want [\"GHOSTKEY\"])", shortlist_codenames(&db, 40000)?);
    drop(db);
    std::fs::remove_dir_all("c73_exercise_vault").ok();
    Ok(())
}
