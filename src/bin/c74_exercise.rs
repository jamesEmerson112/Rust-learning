// RUST GENERAL HOSPITAL — Pharmacy — ★ CAPSTONE ★
// The supplier's delivery note is a CSV: drug,units. Import it through YOUR OWN c71
// codec — the module below is literally your c71_exercise.rs, imported. The
// capstone runs on the code you built. (If c71 is still unsolved, finish it first.)
#[allow(dead_code)]
#[path = "c71_exercise.rs"]
pub mod stock;

#[allow(unused_imports)]
use stock::{Stock, encode_stock};

pub fn import_delivery(db: &sled::Db, csv_path: &str) -> anyhow::Result<u32> {
    // TODO: Read the CSV at csv_path with csv::Reader::from_path; each row
    // deserializes straight into your c71 Stock (serde does the work). Store
    // every record with YOUR encode_stock, then iterate the store (db.iter())
    // and return the total units of everything in the pharmacy.
    let _ = (db, csv_path);
    Ok(0)
}

pub fn most_stocked(db: &sled::Db) -> anyhow::Result<Option<Stock>> {
    // TODO: Scan the pharmacy and return the drug with the most units on hand —
    // track the max by .units as you decode. None if the pharmacy is empty.
    let _ = db;
    Ok(None)
}

fn main() -> anyhow::Result<()> {
    std::fs::write(
        "c74_exercise_delivery.csv",
        "drug,units\nAmoxicillin,4200\nInsulin,1850\nParacetamol,6400\n",
    )?;
    let db = sled::open("c74_exercise_sled_db")?;
    let total = import_delivery(&db, "c74_exercise_delivery.csv")?;
    println!("[pharmacy] units on hand: {total} (want 12450)");
    println!("[pharmacy] best stocked: {:?} (want Paracetamol)", most_stocked(&db)?);
    if total == 12_450 {
        println!("╔══════════════════════════════════════════╗");
        println!("║  PHARMACY ONLINE                         ║");
        println!("║  Every delivery counted, every dose      ║");
        println!("║  accounted for. 🦀                       ║");
        println!("╚══════════════════════════════════════════╝");
    }
    drop(db);
    std::fs::remove_dir_all("c74_exercise_sled_db").ok();
    std::fs::remove_file("c74_exercise_delivery.csv").ok();
    Ok(())
}
