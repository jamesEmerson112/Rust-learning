// RPG Progress Tracker for Rust Learning
// Scans test results, tracks a persistent character, renders a terminal character sheet.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

// ─── ANSI Colors ───────────────────────────────────────────────────────────

mod color {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const RED: &str = "\x1b[31m";
    pub const CYAN: &str = "\x1b[36m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub fn green(s: &str) -> String {
        format!("{GREEN}{s}{RESET}")
    }
    pub fn yellow(s: &str) -> String {
        format!("{YELLOW}{s}{RESET}")
    }
    pub fn red(s: &str) -> String {
        format!("{RED}{s}{RESET}")
    }
    pub fn cyan(s: &str) -> String {
        format!("{CYAN}{s}{RESET}")
    }
    pub fn bold(s: &str) -> String {
        format!("{BOLD}{s}{RESET}")
    }
    pub fn dim(s: &str) -> String {
        format!("{DIM}{s}{RESET}")
    }
    pub fn bold_green(s: &str) -> String {
        format!("{BOLD}{GREEN}{s}{RESET}")
    }
    pub fn bold_yellow(s: &str) -> String {
        format!("{BOLD}{YELLOW}{s}{RESET}")
    }
    pub fn bold_cyan(s: &str) -> String {
        format!("{BOLD}{CYAN}{s}{RESET}")
    }
    pub fn bold_magenta(s: &str) -> String {
        format!("{BOLD}{MAGENTA}{s}{RESET}")
    }
}

// ─── Data Types ────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone)]
struct SaveFile {
    version: u32,
    character: Character,
    lessons: HashMap<String, LessonStatus>,
    last_scan: u64,
}

