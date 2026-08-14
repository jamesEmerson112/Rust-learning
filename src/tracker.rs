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

// ─── Save File (v2) ────────────────────────────────────────────────────────

pub const SAVE_VERSION: u32 = 2;

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

/// v1 -> v2. Returns true if anything changed (caller decides whether to persist).
///
/// v1 stored only `{passed, completed_at}`, and `--rescan` rewrote every timestamp,
/// so existing timestamps are all within seconds of one bulk rescan. We keep them as
/// the first-pass time but flag them `backfilled` so history-based panels can ignore
/// them instead of rendering a fake "45 lessons in one day" spike.
pub fn migrate(save: &mut SaveFile) -> bool {
    if save.version >= SAVE_VERSION {
        return false;
    }
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
    LessonMeta { number: 1,  title: "Hello Variables",                      stat_group: "Core Firmware" },
    LessonMeta { number: 2,  title: "Strings and Formatting",              stat_group: "Core Firmware" },
    LessonMeta { number: 3,  title: "Arrays and Iteration",                stat_group: "Data Cortex" },
    LessonMeta { number: 4,  title: "Tuples and Type Casting",             stat_group: "Data Cortex" },
    LessonMeta { number: 5,  title: "If/Else and For Loops",               stat_group: "Logic Matrix" },
    LessonMeta { number: 6,  title: "Match and String Building",           stat_group: "Logic Matrix" },
    LessonMeta { number: 7,  title: "Ownership and Borrowing",             stat_group: "Neural Wetware" },
    LessonMeta { number: 8,  title: "String Slices and Methods",           stat_group: "Neural Wetware" },
    LessonMeta { number: 9,  title: "Structs",                             stat_group: "Cyber Architecture" },
    LessonMeta { number: 10, title: "Methods and impl Blocks",             stat_group: "Cyber Architecture" },
    LessonMeta { number: 11, title: "Enums",                               stat_group: "Type Encoding" },
    LessonMeta { number: 12, title: "Option<T>",                           stat_group: "Type Encoding" },
    LessonMeta { number: 13, title: "HashMap Basics",                      stat_group: "Data Vault" },
    LessonMeta { number: 14, title: "HashMap Entry API",                   stat_group: "Data Vault" },
    LessonMeta { number: 15, title: "String Normalization",                stat_group: "String Forge" },
    LessonMeta { number: 16, title: "Word Count (HashMap + Strings)",      stat_group: "String Forge" },
    LessonMeta { number: 17, title: "Result Basics",                       stat_group: "Fault Tolerance" },
    LessonMeta { number: 18, title: "The ? Operator",                      stat_group: "Fault Tolerance" },
    LessonMeta { number: 19, title: "Error Chaining with map_err",         stat_group: "Error Channel" },
    LessonMeta { number: 20, title: "Closures",                            stat_group: "Error Channel" },
    LessonMeta { number: 21, title: "Map + Collect",                       stat_group: "Iterator Matrix" },
    LessonMeta { number: 22, title: "Filter",                              stat_group: "Iterator Matrix" },
    LessonMeta { number: 23, title: "Sum",                                 stat_group: "Iterator Matrix" },
    LessonMeta { number: 24, title: "Fold",                                stat_group: "Iterator Matrix" },
    LessonMeta { number: 25, title: "Debug Format",                        stat_group: "Debug Console" },
    LessonMeta { number: 26, title: "Traits",                              stat_group: "Protocol Layer" },
    LessonMeta { number: 27, title: "Generics",                            stat_group: "Protocol Layer" },
    LessonMeta { number: 28, title: "Trait Bounds",                        stat_group: "Bound Compiler" },
    LessonMeta { number: 29, title: "Lifetimes Intro",                     stat_group: "Bound Compiler" },
    LessonMeta { number: 30, title: "Box<T> — Heap Allocation",            stat_group: "Pointer Grid" },
    LessonMeta { number: 31, title: "Rc<T> — Shared Ownership",            stat_group: "Pointer Grid" },
    LessonMeta { number: 32, title: "Modules and Visibility",              stat_group: "System Kernel" },
    LessonMeta { number: 33, title: "Capstone: Service Log",               stat_group: "System Kernel" },
    LessonMeta { number: 34, title: "Custom Error Enum",                   stat_group: "Error Forge" },
    LessonMeta { number: 35, title: "impl Display for Errors",             stat_group: "Error Forge" },
    LessonMeta { number: 36, title: "impl Error Trait",                    stat_group: "Error Forge" },
    LessonMeta { number: 37, title: "thiserror Derive",                    stat_group: "Error Forge" },
    LessonMeta { number: 38, title: "anyhow Catch-All",                    stat_group: "Error Forge" },
    LessonMeta { number: 39, title: "Slices &[T]",                         stat_group: "Slice Wire" },
    LessonMeta { number: 40, title: "Struct Lifetimes",                    stat_group: "Slice Wire" },
    LessonMeta { number: 41, title: "Lifetime Elision",                    stat_group: "Slice Wire" },
    LessonMeta { number: 42, title: "Cell<T>",                             stat_group: "Mutation Core" },
    LessonMeta { number: 43, title: "RefCell<T>",                          stat_group: "Mutation Core" },
    LessonMeta { number: 44, title: "Rc<RefCell<T>>",                      stat_group: "Mutation Core" },
    LessonMeta { number: 45, title: "Custom Iterator",                     stat_group: "Iterator Engine" },
    LessonMeta { number: 46, title: "Iterator Adaptors",                   stat_group: "Iterator Engine" },
    LessonMeta { number: 47, title: "CSV Read",                            stat_group: "Data Pipeline" },
    LessonMeta { number: 48, title: "CSV Write",                           stat_group: "Data Pipeline" },
    LessonMeta { number: 49, title: "Serde JSON",                          stat_group: "Data Pipeline" },
    LessonMeta { number: 50, title: "async fn + tokio",                    stat_group: "Async Grid" },
    LessonMeta { number: 51, title: "tokio::spawn",                        stat_group: "Async Grid" },
    LessonMeta { number: 52, title: "Async Channels",                      stat_group: "Async Grid" },
    LessonMeta { number: 53, title: "clap Arg Parsing",                    stat_group: "Command Shell" },
    LessonMeta { number: 54, title: "Capstone: Salon CLI",                 stat_group: "Command Shell" },
    LessonMeta { number: 55, title: "Warmup: Fibonacci",                   stat_group: "Drill Matrix" },
    LessonMeta { number: 56, title: "Recursive Types with Box",            stat_group: "Heap Forge" },
    LessonMeta { number: 57, title: "Trait Objects (Box<dyn>)",            stat_group: "Heap Forge" },
    LessonMeta { number: 58, title: "Deref — Custom Smart Pointer",        stat_group: "Heap Forge" },
    LessonMeta { number: 59, title: "Warmup: Two Sum",                     stat_group: "Drill Matrix" },
    LessonMeta { number: 60, title: "Drop (RAII)",                         stat_group: "Lifecycle Core" },
    LessonMeta { number: 61, title: "Weak<T> and Cycles",                  stat_group: "Lifecycle Core" },
    LessonMeta { number: 62, title: "Warmup: Reverse a Vec",              stat_group: "Drill Matrix" },
    LessonMeta { number: 63, title: "Cell<T> Full API",                    stat_group: "Borrow Runtime" },
    LessonMeta { number: 64, title: "RefCell Runtime Borrow",              stat_group: "Borrow Runtime" },
    LessonMeta { number: 65, title: "Warmup: Contains Duplicate",          stat_group: "Drill Matrix" },
    LessonMeta { number: 66, title: "Arc<T> Across Threads",               stat_group: "Concurrency Lattice" },
    LessonMeta { number: 67, title: "Arc<Mutex<T>>",                       stat_group: "Concurrency Lattice" },
    LessonMeta { number: 68, title: "RwLock<T>",                           stat_group: "Concurrency Lattice" },
    LessonMeta { number: 69, title: "Sled Insert",                         stat_group: "Datavault" },
    LessonMeta { number: 70, title: "Sled Get",                            stat_group: "Datavault" },
    LessonMeta { number: 71, title: "Sled + Serde",                        stat_group: "Datavault" },
    LessonMeta { number: 72, title: "Sled Iterate",                        stat_group: "Datavault" },
    LessonMeta { number: 73, title: "Sled Query",                          stat_group: "Datavault" },
    LessonMeta { number: 74, title: "CSV to Sled",                         stat_group: "Datavault" },
    LessonMeta { number: 75, title: "Bug Hunt: The Vanishing Tally",       stat_group: "Bug Hunt" },
    LessonMeta { number: 76, title: "Bug Hunt: The Silent Zero",           stat_group: "Bug Hunt" },
    LessonMeta { number: 77, title: "Bug Hunt: The Missing VIP Tips",      stat_group: "Bug Hunt" },
    LessonMeta { number: 78, title: "Bug Hunt: The Overlooked Rush",       stat_group: "Bug Hunt" },
    LessonMeta { number: 79, title: "Bug Hunt: The Double-Booked Borrow",  stat_group: "Bug Hunt" },
    LessonMeta { number: 80, title: "Bug Hunt: The Half-Heard Clock-Out",  stat_group: "Bug Hunt" },
];

