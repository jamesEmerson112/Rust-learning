// THE VAULT RUN — FINALE — ★ ASSEMBLY ★
// The exfiltrated dump is a CSV: codename,value. Import it through YOUR OWN c71
// codec — the module below is literally your c71_exercise.rs, imported. The
// capstone runs on the code you built. (If c71 is still unsolved, finish it first.)
#[allow(dead_code)]
#[path = "c71_exercise.rs"]
pub mod intel;

#[allow(unused_imports)]
use intel::{Intel, encode_intel};

pub fn import_dump(db: &sled::Db, csv_path: &str) -> anyhow::Result<u32> {
    // TODO: Read the CSV at csv_path with csv::Reader::from_path; each row
    // deserializes straight into your c71 Intel (serde does the work). Stash
    // every record with YOUR encode_intel, then iterate the store (db.iter())
    // and return the summed value of everything in the vault.
    let _ = (db, csv_path);
    Ok(0)
}

pub fn crown_jewel(db: &sled::Db) -> anyhow::Result<Option<Intel>> {
    // TODO: Scan the vault and return the single most valuable Intel —
    // track the max by .value as you decode. None if the vault is empty.
    let _ = db;
    Ok(None)
}

fn main() -> anyhow::Result<()> {
    std::fs::write(
        "c74_exercise_dump.csv",
        "codename,value\nBLACKOUT,42000\nEXEC-DIRT,18500\nGHOSTKEY,64000\n",
    )?;
    let db = sled::open("c74_exercise_vault")?;
    let haul = import_dump(&db, "c74_exercise_dump.csv")?;
    println!("[finale] haul: {haul} creds (want 124500)");
    println!("[finale] crown jewel: {:?} (want GHOSTKEY)", crown_jewel(&db)?);
    if haul == 124_500 {
        println!("╔══════════════════════════════════════════╗");
        println!("║  SYSTEM FULLY COMPROMISED                ║");
        println!("║  THE VAULT RUN IS COMPLETE.              ║");
        println!("║  Walk away clean, Chrome Surgeon. 🦀     ║");
        println!("╚══════════════════════════════════════════╝");
    }
    drop(db);
    std::fs::remove_dir_all("c74_exercise_vault").ok();
    std::fs::remove_file("c74_exercise_dump.csv").ok();
    Ok(())
}