#[derive(Serialize, Deserialize, Clone)]
struct Character {
    name: String,
    class: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct LessonStatus {
    passed: bool,
    completed_at: Option<u64>,
}

// ─── Lesson Metadata ───────────────────────────────────────────────────────

struct LessonMeta {
    number: u32,
    title: &'static str,
    stat_group: &'static str,
}

const LESSONS: [LessonMeta; 20] = [
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
    LessonMeta { number: 15, title: "Result Basics",                       stat_group: "Fault Tolerance" },
    LessonMeta { number: 16, title: "The ? Operator",                      stat_group: "Fault Tolerance" },
    LessonMeta { number: 17, title: "Traits",                              stat_group: "Protocol Layer" },
    LessonMeta { number: 18, title: "Generics and Trait Bounds",           stat_group: "Protocol Layer" },
    LessonMeta { number: 19, title: "Modules and Visibility",              stat_group: "System Kernel" },
    LessonMeta { number: 20, title: "Capstone: Gradebook",                 stat_group: "System Kernel" },
];

const STAT_GROUPS: [&str; 10] = [
    "Core Firmware",
    "Data Cortex",
    "Logic Matrix",
    "Neural Wetware",
    "Cyber Architecture",
    "Type Encoding",
    "Data Vault",
    "Fault Tolerance",
    "Protocol Layer",
    "System Kernel",
];

// ─── Abilities ─────────────────────────────────────────────────────────────

struct Ability {
    name: &'static str,
    description: &'static str,
    lesson: u32,
}

const ABILITIES: [Ability; 20] = [
    // Script Kiddie (L1–4)
    Ability { name: "Stack Injection",         description: "Bind data to the stack with `let` and `mut`.",                       lesson: 1  },
    Ability { name: "String Splice",           description: "Manipulate heap strings and format output into the datastream.",     lesson: 2  },
    Ability { name: "Array Scan",              description: "Iterate fixed-size memory blocks and extract aggregate signals.",    lesson: 3  },
    Ability { name: "Tuple Decode",            description: "Destructure multi-value payloads and cast between data types.",      lesson: 4  },
    // Netrunner (L5–8)
    Ability { name: "Logic Gate Bypass",       description: "Route execution through conditional branches and loops.",            lesson: 5  },
    Ability { name: "Pattern Lock Crack",      description: "Match data against patterns and build string payloads.",             lesson: 6  },
    Ability { name: "Memory Ownership Hack",   description: "Seize exclusive control of heap-allocated data.",                    lesson: 7  },
    Ability { name: "Slice Wire Tap",          description: "Extract views into string memory without copying.",                  lesson: 8  },
    // ICE Breaker (L9–12)
    Ability { name: "Struct Fabrication",      description: "Forge custom data structures with named fields.",                    lesson: 9  },
    Ability { name: "Method Implant",          description: "Graft behavior onto structures with `impl` blocks.",                 lesson: 10 },
    Ability { name: "Enum Polymorphism",       description: "Define variant types that carry different payloads.",                lesson: 11 },
    Ability { name: "Null Shield",             description: "Handle absent data safely with `Option<T>` — no null crashes.",      lesson: 12 },
    // Chrome Operative (L13–16)
    Ability { name: "Hash Map Infiltration",   description: "Index and query key-value data stores at O(1).",                     lesson: 13 },
    Ability { name: "Entry Point Exploit",     description: "Use the entry API to atomically insert-or-update.",                  lesson: 14 },
    Ability { name: "Error Channel",           description: "Propagate failure signals through `Result<T, E>`.",                  lesson: 15 },
    Ability { name: "Fault Cascade",           description: "Chain fallible operations with `?` without losing context.",         lesson: 16 },
    // Ghost in the Wire (L17–19)
    Ability { name: "Trait Interface",         description: "Define shared behavior contracts across disparate types.",           lesson: 17 },
    Ability { name: "Generic Protocol",        description: "Write type-agnostic code with compile-time trait bounds.",           lesson: 18 },
    Ability { name: "Module Firewall",         description: "Partition code into modules with controlled visibility.",            lesson: 19 },
    // Zero-Day Sovereign (L20)
    Ability { name: "Full System Access",      description: "All subsystems integrated. The compiler bends to your will.",        lesson: 20 },
];

// ─── Ranks ─────────────────────────────────────────────────────────────────

struct Rank {
    min_level: u32,
    name: &'static str,
    quote: &'static str,
}

const RANKS: [Rank; 7] = [
    Rank { min_level: 0,  name: "Disconnected",        quote: "No signal detected. Jack in to begin." },
    Rank { min_level: 1,  name: "Script Kiddie",        quote: "You've breached the first firewall. The grid recognizes you." },
    Rank { min_level: 5,  name: "Netrunner",            quote: "The heap sprawls before you — an endless neon datascape." },
    Rank { min_level: 9,  name: "ICE Breaker",          quote: "Intrusion Countermeasures mean nothing to you now." },
    Rank { min_level: 13, name: "Chrome Operative",     quote: "Your pattern recognition subroutines are fully online." },
    Rank { min_level: 17, name: "Ghost in the Wire",    quote: "You exist in the protocol layer — type-agnostic, untraceable." },
    Rank { min_level: 20, name: "Zero-Day Sovereign",   quote: "You own the system. Every compiler warning bows to your will. 🦀" },
];

const CLASSES: [&str; 4] = [
    "Chrome Surgeon",
    "Data Wraith",
    "Borrow Monk",
    "Codec Alchemist",
];

// ─── Helpers ───────────────────────────────────────────────────────────────

fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn lesson_key(n: u32) -> String {
    format!("c{n:02}")
}

fn lessons_passed(save: &SaveFile) -> u32 {
    save.lessons.values().filter(|s| s.passed).count() as u32
}

fn get_rank(level: u32) -> &'static Rank {
    RANKS.iter().rev().find(|r| level >= r.min_level).unwrap()
}

fn stat_score(save: &SaveFile, group: &str) -> u32 {
    LESSONS
        .iter()
        .filter(|l| l.stat_group == group)
        .filter(|l| {
            save.lessons
                .get(&lesson_key(l.number))
                .is_some_and(|s| s.passed)
        })
        .count() as u32
}

fn is_lesson_passed(save: &SaveFile, lesson_num: u32) -> bool {
    save.lessons
        .get(&lesson_key(lesson_num))
        .is_some_and(|s| s.passed)
}

// ─── Save File I/O ─────────────────────────────────────────────────────────

fn save_path() -> std::path::PathBuf {
    std::path::PathBuf::from(".rustacean_save.json")
}

fn load_save() -> Option<SaveFile> {
    let data = std::fs::read_to_string(save_path()).ok()?;
    serde_json::from_str(&data).ok()
}

fn write_save(save: &SaveFile) {
    let json = serde_json::to_string_pretty(save).expect("Failed to serialize save file");
    std::fs::write(save_path(), json).expect("Failed to write save file");
}

// ─── Character Creation ────────────────────────────────────────────────────