pub const STAT_GROUPS: [&str; 30] = [
    "Core Firmware",
    "Data Cortex",
    "Logic Matrix",
    "Neural Wetware",
    "Cyber Architecture",
    "Type Encoding",
    "Data Vault",
    "String Forge",
    "Fault Tolerance",
    "Error Channel",
    "Iterator Matrix",
    "Debug Console",
    "Protocol Layer",
    "Bound Compiler",
    "Pointer Grid",
    "System Kernel",
    "Error Forge",
    "Slice Wire",
    "Mutation Core",
    "Iterator Engine",
    "Data Pipeline",
    "Async Grid",
    "Command Shell",
    "Drill Matrix",
    "Heap Forge",
    "Lifecycle Core",
    "Borrow Runtime",
    "Concurrency Lattice",
    "Datavault",
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
    Chapter { name: "Salon Systems",  subtitle: "errors, I/O, async",     first: 34, last: 54 },
    Chapter { name: "Loadout",        subtitle: "Box & trait objects",    first: 55, last: 58 },
    Chapter { name: "Ghost Protocol", subtitle: "Drop & Weak",            first: 59, last: 61 },
    Chapter { name: "Inside the ICE", subtitle: "Cell & RefCell",         first: 62, last: 64 },
    Chapter { name: "The Crew",       subtitle: "Arc, Mutex, RwLock",     first: 65, last: 68 },
    Chapter { name: "The Vault",      subtitle: "sled + serde",           first: 69, last: 74 },
    Chapter { name: "Bug Hunt",       subtitle: "salon side jobs",        first: 75, last: 80 },
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
    Boss { lesson: 54, name: "THE SALON CLI",   tagline: "Errors, files, async — and a binary that ships.", guards_first: 34, guards_last: 54 },
    Boss { lesson: 74, name: "AEGIS-9",         tagline: "The vault runs on your own c71 codec.",           guards_first: 55, guards_last: 74 },
    Boss { lesson: 80, name: "THE LAST BUG",    tagline: "Six planted defects. No new concepts. Just you.", guards_first: 75, guards_last: 80 },
];

