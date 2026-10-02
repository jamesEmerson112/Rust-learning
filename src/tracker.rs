// Shared data model for the RPG progress tracker and the HTML dashboard.
//
// Included by BOTH binaries via `#[path = "../tracker.rs"] mod tracker;` — the repo
// has no lib.rs on purpose (see CLAUDE.md), so this follows the same `#[path]`
// convention used by src/lesson32, the test suites, and c74 -> c71.
//
// `progress` owns every write to the save file. `dashboard` is strictly read-only.
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ─── Save File (v3) ────────────────────────────────────────────────────────

pub const SAVE_VERSION: u32 = 3;

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveFile {
    pub version: u32,
    pub character: Character,
    pub lessons: HashMap<String, LessonStatus>,
    pub last_scan: u64,
    /// Append-only. Never cleared — not even by `--rescan`.
    #[serde(default)]
    pub history: Vec<Event>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Character {
    pub name: String,
    pub class: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct LessonStatus {
    pub passed: bool,
    pub completed_at: Option<u64>,
    /// First time this lesson ever went green. Survives `--rescan`.
    #[serde(default)]
    pub first_passed_at: Option<u64>,
    /// Failed scans observed in incremental mode (a "how hard was this for me" signal).
    #[serde(default)]
    pub attempts: u32,
    /// True for entries migrated from a v1 save, whose timestamps all collapsed onto
    /// one rescan minute. Counted for level/XP/stats, excluded from streaks and pace.
    #[serde(default)]
    pub backfilled: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    Pass,
    Fail,
    Regression,
    Scan,
    RankUp,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Event {
    pub at: u64,
    pub kind: EventKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lesson: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl Event {
    pub fn new(kind: EventKind, lesson: Option<u32>) -> Self {
        Event { at: now_epoch(), kind, lesson, detail: None }
    }

    pub fn with_detail(kind: EventKind, detail: &str) -> Self {
        Event { at: now_epoch(), kind, lesson: None, detail: Some(detail.to_string()) }
    }
}

/// The v2 class names, in the same order as `CLASSES` — index i was renamed to CLASSES[i].
const V2_CLASSES: [&str; 4] = ["Chrome Surgeon", "Data Wraith", "Borrow Monk", "Codec Alchemist"];

/// Upgrades an older save in place, one version step at a time. Returns true if anything
/// changed (caller decides whether to persist).
///
/// v1 -> v2: v1 stored only `{passed, completed_at}`, and `--rescan` rewrote every
/// timestamp, so existing timestamps are all within seconds of one bulk rescan. We keep
/// them as the first-pass time but flag them `backfilled` so history-based panels can
/// ignore them instead of rendering a fake "45 lessons in one day" spike.
///
/// v2 -> v3: the tracker moved to the Rust General Hospital theme, so the four character
/// classes were renamed. A class that matches none of the old names is left unchanged.
pub fn migrate(save: &mut SaveFile) -> bool {
    if save.version >= SAVE_VERSION {
        return false;
    }
    // Each step runs only for saves older than it, so a v2 save is never re-backfilled.
    if save.version < 2 {
        for status in save.lessons.values_mut() {
            if status.passed {
                if status.first_passed_at.is_none() {
                    status.first_passed_at = status.completed_at;
                }
                if status.attempts == 0 {
                    status.attempts = 1;
                }
                status.backfilled = true;
            }
        }
    }
    if save.version < 3 {
        if let Some(i) = V2_CLASSES.iter().position(|old| *old == save.character.class) {
            save.character.class = CLASSES[i].to_string();
        }
    }
    save.version = SAVE_VERSION;
    true
}

// ─── Save File I/O ─────────────────────────────────────────────────────────

pub fn save_path() -> std::path::PathBuf {
    std::path::PathBuf::from(".rustacean_save.json")
}

pub fn load_save() -> Option<SaveFile> {
    let data = std::fs::read_to_string(save_path()).ok()?;
    serde_json::from_str(&data).ok()
}

pub fn write_save(save: &SaveFile) {
    let json = serde_json::to_string_pretty(save).expect("Failed to serialize save file");
    std::fs::write(save_path(), json).expect("Failed to write save file");
}

// ─── Lesson Metadata ───────────────────────────────────────────────────────

pub struct LessonMeta {
    pub number: u32,
    pub title: &'static str,
    pub stat_group: &'static str,
}

pub const NUM_LESSONS: u32 = LESSONS.len() as u32;
pub const MAX_XP: u32 = NUM_LESSONS * 100;

#[rustfmt::skip]
pub const LESSONS: [LessonMeta; 80] = [
    LessonMeta { number: 1,  title: "Hello Variables",                      stat_group: "First Aid" },
    LessonMeta { number: 2,  title: "Strings and Formatting",              stat_group: "First Aid" },
    LessonMeta { number: 3,  title: "Arrays and Iteration",                stat_group: "Vital Signs" },
    LessonMeta { number: 4,  title: "Tuples and Type Casting",             stat_group: "Vital Signs" },
    LessonMeta { number: 5,  title: "If/Else and For Loops",               stat_group: "Triage" },
    LessonMeta { number: 6,  title: "Match and String Building",           stat_group: "Triage" },
    LessonMeta { number: 7,  title: "Ownership and Borrowing",             stat_group: "Chain of Custody" },
    LessonMeta { number: 8,  title: "String Slices and Methods",           stat_group: "Chain of Custody" },
    LessonMeta { number: 9,  title: "Structs",                             stat_group: "Anatomy" },
    LessonMeta { number: 10, title: "Methods and impl Blocks",             stat_group: "Anatomy" },
    LessonMeta { number: 11, title: "Enums",                               stat_group: "Diagnosis" },
    LessonMeta { number: 12, title: "Option<T>",                           stat_group: "Diagnosis" },
    LessonMeta { number: 13, title: "HashMap Basics",                      stat_group: "Patient Index" },
    LessonMeta { number: 14, title: "HashMap Entry API",                   stat_group: "Patient Index" },
    LessonMeta { number: 15, title: "String Normalization",                stat_group: "Medical Records" },
    LessonMeta { number: 16, title: "Word Count (HashMap + Strings)",      stat_group: "Medical Records" },
    LessonMeta { number: 17, title: "Result Basics",                       stat_group: "Safety Checks" },
    LessonMeta { number: 18, title: "The ? Operator",                      stat_group: "Safety Checks" },
    LessonMeta { number: 19, title: "Error Chaining with map_err",         stat_group: "Incident Reports" },
    LessonMeta { number: 20, title: "Closures",                            stat_group: "Incident Reports" },
    LessonMeta { number: 21, title: "Map + Collect",                       stat_group: "Ward Rounds" },
    LessonMeta { number: 22, title: "Filter",                              stat_group: "Ward Rounds" },
    LessonMeta { number: 23, title: "Sum",                                 stat_group: "Ward Rounds" },
    LessonMeta { number: 24, title: "Fold",                                stat_group: "Ward Rounds" },
    LessonMeta { number: 25, title: "Debug Format",                        stat_group: "Examination" },
    LessonMeta { number: 26, title: "Traits",                              stat_group: "Clinical Protocols" },
    LessonMeta { number: 27, title: "Generics",                            stat_group: "Clinical Protocols" },
    LessonMeta { number: 28, title: "Trait Bounds",                        stat_group: "Scope of Practice" },
    LessonMeta { number: 29, title: "Lifetimes Intro",                     stat_group: "Scope of Practice" },
    LessonMeta { number: 30, title: "Box<T> — Heap Allocation",            stat_group: "Referrals" },
    LessonMeta { number: 31, title: "Rc<T> — Shared Ownership",            stat_group: "Referrals" },
    LessonMeta { number: 32, title: "Modules and Visibility",              stat_group: "Departments" },
    LessonMeta { number: 33, title: "Capstone: Service Log",               stat_group: "Departments" },
    LessonMeta { number: 34, title: "Custom Error Enum",                   stat_group: "Risk Management" },
    LessonMeta { number: 35, title: "impl Display for Errors",             stat_group: "Risk Management" },
    LessonMeta { number: 36, title: "impl Error Trait",                    stat_group: "Risk Management" },
    LessonMeta { number: 37, title: "thiserror Derive",                    stat_group: "Risk Management" },
    LessonMeta { number: 38, title: "anyhow Catch-All",                    stat_group: "Risk Management" },
    LessonMeta { number: 39, title: "Slices &[T]",                         stat_group: "Pathology" },
    LessonMeta { number: 40, title: "Struct Lifetimes",                    stat_group: "Pathology" },
    LessonMeta { number: 41, title: "Lifetime Elision",                    stat_group: "Pathology" },
    LessonMeta { number: 42, title: "Cell<T>",                             stat_group: "Care Notes" },
    LessonMeta { number: 43, title: "RefCell<T>",                          stat_group: "Care Notes" },
    LessonMeta { number: 44, title: "Rc<RefCell<T>>",                      stat_group: "Care Notes" },
    LessonMeta { number: 45, title: "Custom Iterator",                     stat_group: "Patient Queue" },
    LessonMeta { number: 46, title: "Iterator Adaptors",                   stat_group: "Patient Queue" },
    LessonMeta { number: 47, title: "CSV Read",                            stat_group: "Lab Reports" },
    LessonMeta { number: 48, title: "CSV Write",                           stat_group: "Lab Reports" },
    LessonMeta { number: 49, title: "Serde JSON",                          stat_group: "Lab Reports" },
    LessonMeta { number: 50, title: "async fn + tokio",                    stat_group: "Scheduling" },
    LessonMeta { number: 51, title: "tokio::spawn",                        stat_group: "Scheduling" },
    LessonMeta { number: 52, title: "Async Channels",                      stat_group: "Scheduling" },
    LessonMeta { number: 53, title: "clap Arg Parsing",                    stat_group: "Ward Terminal" },
    LessonMeta { number: 54, title: "Capstone: Ward CLI",                  stat_group: "Ward Terminal" },
    LessonMeta { number: 55, title: "Warmup: Climbing Stairs",             stat_group: "Robot Drills" },
    LessonMeta { number: 56, title: "Recursive Types with Box",            stat_group: "Ward Equipment" },
    LessonMeta { number: 57, title: "Trait Objects (Box<dyn>)",            stat_group: "Ward Equipment" },
    LessonMeta { number: 58, title: "Deref — Custom Smart Pointer",        stat_group: "Ward Equipment" },
    LessonMeta { number: 59, title: "Warmup: Two Sum",                     stat_group: "Robot Drills" },
    LessonMeta { number: 60, title: "Drop (RAII)",                         stat_group: "Safe Shutdown" },
    LessonMeta { number: 61, title: "Weak<T> and Cycles",                  stat_group: "Safe Shutdown" },
    LessonMeta { number: 62, title: "Warmup: Reverse a Vec",              stat_group: "Robot Drills" },
    LessonMeta { number: 63, title: "Cell<T> Full API",                    stat_group: "Shared Care" },
    LessonMeta { number: 64, title: "RefCell Runtime Borrow",              stat_group: "Shared Care" },
    LessonMeta { number: 65, title: "Warmup: Contains Duplicate",          stat_group: "Robot Drills" },
    LessonMeta { number: 66, title: "Arc<T> Across Threads",               stat_group: "Night Shift" },
    LessonMeta { number: 67, title: "Arc<Mutex<T>>",                       stat_group: "Night Shift" },
    LessonMeta { number: 68, title: "RwLock<T>",                           stat_group: "Night Shift" },
    LessonMeta { number: 69, title: "Sled Insert",                         stat_group: "Pharmacy" },
    LessonMeta { number: 70, title: "Sled Get",                            stat_group: "Pharmacy" },
    LessonMeta { number: 71, title: "Sled + Serde",                        stat_group: "Pharmacy" },
    LessonMeta { number: 72, title: "Sled Iterate",                        stat_group: "Pharmacy" },
    LessonMeta { number: 73, title: "Sled Query",                          stat_group: "Pharmacy" },
    LessonMeta { number: 74, title: "CSV to Sled",                         stat_group: "Pharmacy" },
    LessonMeta { number: 75, title: "Bug Hunt: The Vanishing Tally",       stat_group: "Bug Hunt" },
    LessonMeta { number: 76, title: "Bug Hunt: The Silent Zero",           stat_group: "Bug Hunt" },
    LessonMeta { number: 77, title: "Bug Hunt: The Missing Critical Beds", stat_group: "Bug Hunt" },
    LessonMeta { number: 78, title: "Bug Hunt: The Overlooked Rush",       stat_group: "Bug Hunt" },
    LessonMeta { number: 79, title: "Bug Hunt: The Double-Booked Borrow",  stat_group: "Bug Hunt" },
    LessonMeta { number: 80, title: "Bug Hunt: The Half-Heard Clock-Out",  stat_group: "Bug Hunt" },
];

pub const STAT_GROUPS: [&str; 30] = [
    "First Aid",
    "Vital Signs",
    "Triage",
    "Chain of Custody",
    "Anatomy",
    "Diagnosis",
    "Patient Index",
    "Medical Records",
    "Safety Checks",
    "Incident Reports",
    "Ward Rounds",
    "Examination",
    "Clinical Protocols",
    "Scope of Practice",
    "Referrals",
    "Departments",
    "Risk Management",
    "Pathology",
    "Care Notes",
    "Patient Queue",
    "Lab Reports",
    "Scheduling",
    "Ward Terminal",
    "Robot Drills",
    "Ward Equipment",
    "Safe Shutdown",
    "Shared Care",
    "Night Shift",
    "Pharmacy",
    "Bug Hunt",
];

// ─── Chapters (story arcs) ─────────────────────────────────────────────────

pub struct Chapter {
    pub name: &'static str,
    pub subtitle: &'static str,
    pub first: u32,
    pub last: u32,
}

/// The curriculum's narrative arc, per ROADMAP.md. Eight axes — the right size for a
/// radar chart, where the 30 STAT_GROUPS would be an unreadable 30-spoke mess.
#[rustfmt::skip]
pub const CHAPTERS: [Chapter; 8] = [
    Chapter { name: "Foundations",    subtitle: "core Rust",              first: 1,  last: 33 },
    Chapter { name: "Practical Rust", subtitle: "errors, I/O, async",     first: 34, last: 54 },
    Chapter { name: "Ward Equipment", subtitle: "Box & trait objects",    first: 55, last: 58 },
    Chapter { name: "Safe Shutdown",  subtitle: "Drop & Weak",            first: 59, last: 61 },
    Chapter { name: "Shared Care",    subtitle: "Cell & RefCell",         first: 62, last: 64 },
    Chapter { name: "Night Shift",    subtitle: "Arc, Mutex, RwLock",     first: 65, last: 68 },
    Chapter { name: "Pharmacy",       subtitle: "sled + serde",           first: 69, last: 74 },
    Chapter { name: "Bug Hunt",       subtitle: "hospital side jobs",     first: 75, last: 80 },
];

// ─── Bosses (capstones) ────────────────────────────────────────────────────

pub struct Boss {
    /// The capstone lesson that ends the fight.
    pub lesson: u32,
    pub name: &'static str,
    pub tagline: &'static str,
    /// The block this boss guards — its unpassed lessons are the boss's HP.
    pub guards_first: u32,
    pub guards_last: u32,
}

#[rustfmt::skip]
pub const BOSSES: [Boss; 4] = [
    Boss { lesson: 33, name: "THE SERVICE LOG", tagline: "Modules, structs, and your first full system.", guards_first: 1,  guards_last: 33 },
    Boss { lesson: 54, name: "THE WARD CLI",    tagline: "Errors, files, async — and a binary that ships.", guards_first: 34, guards_last: 54 },
    Boss { lesson: 74, name: "THE PHARMACY",    tagline: "The stockroom runs on your own c71 codec.",       guards_first: 55, guards_last: 74 },
    Boss { lesson: 80, name: "THE LAST BUG",    tagline: "Six planted defects. No new concepts. Just you.", guards_first: 75, guards_last: 80 },
];

/// ★ BUG HUNT lessons — ship compiling-but-wrong code with a `// BUG:` symptom comment.
pub const BUG_LESSONS: [u32; 10] = [58, 61, 64, 67, 75, 76, 77, 78, 79, 80];

/// Lessons in c01–c54 the learner deliberately left open, as of the 2026-07-21 rescan.
pub const HOMEWORK_GAPS: [u32; 9] = [16, 19, 35, 37, 38, 40, 48, 49, 53];

// ─── Abilities ─────────────────────────────────────────────────────────────

pub struct Ability {
    pub name: &'static str,
    pub description: &'static str,
    pub lesson: u32,
}

#[rustfmt::skip]
pub const ABILITIES: [Ability; 80] = [
    // Volunteer (L1-4)
    Ability { name: "Bedside Basics",          description: "Bind values with `let` and `mut`.",                                  lesson: 1  },
    Ability { name: "Clear Notes",             description: "Build and format strings with `format!` and `println!`.",            lesson: 2  },
    Ability { name: "Head Count",              description: "Iterate a fixed-size array and total it with `.sum()`.",             lesson: 3  },
    Ability { name: "Paired Readings",         description: "Return several values as a tuple and cast between numeric types.",   lesson: 4  },
    // Pre-Med Student (L5-8)
    Ability { name: "Decision Path",           description: "Route execution through `if`/`else` branches and `for` loops.",      lesson: 5  },
    Ability { name: "Case Sorting",            description: "Match values against patterns and build strings.",                   lesson: 6  },
    Ability { name: "Single Custody",          description: "Give every value exactly one owner, and lend it out with `&`.",      lesson: 7  },
    Ability { name: "Quick Glance",            description: "Read part of a string through a slice, without copying.",            lesson: 8  },
    // Medical Student (L9-12)
    Ability { name: "Patient Record",          description: "Define structs with named fields.",                                  lesson: 9  },
    Ability { name: "Standard Procedure",      description: "Attach methods to a type with `impl` blocks.",                       lesson: 10 },
    Ability { name: "Differential",            description: "Define enums whose variants carry different data.",                  lesson: 11 },
    Ability { name: "Nothing Assumed",         description: "Handle missing values safely with `Option<T>` — no null crashes.",   lesson: 12 },
    // Clinical Student (L13-16)
    Ability { name: "Index Lookup",            description: "Store and query key-value data in a `HashMap` at O(1).",             lesson: 13 },
    Ability { name: "Running Tally",           description: "Insert-or-update in a single lookup with the entry API.",            lesson: 14 },
    Ability { name: "Clean Input",             description: "Normalize messy text — trim punctuation, lowercase, prep for lookup.", lesson: 15 },
    Ability { name: "Word Census",             description: "Combine HashMap + string normalization into a frequency count.",     lesson: 16 },
    // Intern (L17-20)
    Ability { name: "Report Failure",          description: "Return success or failure with `Result<T, E>`.",                     lesson: 17 },
    Ability { name: "Escalate",                description: "Pass errors up the chain with `?` without losing context.",          lesson: 18 },
    Ability { name: "Translate Errors",        description: "Convert one error type into another with `map_err`.",                lesson: 19 },
    Ability { name: "Portable Procedure",      description: "Pass closures around as values that capture their surroundings.",    lesson: 20 },
    // Junior Resident (L21-24)
    Ability { name: "Map the Ward",            description: "Transform every element with `.map()` and `.collect()` into a Vec.", lesson: 21 },
    Ability { name: "Screening",               description: "Select elements with `.filter()` — and handle the double reference.", lesson: 22 },
    Ability { name: "Totals",                  description: "Collapse an iterator into one value with `.sum()`.",                 lesson: 23 },
    Ability { name: "Fold Summary",            description: "Reduce any sequence with `.fold(init, |acc, x| ...)`.",              lesson: 24 },
    // Resident (L25-29)
    Ability { name: "Close Inspection",        description: "Inspect any `#[derive(Debug)]` type with `{:?}` formatting.",        lesson: 25 },
    Ability { name: "Shared Protocol",         description: "Define behavior that many different types promise to provide.",      lesson: 26 },
    Ability { name: "General Practice",        description: "Write code that works for any type with generics.",                  lesson: 27 },
    Ability { name: "Qualified Only",          description: "Constrain generics with trait bounds to unlock comparison.",         lesson: 28 },
    Ability { name: "Time Limits",             description: "Annotate `'a` so references never outlive the data they point to.", lesson: 29 },
    // Senior Resident (L30-33)
    Ability { name: "Heap Storage",            description: "Put values on the heap with `Box` and size recursive types.",        lesson: 30 },
    Ability { name: "Shared Custody",          description: "Share one allocation among owners with `Rc` reference counting.",    lesson: 31 },
    Ability { name: "Department Layout",       description: "Organize code into modules with controlled visibility.",             lesson: 32 },
    Ability { name: "Full Service Log",        description: "Every core system working together in one program.",                 lesson: 33 },
    // Chief Resident (L34-38)
    Ability { name: "Error Catalog",           description: "Define custom error enums with distinct failure variants.",          lesson: 34 },
    Ability { name: "Plain Language",          description: "Implement `Display` so errors print readable messages.",             lesson: 35 },
    Ability { name: "Standard Reporting",      description: "Implement `std::error::Error` so errors compose.",                   lesson: 36 },
    Ability { name: "Less Paperwork",          description: "Derive error boilerplate away with `thiserror`.",                    lesson: 37 },
    Ability { name: "Catch-All Intake",        description: "Accept any error type with `anyhow::Result` — zero boilerplate.",   lesson: 38 },
    // Fellow (L39-44)
    Ability { name: "Section View",            description: "Work with contiguous views of data, without allocation.",            lesson: 39 },
    Ability { name: "Borrowed Records",        description: "Hold borrowed references inside structs with lifetime parameters.", lesson: 40 },
    Ability { name: "Elision Reading",         description: "Read the compiler's lifetime elision rules fluently.",              lesson: 41 },
    Ability { name: "Cell Update",             description: "Change values through shared references with `Cell<T>`.",          lesson: 42 },
    Ability { name: "Checked Access",          description: "Borrow-check at runtime with `RefCell<T>` for complex interiors.",  lesson: 43 },
    Ability { name: "Shared Chart",            description: "Combine `Rc` and `RefCell` for shared mutable state.",              lesson: 44 },
    // Attending Physician (L45-49)
    Ability { name: "Custom Queue",            description: "Implement the `Iterator` trait on your own struct.",                 lesson: 45 },
    Ability { name: "Queue Screening",         description: "Chain filter and map on a custom iterator — a full pipeline.",       lesson: 46 },
    Ability { name: "CSV Intake",              description: "Parse a CSV file into structs with `csv` + `serde`.",               lesson: 47 },
    Ability { name: "CSV Export",              description: "Write structs to a CSV file with `csv` + `serde`.",                 lesson: 48 },
    Ability { name: "JSON Records",            description: "Serialize and deserialize structs as JSON.",                         lesson: 49 },
    // Senior Attending (L50-54)
    Ability { name: "Async Check",             description: "Write your first `async fn` and `.await` it.",                      lesson: 50 },
    Ability { name: "Parallel Booking",        description: "Run concurrent work with `tokio::spawn`.",                          lesson: 51 },
    Ability { name: "Station Relay",           description: "Send messages between async tasks with `mpsc`.",                    lesson: 52 },
    Ability { name: "Order Entry",             description: "Parse command-line arguments with `clap` derive macros.",           lesson: 53 },
    Ability { name: "Ward Manager",            description: "A full ward care-log CLI. Every system integrated. You ship software.", lesson: 54 },
    // Specialist + Consultant: robot warmups and the smart-pointer deep dive (L55-65)
    Ability { name: "Stair Planner",           description: "Count stair-climbing patterns iteratively — the Fibonacci warmup.",  lesson: 55 },
    Ability { name: "Route Planner",           description: "Box a self-referential type so the compiler can size it.",           lesson: 56 },
    Ability { name: "Monitor Rack",            description: "Store mixed types behind `Box<dyn Trait>` and call through the vtable.", lesson: 57 },
    Ability { name: "Pump Reader",             description: "Implement `Deref` so a wrapper reads like the value inside it.",     lesson: 58 },
    Ability { name: "Trip Pairing",            description: "Two Sum in one pass with a HashMap.",                                 lesson: 59 },
    Ability { name: "Safe Stop",               description: "Hook `Drop` for cleanup that runs on every exit path.",               lesson: 60 },
    Ability { name: "Alarm Release",           description: "Break reference cycles with `Weak<T>` so nothing outlives its owner.", lesson: 61 },
    Ability { name: "Retrace Path",            description: "Reverse a Vec in place with two pointers.",                           lesson: 62 },
    Ability { name: "Crash Cart",              description: "Swap and take state through `&self` with the full Cell API.",        lesson: 63 },
    Ability { name: "Chart Guard",             description: "Survive RefCell's runtime borrow check with `try_borrow_mut`.",       lesson: 64 },
    Ability { name: "Double-Dose Check",       description: "Detect duplicates in one pass with a HashSet.",                       lesson: 65 },
    // Department Head (L66-68)
    Ability { name: "Shared Supply List",      description: "Share read-only data across threads with `Arc<T>`.",                  lesson: 66 },
    Ability { name: "Fluid Balance",           description: "Guard shared mutable state across threads with `Arc<Mutex<T>>`.",     lesson: 67 },
    Ability { name: "Bed Board",               description: "Many readers, one writer — concurrent access with `RwLock<T>`.",      lesson: 68 },
    // Medical Director (L69-74)
    Ability { name: "Stock Entry",             description: "Open a sled database and insert key/value bytes.",                    lesson: 69 },
    Ability { name: "Stock Lookup",            description: "Read values back out of sled by key.",                                lesson: 70 },
    Ability { name: "Stock Codec",             description: "Store and load structs in sled with serde.",                          lesson: 71 },
    Ability { name: "Stock Take",              description: "Iterate every entry in the database.",                                lesson: 72 },
    Ability { name: "Reorder List",            description: "Filter and project records with iterators, no SQL.",                  lesson: 73 },
    Ability { name: "Delivery Intake",         description: "Load a CSV into sled and report on it.",                              lesson: 74 },
    // Bug Hunt (L75-80) — hospital side jobs: debug broken code, don't write from scratch
    Ability { name: "Visit Tally",             description: "Fix the HashMap clobber — `entry().or_insert()` accumulates, `insert` resets.", lesson: 75 },
    Ability { name: "Fault Surfacing",         description: "Un-swallow parse errors — a corrupt row must `Err`, not zero out.",   lesson: 76 },
    Ability { name: "Predicate Flip",          description: "Right an inverted `filter` — keep what the predicate says.",          lesson: 77 },
    Ability { name: "Late Rush",               description: "Fix the off-by-one — the last window starts at `n - width`.",         lesson: 78 },
    Ability { name: "Borrow Unlock",           description: "Release the read guard before writing — RefCell forgives nothing.",   lesson: 79 },
    Ability { name: "Channel Drain",           description: "Drain an mpsc with `while let` — a single `recv` hears one voice.",   lesson: 80 },
];

// ─── Ranks ─────────────────────────────────────────────────────────────────

pub struct Rank {
    pub min_level: u32,
    pub name: &'static str,
    pub quote: &'static str,
}

#[rustfmt::skip]
pub const RANKS: [Rank; 19] = [
    Rank { min_level: 0,  name: "Visitor",             quote: "Sign in at the front desk to begin." },
    Rank { min_level: 1,  name: "Volunteer",           quote: "Your first shift is done. The ward knows your name." },
    Rank { min_level: 5,  name: "Pre-Med Student",     quote: "Every value has exactly one owner, like every patient has one attending." },
    Rank { min_level: 9,  name: "Medical Student",     quote: "You model the world in structs and enums, and never assume a value is there." },
    Rank { min_level: 13, name: "Clinical Student",    quote: "Lookups are instant, and messy text is cleaned before it reaches the chart." },
    Rank { min_level: 17, name: "Intern",              quote: "Failures are reported, not hidden. Every error has a path." },
    Rank { min_level: 21, name: "Junior Resident",     quote: "You make your rounds with map, filter, and fold — no index juggling." },
    Rank { min_level: 25, name: "Resident",            quote: "Traits, generics, and lifetimes follow your protocols." },
    Rank { min_level: 30, name: "Senior Resident",     quote: "Heap and stack are yours. Pointers and modules follow your orders." },
    Rank { min_level: 34, name: "Chief Resident",      quote: "Errors are data. You design the failure paths." },
    Rank { min_level: 40, name: "Fellow",              quote: "Slices, lifetimes, interior mutability — you work with precision." },
    Rank { min_level: 45, name: "Attending Physician", quote: "Custom iterators and file pipelines keep the records moving." },
    Rank { min_level: 50, name: "Senior Attending",    quote: "Concurrent tasks run on your schedule. Nothing blocks the ward." },
    Rank { min_level: 55, name: "Specialist",          quote: "Box, trait objects, and Deref — the ward's equipment answers to you." },
    Rank { min_level: 60, name: "Consultant",          quote: "Cleanup always runs, and no cycle keeps a record alive past its time." },
    Rank { min_level: 66, name: "Department Head",     quote: "Many hands, one chart. Arc and locks keep every entry straight." },
    Rank { min_level: 69, name: "Medical Director",    quote: "Records persist on disk and come back intact — no query language required." },
    Rank { min_level: 74, name: "Chief of Staff",      quote: "The pharmacy runs on your own code. 🦀" },
    Rank { min_level: 80, name: "Chief of Medicine",   quote: "Every bug on the ward is found and fixed. Nothing broken gets past you." },
];

/// Character classes, in the same order as `V2_CLASSES` (the save migration relies on it).
pub const CLASSES: [&str; 4] = [
    "Surgeon",
    "Neurologist",
    "Emergency Physician",
    "Pharmacist",
];

// ─── Helpers ───────────────────────────────────────────────────────────────

pub fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn lesson_key(n: u32) -> String {
    format!("c{n:02}")
}

pub fn lessons_passed(save: &SaveFile) -> u32 {
    save.lessons.values().filter(|s| s.passed).count() as u32
}

pub fn get_rank(level: u32) -> &'static Rank {
    RANKS.iter().rev().find(|r| level >= r.min_level).unwrap()
}

pub fn stat_score(save: &SaveFile, group: &str) -> u32 {
    LESSONS
        .iter()
        .filter(|l| l.stat_group == group)
        .filter(|l| is_lesson_passed(save, l.number))
        .count() as u32
}

pub fn stat_max(group: &str) -> u32 {
    LESSONS.iter().filter(|l| l.stat_group == group).count() as u32
}

pub fn is_lesson_passed(save: &SaveFile, lesson_num: u32) -> bool {
    save.lessons
        .get(&lesson_key(lesson_num))
        .is_some_and(|s| s.passed)
}

pub fn lesson_status<'a>(save: &'a SaveFile, lesson_num: u32) -> Option<&'a LessonStatus> {
    save.lessons.get(&lesson_key(lesson_num))
}

/// Passed / total for an inclusive lesson range.
pub fn range_progress(save: &SaveFile, first: u32, last: u32) -> (u32, u32) {
    let done = (first..=last).filter(|n| is_lesson_passed(save, *n)).count() as u32;
    (done, last - first + 1)
}

/// The first lesson not yet passed — the learner's frontier.
pub fn next_lesson(save: &SaveFile) -> Option<&'static LessonMeta> {
    LESSONS.iter().find(|l| !is_lesson_passed(save, l.number))
}

// ─── Calendar (dependency-free) ────────────────────────────────────────────
//
// Day bucketing for the activity heatmap without pulling in a date crate.
// Days are UTC: a late-night local session can land on the next day's cell.

/// Whole days since the Unix epoch.
pub fn epoch_day(secs: u64) -> i64 {
    (secs / 86_400) as i64
}

/// Howard Hinnant's civil_from_days: days-since-epoch -> (year, month, day).
pub fn civil_from_days(z: i64) -> (i32, u32, u32) {
    let z = z + 719_468;
    let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
    let doe = (z - era * 146_097) as u64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m, d)
}

/// Day of week for a days-since-epoch value. 0 = Sunday .. 6 = Saturday.
/// 1970-01-01 was a Thursday (weekday 4).
pub fn weekday(z: i64) -> u32 {
    (((z % 7) + 11) % 7) as u32
}

pub const MONTH_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

pub fn format_day(z: i64) -> String {
    let (y, m, d) = civil_from_days(z);
    format!("{} {} {}", MONTH_NAMES[(m - 1) as usize], d, y)
}
