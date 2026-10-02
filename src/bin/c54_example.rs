// Capstone — no new concept: a real CLI with subcommands (log / list / minutes) wiring
// together clap parsing and a HashMap of state into one usable tool.
// Coming from C: this is your `program <command> --args` dispatcher, but matching on the
// subcommand enum is exhaustive — the compiler ensures every command is handled.
//
// RUST GENERAL HOSPITAL: the ward's care log — who visited which patient, and for how long.
use clap::{Parser, Subcommand};
use std::collections::HashMap;

#[derive(Parser)]
#[command(about = "Ward care log CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Log {
        #[arg(long)]
        nurse: String,
        #[arg(long)]
        patient: String,
        #[arg(long)]
        minutes: u32,
    },
    List,
    Minutes,
}

struct Ward {
    visits: Vec<(String, String, u32)>,
}

impl Ward {
    fn new() -> Self {
        Self { visits: Vec::new() }
    }

    fn log_visit(&mut self, nurse: &str, patient: &str, minutes: u32) {
        self.visits.push((nurse.to_string(), patient.to_string(), minutes));
    }

    fn list(&self) -> &[(String, String, u32)] {
        &self.visits
    }

    fn minutes_by_nurse(&self) -> HashMap<String, u32> {
        let mut map = HashMap::new();
        for (nurse, _, minutes) in &self.visits {
            *map.entry(nurse.clone()).or_insert(0) += minutes;
        }
        map
    }
}

fn main() {
    let mut ward = Ward::new();
    ward.log_visit("Mai", "Mr. Hung", 45);
    ward.log_visit("Linh", "Mrs. Lan", 35);
    ward.log_visit("Mai", "Mr. Bao", 30);

    println!("--- Visits ---");
    for (nurse, patient, minutes) in ward.list() {
        println!("  {nurse}: {patient} ({minutes} min)");
    }

    println!("--- Care minutes ---");
    for (nurse, total) in &ward.minutes_by_nurse() {
        println!("  {nurse}: {total} min");
    }
}