fn prompt_line(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn create_character() -> SaveFile {
    println!();
    println!("{}",   color::bold_cyan("╔══════════════════════════════════════════════╗"));
    println!("{}",   color::bold_cyan("║       >> NEURAL LINK ESTABLISHED <<          ║"));
    println!("{}",   color::bold_cyan("║       Welcome to the Rust Underground        ║"));
    println!("{}",   color::bold_cyan("╚══════════════════════════════════════════════╝"));
    println!();
    println!("  Before you lies a grid of {} data nodes.", color::bold("20"));
    println!("  Master them all to earn the rank of {}.", color::bold_yellow("Zero-Day Sovereign"));
    println!();

    // Name
    let name = {
        let input = prompt_line(&format!("  {} ", color::cyan("Enter your handle [Rustacean]:")));
        if input.is_empty() { "Rustacean".to_string() } else { input }
    };

    // Class
    println!();
    println!("  {}", color::bold("Choose your class:"));
    for (i, class) in CLASSES.iter().enumerate() {
        println!("    {}  {}", color::bold_yellow(&format!("[{}]", i + 1)), class);
    }
    println!();
    let class = loop {
        let input = prompt_line(&format!("  {} ", color::cyan("Pick 1-4 [1]:")));
        let choice: usize = if input.is_empty() {
            1
        } else {
            match input.parse() {
                Ok(n) if (1..=4).contains(&n) => n,
                _ => {
                    println!("  {} Pick a number 1-4.", color::red("Invalid."));
                    continue;
                }
            }
        };
        break CLASSES[choice - 1].to_string();
    };

    println!();
    println!(
        "  {} {}, the {}!",
        color::bold_green("Link established,"),
        color::bold(&name),
        color::bold_magenta(&class)
    );
    println!("  Your infiltration begins now. Complete exercises and run this tracker to level up.");
    println!();

    SaveFile {
        version: 1,
        character: Character { name, class },
        lessons: HashMap::new(),
        last_scan: now_epoch(),
    }
}

// ─── Test Runner ───────────────────────────────────────────────────────────

fn check_lesson(n: u32) -> bool {
    let test_name = format!("c{n:02}_tests");
    let output = Command::new("cargo")
        .args(["test", "--test", &test_name, "--quiet"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    match output {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}

struct ScanResult {
    newly_completed: Vec<u32>,
    #[allow(dead_code)]
    regressions: Vec<u32>,
}

fn scan_lessons(save: &mut SaveFile, rescan: bool) -> ScanResult {
    let mut newly_completed = Vec::new();
    let mut regressions = Vec::new();

    println!();
    println!("  {}", color::bold("Scanning data nodes..."));
    println!();

    // Determine start point
    let start_from = if rescan {
        // Clear all statuses for full rescan
        save.lessons.clear();
        1
    } else {
        // Find the first lesson that hasn't been passed yet,
        // but go back one to re-verify for regression
        let first_incomplete = LESSONS
            .iter()
            .find(|l| {
                !save
                    .lessons
                    .get(&lesson_key(l.number))
                    .is_some_and(|s| s.passed)
            })
            .map(|l| l.number)
            .unwrap_or(20);
        first_incomplete.saturating_sub(1).max(1)
    };

    for lesson in &LESSONS {
        if lesson.number < start_from {
            // Already verified, show cached status
            let key = lesson_key(lesson.number);
            let passed = save.lessons.get(&key).is_some_and(|s| s.passed);
            if passed {
                println!(
                    "    {} c{:02} {} {}",
                    color::green("[SKIP]"),
                    lesson.number,
                    lesson.title,
                    color::dim("(verified)")
                );
            }
            continue;
        }

        // Print lesson being tested
        print!(
            "    {} c{:02} {}... ",
            color::dim("[TEST]"),
            lesson.number,
            lesson.title
        );
        io::stdout().flush().unwrap();

        let passed = check_lesson(lesson.number);
        let key = lesson_key(lesson.number);

        let was_passed = save.lessons.get(&key).is_some_and(|s| s.passed);

        if passed {
            println!("{}", color::bold_green("PASS"));
            if !was_passed {
                newly_completed.push(lesson.number);
                save.lessons.insert(
                    key,
                    LessonStatus {
                        passed: true,
                        completed_at: Some(now_epoch()),
                    },
                );
            }
        } else {
            println!("{}", color::red("FAIL"));
            if was_passed {
                regressions.push(lesson.number);
                save.lessons.insert(
                    key,
                    LessonStatus {
                        passed: false,
                        completed_at: None,
                    },
                );
            }
            if !rescan {
                // Linear unlock: stop at first failure
                break;
            }
        }
    }

    println!();

    if !regressions.is_empty() {
        println!(
            "  {} Regressions detected in: {}",
            color::bold_yellow("WARNING:"),
            regressions
                .iter()
                .map(|n| format!("c{n:02}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!();
    }

    save.last_scan = now_epoch();

    ScanResult {
        newly_completed,
        regressions,
    }
}

// ─── Level-Up Celebration ──────────────────────────────────────────────────

fn celebrate_levelup(save: &SaveFile, scan: &ScanResult) {
    if scan.newly_completed.is_empty() {
        return;
    }

    let level = lessons_passed(save);
    let xp_gained = scan.newly_completed.len() as u32 * 100;

    println!("  {}", color::bold_yellow("╔══════════════════════════════════════════════╗"));
    println!("  {}",   color::bold_yellow("║              ★  LEVEL UP!  ★                ║"));
    println!("  {}", color::bold_yellow("╚══════════════════════════════════════════════╝"));
    println!();

    for &n in &scan.newly_completed {
        let lesson = &LESSONS[(n - 1) as usize];
        println!(
            "    {} c{:02} {} — {} +100 XP",
            color::bold_green("✓"),
            n,
            lesson.title,
            color::bold_green(lesson.stat_group)
        );
    }

    println!();
    println!(
        "    {} +{} XP  |  Level {} → {}",
        color::bold_yellow("XP:"),
        xp_gained,
        level - scan.newly_completed.len() as u32,
        level
    );

    // Ability unlocks
    let newly_unlocked: Vec<&Ability> = ABILITIES
        .iter()
        .filter(|a| scan.newly_completed.contains(&a.lesson))
        .collect();

    if !newly_unlocked.is_empty() {
        println!();
        for ability in &newly_unlocked {
            println!(
                "    {}  {}",
                color::bold_cyan("ABILITY UNLOCKED >>"),
                color::bold_green(ability.name)
            );
            println!(
                "      \"{}\"",
                color::cyan(ability.description)
            );
        }
    }

    // Check for rank change
    let old_level = level - scan.newly_completed.len() as u32;
    let old_rank = get_rank(old_level);
    let new_rank = get_rank(level);

    if old_rank.name != new_rank.name {
        println!();
        println!(
            "    {}",
            color::bold_magenta("═══ NEW RANK UNLOCKED ═══")
        );
        println!(
            "    {}  {}",
            color::bold_cyan(">>"),
            color::bold(&new_rank.name.to_uppercase())
        );
        println!(
            "    {}  \"{}\"",
            color::dim("  "),
            color::cyan(new_rank.quote)
        );
    }

    println!();
}

// ─── Display: Character Sheet ──────────────────────────────────────────────

fn xp_bar(current: u32, max: u32, width: usize) -> String {
    let filled = if max > 0 {
        (current as f64 / max as f64 * width as f64).round() as usize
    } else {
        0
    };
    let empty = width - filled;
    format!(
        "{}{}{}{}",
        color::GREEN,
        "█".repeat(filled),
        color::DIM,
        "░".repeat(empty),
    ) + color::RESET
}

fn display_character_sheet(save: &SaveFile) {
    let level = lessons_passed(save);
    let xp = level * 100;
    let rank = get_rank(level);

    // Header
    println!(
        "  {}",
        color::bold_cyan("╔══════════════════════════════════════════════════════════╗")
    );
    println!(
        "  {}  {} {}",
        color::bold_cyan("║"),
        color::bold(&save.character.name),
        color::dim(&format!("the {}", save.character.class))
    );
    println!(
        "  {}  Rank: {}",
        color::bold_cyan("║"),
        color::bold_yellow(rank.name)
    );
    println!(
        "  {}  Level: {}/20   XP: {}/2000",
        color::bold_cyan("║"),
        color::bold(&level.to_string()),
        xp
    );
    println!(
        "  {}  [{}] {}/2000",
        color::bold_cyan("║"),
        xp_bar(xp, 2000, 30),
        xp
    );
    println!(
        "  {}",
        color::bold_cyan("╚══════════════════════════════════════════════════════════╝")
    );

    // Stats
    println!();
    println!("  {}", color::bold("STATS"));
    for group in &STAT_GROUPS {
        let score = stat_score(save, group);
        let bar = match score {
            0 => color::dim("○ ○"),
            1 => format!("{} {}", color::yellow("●"), color::dim("○")),
            _ => format!("{} {}", color::green("●"), color::green("●")),
        };
        let label = match score {
            2 => color::green(group),
            1 => color::yellow(group),
            _ => color::dim(group),
        };
        println!("    {bar}  {label}  ({score}/2)");
    }

    // Abilities
    println!();
    println!("  {}", color::bold("ABILITIES"));
    for ability in &ABILITIES {
        let unlocked = is_lesson_passed(save, ability.lesson);
        if unlocked {
            println!(
                "    {}  {} — {}",
                color::bold_green("[ON] "),
                color::green(ability.name),
                color::dim(ability.description)
            );
        } else {
            println!(
                "    {}  {} — {}",
                color::dim("[OFF]"),
                color::dim(ability.name),
                color::dim("████████████████████████████████████████")
            );
        }
    }

    // Quest Log
    println!();
    println!("  {}", color::bold("QUEST LOG"));

    let mut hit_first_fail = false;
    for lesson in &LESSONS {
        let key = lesson_key(lesson.number);
        let passed = save.lessons.get(&key).is_some_and(|s| s.passed);

        let (tag, title_display) = if passed {
            (color::bold_green("[PASS]"), color::green(lesson.title))
        } else if !hit_first_fail {
            hit_first_fail = true;
            (
                color::bold_yellow("[NEXT]"),
                color::bold_yellow(lesson.title),
            )
        } else {
            (color::dim("[LOCK]"), color::dim(lesson.title))
        };

        println!(
            "    {tag}  c{:02} — {title_display}",
            lesson.number
        );
    }

    // Next step hint
    println!();
    if level == 20 {
        println!("  {}", color::bold_cyan("╔══════════════════════════════════════════════════════════╗"));
        println!("  {}",   color::bold_cyan("║        🦀  SYSTEM FULLY COMPROMISED  🦀                ║"));
        println!("  {}", color::bold_cyan("╚══════════════════════════════════════════════════════════╝"));
        println!();
        println!("  {}",   color::bold_green("  You are the Zero-Day Sovereign. The Rust Underground salutes you."));
        println!("  {}",   color::dim("  Check the README for suggested next topics beyond chapter 20."));
    } else {
        let next = LESSONS.iter().find(|l| {
            !save
                .lessons
                .get(&lesson_key(l.number))
                .is_some_and(|s| s.passed)
        });
        if let Some(lesson) = next {
            let n = lesson.number;
            println!("  {}", color::bold("NEXT STEP"));
            println!(
                "    Read the example:    {}",
                color::cyan(&format!("cargo run --bin c{n:02}_example"))
            );
            println!(
                "    Edit the exercise:   {}",
                color::cyan(&format!("src/bin/c{n:02}_exercise.rs"))
            );
            println!(
                "    Test your solution:  {}",
                color::cyan(&format!("cargo test --test c{n:02}_tests"))
            );
            println!(
                "    Then come back:      {}",
                color::cyan("cargo run --bin progress")
            );
        }
    }

    println!();
}

// ─── Help ──────────────────────────────────────────────────────────────────

fn print_help() {
    println!();
    println!("  {}", color::bold("Rust Underground — Cyberpunk Progress Tracker"));
    println!();
    println!("  {}", color::bold("USAGE:"));
    println!("    cargo run --bin progress              Normal run (incremental scan + display)");
    println!("    cargo run --bin progress -- --rescan  Re-test all 20 data nodes from scratch");
    println!("    cargo run --bin progress -- --reset   Delete save file and start fresh");
    println!("    cargo run --bin progress -- --help    Show this help message");
    println!();
    println!("  {}", color::bold("HOW IT WORKS:"));
    println!("    1. Complete exercises in src/bin/cXX_exercise.rs");
    println!("    2. Run this tracker to scan your test results");
    println!("    3. Watch your character level up as you progress!");
    println!();
    println!("  {}", color::bold("FILES:"));
    println!("    .rustacean_save.json  Your character save file (auto-created)");
    println!();
}

// ─── Main ──────────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let rescan = args.iter().any(|a| a == "--rescan");
    let reset = args.iter().any(|a| a == "--reset");
    let help = args.iter().any(|a| a == "--help" || a == "-h");

    if help {
        print_help();
        return;
    }

    // Handle reset
    if reset {
        if save_path().exists() {
            std::fs::remove_file(save_path()).expect("Failed to delete save file");
            println!();
            println!("  {} Save file deleted.", color::bold_yellow("RESET:"));
        }
        let save = create_character();
        write_save(&save);
        display_character_sheet(&save);
        return;
    }

    // Load or create
    let mut save = match load_save() {
        Some(s) => s,
        None => {
            let s = create_character();
            write_save(&s);
            s
        }
    };

    // Scan
    let scan_result = scan_lessons(&mut save, rescan);

    // Celebrate
    celebrate_levelup(&save, &scan_result);

    // Display
    display_character_sheet(&save);

    // Save
    write_save(&save);
}
