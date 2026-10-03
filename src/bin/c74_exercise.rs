// RUST GENERAL HOSPITAL — Pharmacy — ★ CAPSTONE ★
// The supplier's delivery note is a CSV file with the columns drug,units. Import it with the
// encode and decode functions you wrote in c71. The module below is your own c71_exercise.rs,
// so this capstone runs on the code you built. If c71 is still unsolved, finish it first.
#[allow(dead_code)]
#[path = "c71_exercise.rs"]
pub mod stock;

#[allow(unused_imports)]
use stock::{Stock, encode_stock};

pub fn import_delivery(db: &sled::Db, csv_path: &str) -> anyhow::Result<u32> {
    // TODO: Read the CSV at csv_path with csv::Reader::from_path. serde deserializes each
    // row straight into your c71 Stock. Store every record with your own encode_stock,
    // then loop over the store with db.iter() and return the total units of everything
    // in the pharmacy.
    let _ = (db, csv_path);
    Ok(0)
}

pub fn most_stocked(db: &sled::Db) -> anyhow::Result<Option<Stock>> {
    // TODO: Scan the pharmacy and return the drug with the most units on hand. Track the
    // largest .units value as you decode. Return None if the pharmacy is empty.
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