/// ★ BUG HUNT lessons — ship compiling-but-wrong code with a `// BUG:` symptom comment.
pub const BUG_LESSONS: [u32; 10] = [58, 61, 64, 67, 75, 76, 77, 78, 79, 80];

/// Lessons in c01–c54 left open as of the 2026-07-21 rescan (see CLAUDE.md).
pub const HOMEWORK_GAPS: [u32; 9] = [16, 19, 35, 37, 38, 40, 48, 49, 53];

// ─── Abilities ─────────────────────────────────────────────────────────────

pub struct Ability {
    pub name: &'static str,
    pub description: &'static str,
    pub lesson: u32,
}

#[rustfmt::skip]
pub const ABILITIES: [Ability; 80] = [
    // Script Kiddie (L1-4)
    Ability { name: "Stack Injection",         description: "Bind data to the stack with `let` and `mut`.",                       lesson: 1  },
    Ability { name: "String Splice",           description: "Manipulate heap strings and format output into the datastream.",     lesson: 2  },
    Ability { name: "Array Scan",              description: "Iterate fixed-size memory blocks and extract aggregate signals.",    lesson: 3  },
    Ability { name: "Tuple Decode",            description: "Destructure multi-value payloads and cast between data types.",      lesson: 4  },
    // Netrunner (L5-8)
    Ability { name: "Logic Gate Bypass",       description: "Route execution through conditional branches and loops.",            lesson: 5  },
    Ability { name: "Pattern Lock Crack",      description: "Match data against patterns and build string payloads.",             lesson: 6  },
    Ability { name: "Memory Ownership Hack",   description: "Seize exclusive control of heap-allocated data.",                    lesson: 7  },
    Ability { name: "Slice Wire Tap",          description: "Extract views into string memory without copying.",                  lesson: 8  },
    // ICE Breaker (L9-12)
    Ability { name: "Struct Fabrication",      description: "Forge custom data structures with named fields.",                    lesson: 9  },
    Ability { name: "Method Implant",          description: "Graft behavior onto structures with `impl` blocks.",                 lesson: 10 },
    Ability { name: "Enum Polymorphism",       description: "Define variant types that carry different payloads.",                lesson: 11 },
    Ability { name: "Null Shield",             description: "Handle absent data safely with `Option<T>` — no null crashes.",      lesson: 12 },
    // Chrome Operative (L13-16)
    Ability { name: "Hash Map Infiltration",   description: "Index and query key-value data stores at O(1).",                     lesson: 13 },
    Ability { name: "Entry Point Exploit",     description: "Use the entry API to atomically insert-or-update.",                  lesson: 14 },
    Ability { name: "String Normalizer",       description: "Sanitize raw text — trim punctuation, lowercase, prep for lookup.",  lesson: 15 },
    Ability { name: "Word Count Weave",        description: "Combine HashMap + string normalization into frequency analysis.",    lesson: 16 },
    // Ghost in the Wire (L17-20)
    Ability { name: "Error Channel",           description: "Propagate failure signals through `Result<T, E>`.",                  lesson: 17 },
    Ability { name: "Fault Cascade",           description: "Chain fallible operations with `?` without losing context.",         lesson: 18 },
    Ability { name: "Exception Gadget",        description: "Convert error types with `map_err` and bridge disparate failures.",  lesson: 19 },
    Ability { name: "Lambda Grenade",          description: "Deploy closures as first-class functions that capture scope.",       lesson: 20 },
    // Neon Assassin (L21-24)
    Ability { name: "Map Transmuter",          description: "Transform every element with `.map()` and `.collect()` into a Vec.", lesson: 21 },
    Ability { name: "Filter Prism",            description: "Select elements with `.filter()` — mastering the double-reference.", lesson: 22 },
    Ability { name: "Sum Aggregator",          description: "Collapse an iterator into one value with `.sum()`.",                 lesson: 23 },
    Ability { name: "Fold Reducer",            description: "Custom-reduce any sequence with `.fold(init, |acc, x| ...)`.",       lesson: 24 },
    // Silicon Shaman (L25-29)
    Ability { name: "Debug Lens",              description: "Inspect any `#[derive(Debug)]` type with `{:?}` formatting.",        lesson: 25 },
    Ability { name: "Trait Interface",         description: "Define shared behavior contracts across disparate types.",           lesson: 26 },
    Ability { name: "Generic Protocol",        description: "Write type-agnostic code with parametric polymorphism.",             lesson: 27 },
    Ability { name: "Bounded Channel",         description: "Constrain generics with trait bounds to unlock comparison.",         lesson: 28 },
    Ability { name: "Lifetime Weaver",         description: "Annotate `'a` so references never outlive the data they point to.", lesson: 29 },
    // Neon Sovereign (L30-33)
    Ability { name: "Heap Injector",           description: "Box values onto the heap and break recursive type cycles.",          lesson: 30 },
    Ability { name: "Ownership Broadcast",     description: "Share one allocation via `Rc` with reference counting.",             lesson: 31 },
    Ability { name: "Module Firewall",         description: "Partition code into modules with controlled visibility.",            lesson: 32 },
    Ability { name: "Full System Access",      description: "All core subsystems integrated. The compiler bends to your will.",   lesson: 33 },
    // Protocol Architect (L34-38)
    Ability { name: "Error Enum Forge",        description: "Define custom error enums with distinct failure variants.",          lesson: 34 },
    Ability { name: "Display Renderer",        description: "Implement `Display` so errors print human-readable messages.",       lesson: 35 },
    Ability { name: "Error Trait Wire",        description: "Wire custom errors into `std::error::Error` for composability.",     lesson: 36 },
    Ability { name: "Derive Shortcut",         description: "Derive error boilerplate away with `thiserror`.",                    lesson: 37 },
    Ability { name: "Anyhow Absorber",         description: "Catch any error type with `anyhow::Result` — zero boilerplate.",    lesson: 38 },
    // Memory Surgeon (L39-41)
    Ability { name: "Slice Scanner",           description: "Operate on contiguous memory views without allocation.",             lesson: 39 },
    Ability { name: "Struct Borrow Bind",      description: "Embed borrowed references inside structs with lifetime params.",    lesson: 40 },
    Ability { name: "Elision Insight",         description: "Read the compiler's lifetime elision rules fluently.",              lesson: 41 },
    // (L42-44)
    Ability { name: "Cell Mutator",            description: "Mutate through shared references with `Cell<T>`.",                  lesson: 42 },
    Ability { name: "RefCell Override",         description: "Runtime borrow-check with `RefCell<T>` for complex interiors.",     lesson: 43 },
    Ability { name: "Shared Mutable Link",     description: "Combine `Rc` and `RefCell` for shared mutable state.",              lesson: 44 },
    // Stream Weaver (L45-46)
    Ability { name: "Iterator Constructor",    description: "Implement the `Iterator` trait on a custom struct.",                 lesson: 45 },
    Ability { name: "Adaptor Chain",           description: "Compose filter/map on custom iterators — full pipeline.",            lesson: 46 },
    // (L47-49)
    Ability { name: "CSV Reader",              description: "Parse a CSV file into structs with `csv` + `serde`.",               lesson: 47 },
    Ability { name: "CSV Writer",              description: "Write structs to a CSV file with `csv` + `serde`.",                 lesson: 48 },
    Ability { name: "Serde Codec",             description: "Serialize and deserialize structs as JSON.",                         lesson: 49 },
    // Async Phantom (L50-52)
    Ability { name: "Async Ignition",          description: "Write your first `async fn` and `.await` it.",                      lesson: 50 },
    Ability { name: "Task Spawner",            description: "Run concurrent work with `tokio::spawn`.",                          lesson: 51 },
    Ability { name: "Channel Relay",           description: "Send messages between async tasks with `mpsc`.",                    lesson: 52 },
    // Salon Sovereign (L53-54)
    Ability { name: "Arg Parser",              description: "Parse CLI arguments with `clap` derive macros.",                    lesson: 53 },
    Ability { name: "Salon Sovereign",         description: "Full salon scheduler. Every system integrated. You ship software.", lesson: 54 },
    // Drill Matrix warmups + Smart Pointers Deep Dive (L55-68)
    Ability { name: "Sequence Solver",         description: "Compute Fibonacci iteratively — warm up the loop muscles.",          lesson: 55 },
    Ability { name: "Recursive Forge",         description: "Box a self-referential type — build lists the compiler can size.",   lesson: 56 },
    Ability { name: "Dynamic Dispatch",        description: "Store mixed types behind `Box<dyn Trait>` and call via vtable.",      lesson: 57 },
    Ability { name: "Deref Conjuration",       description: "Implement `Deref` — forge your own smart pointer with auto-deref.",   lesson: 58 },
    Ability { name: "Pair Finder",             description: "Two-Sum in one pass with a HashMap — the classic warmup.",            lesson: 59 },
    Ability { name: "Drop Ritual",             description: "Hook `Drop` for deterministic RAII cleanup — no garbage collector.",  lesson: 60 },
    Ability { name: "Weak Link",               description: "Break reference cycles with `Weak<T>` — leak-proof shared graphs.",   lesson: 61 },
    Ability { name: "Array Reversal",          description: "Reverse a slice in place with two pointers.",                         lesson: 62 },
    Ability { name: "Cell Phase-Shift",        description: "Swap and take interior state through `&self` with the full Cell API.", lesson: 63 },
    Ability { name: "Borrow Tripwire",         description: "Provoke and survive RefCell's runtime borrow check.",                 lesson: 64 },
    Ability { name: "Dupe Scanner",            description: "Detect duplicates with a HashSet in one pass.",                       lesson: 65 },
    Ability { name: "Atomic Broadcast",        description: "Share immutable data across threads with `Arc<T>`.",                  lesson: 66 },
    Ability { name: "Mutex Lockdown",          description: "Guard shared mutable state across threads with `Arc<Mutex<T>>`.",     lesson: 67 },
    Ability { name: "RwLock Mastery",          description: "Many readers, one writer — concurrent access with `RwLock<T>`.",      lesson: 68 },
    // Data Daemon (L69-74)
    Ability { name: "Key Inserter",            description: "Open a sled database and insert key/value bytes.",                    lesson: 69 },
    Ability { name: "Key Fetcher",             description: "Read values back out of sled by key.",                                lesson: 70 },
    Ability { name: "Struct Persister",        description: "Store and load structs in sled with serde.",                          lesson: 71 },
    Ability { name: "Store Scanner",           description: "Iterate every entry in the database.",                                lesson: 72 },
    Ability { name: "Iterator Query",          description: "Filter and project records with iterators, no SQL.",                  lesson: 73 },
    Ability { name: "Ingest Pipeline",         description: "Load a CSV into sled and query it back.",                             lesson: 74 },
    // Bug Hunt (L75-80) — salon side jobs: debug broken code, don't write from scratch
    Ability { name: "Tally Ward",              description: "Fix the HashMap clobber — `entry().or_insert()` accumulates, `insert` resets.", lesson: 75 },
    Ability { name: "Fault Surfacing",         description: "Un-swallow parse errors — a corrupt row must `Err`, not zero out.",   lesson: 76 },
    Ability { name: "Predicate Flip",          description: "Right an inverted `filter` — keep what the predicate says.",          lesson: 77 },
    Ability { name: "Edge Reclaim",            description: "Reclaim the off-by-one — the last window starts at `n - width`.",     lesson: 78 },
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
    Rank { min_level: 0,  name: "Disconnected",        quote: "No signal detected. Jack in to begin." },
    Rank { min_level: 1,  name: "Script Kiddie",        quote: "You've breached the first firewall. The grid recognizes you." },
    Rank { min_level: 5,  name: "Netrunner",            quote: "The heap sprawls before you — an endless neon datascape." },
    Rank { min_level: 9,  name: "ICE Breaker",          quote: "Intrusion Countermeasures mean nothing to you now." },
    Rank { min_level: 13, name: "Chrome Operative",     quote: "Your pattern recognition subroutines are fully online." },
    Rank { min_level: 17, name: "Ghost in the Wire",    quote: "You exist in the protocol layer — type-agnostic, untraceable." },
    Rank { min_level: 21, name: "Neon Assassin",        quote: "Streams bend to your will — one combinator at a time." },
    Rank { min_level: 25, name: "Silicon Shaman",       quote: "Lifetimes, pointers, and shared memory answer to your incantations." },
    Rank { min_level: 30, name: "Neon Sovereign",        quote: "Heap and stack are yours. Pointers and modules obey." },
    Rank { min_level: 34, name: "Protocol Architect",   quote: "Errors are data. You design the failure paths." },
    Rank { min_level: 40, name: "Memory Surgeon",       quote: "Slices, lifetimes, interior mutability — you operate at the edge." },
    Rank { min_level: 45, name: "Stream Weaver",        quote: "Custom iterators, file pipelines, data in motion." },
    Rank { min_level: 50, name: "Async Phantom",        quote: "Concurrent tasks bend to your will. Nothing blocks you." },
    Rank { min_level: 55, name: "Heap Warden",          quote: "The allocator answers to you — Box, trait objects, and Drop glue obey." },
    Rank { min_level: 60, name: "Memory Reaper",        quote: "Cycles, weak links, interior cells — nothing leaks past your reach." },
    Rank { min_level: 66, name: "Concurrency Daemon",   quote: "You fork reality across threads. Arc and locks are your instruments." },
    Rank { min_level: 69, name: "Data Daemon",          quote: "Keys and values bend to your will — no query language required." },
    Rank { min_level: 74, name: "Vault Sovereign",      quote: "Aegis-9 is yours. The vault runs on your code. 🦀" },
    Rank { min_level: 80, name: "Zero-Day Sovereign",   quote: "Every bug in the grid dies on your blade. Nothing broken survives you." },
];

pub const CLASSES: [&str; 4] = [
    "Chrome Surgeon",
    "Data Wraith",
    "Borrow Monk",
    "Codec Alchemist",
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
