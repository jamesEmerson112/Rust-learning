// RPG Progress Tracker for Rust Learning
// Scans test results, tracks a persistent character, renders a terminal character sheet.
//
// The data model (lessons, abilities, ranks, save file) lives in src/tracker.rs and is
// shared with the `dashboard` binary. This binary owns every write to the save file.

#[path = "../tracker.rs"]
mod tracker;
use tracker::*;

use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use std::process::Command;

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
    println!("{}",   color::bold_cyan("║             >> SHIFT STARTED <<              ║"));
    println!("{}",   color::bold_cyan("║       Welcome to Rust General Hospital       ║"));
    println!("{}",   color::bold_cyan("╚══════════════════════════════════════════════╝"));
    println!();
    println!("  {} cases are waiting on the ward.", color::bold(&NUM_LESSONS.to_string()));
    println!("  Close them all to earn the rank of {}.", color::bold_yellow(RANKS[RANKS.len() - 1].name));
    println!();

    // Name
    let name = {
        let input = prompt_line(&format!("  {} ", color::cyan("Enter your name [Rustacean]:")));
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
        version: SAVE_VERSION,
        character: Character { name, class },
        lessons: HashMap::new(),
        last_scan: now_epoch(),
        history: Vec::new(),
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
    println!("  {}", color::bold("Reviewing case files..."));
    println!();

    save.history.push(Event::new(EventKind::Scan, None));

    // Snapshot the pre-scan state: a rescan is about to reset `passed` on everything,
    // and regression detection needs to know what was green when we walked in.
    let previously_passed: HashSet<u32> = LESSONS
        .iter()
        .map(|l| l.number)
        .filter(|n| is_lesson_passed(save, *n))
        .collect();

    // Determine start point
    let start_from = if rescan {
        // Full rescan re-verifies from zero, but the learner's history is not evidence
        // to be thrown away: first_passed_at, attempts and backfilled all carry forward.
        for status in save.lessons.values_mut() {
            status.passed = false;
            status.completed_at = None;
        }
        1
    } else {
        // Find the first lesson that hasn't been passed yet,
        // but go back one to re-verify for regression
        let first_incomplete = LESSONS
            .iter()
            .find(|l| !is_lesson_passed(save, l.number))
            .map(|l| l.number)
            .unwrap_or(NUM_LESSONS);
        first_incomplete.saturating_sub(1).max(1)
    };

    for lesson in &LESSONS {
        if lesson.number < start_from {
            // Already verified, show cached status
            let passed = is_lesson_passed(save, lesson.number);
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

        let was_passed = previously_passed.contains(&lesson.number);
        // `first_passed_at` is the only spike-proof signal of a genuine first solve:
        // re-verifying old work during a rescan must not mint a Pass event or clear
        // `backfilled`, or the heatmap grows a fake 45-lessons-in-one-day spike.
        let was_ever_passed = save
            .lessons
            .get(&key)
            .is_some_and(|s| s.first_passed_at.is_some());

        if passed {
            println!("{}", color::bold_green("PASS"));
            let now = now_epoch();
            let status = save.lessons.entry(key).or_default();
            status.passed = true;
            status.completed_at = Some(now);
            if !was_ever_passed {
                status.first_passed_at = Some(now);
            }
            // Green now, not green when we walked in: real work on a real day. A rescan
            // re-verifying old code hits neither branch — that is the point.
            if !was_passed {
                status.backfilled = false;
                newly_completed.push(lesson.number);
                save.history
                    .push(Event::new(EventKind::Pass, Some(lesson.number)));
            }
        } else {
            println!("{}", color::red("FAIL"));
            if was_passed {
                regressions.push(lesson.number);
            }
            // A full rescan sweeps all 80 lessons, so it must not count an attempt for
            // lessons the learner has never opened.
            if was_passed || !rescan {
                let status = save.lessons.entry(key).or_default();
                status.passed = false;
                status.completed_at = None;
                if !rescan {
                    status.attempts += 1;
                }
            }
            save.history
                .push(Event::new(EventKind::Fail, Some(lesson.number)));
            if was_passed {
                save.history
                    .push(Event::new(EventKind::Regression, Some(lesson.number)));
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

fn celebrate_levelup(save: &mut SaveFile, scan: &ScanResult) {
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
        save.history
            .push(Event::with_detail(EventKind::RankUp, new_rank.name));
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
        "  {}  Level: {}/{}   XP: {}/{}",
        color::bold_cyan("║"),
        color::bold(&level.to_string()),
        NUM_LESSONS,
        xp,
        MAX_XP
    );
    println!(
        "  {}  [{}] {}/{}",
        color::bold_cyan("║"),
        xp_bar(xp, MAX_XP, 30),
        xp,
        MAX_XP
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
        let max = stat_max(group);
        let bar = {
            let filled: Vec<String> = (0..score).map(|_| {
                if score == max { color::green("●") } else { color::yellow("●") }
            }).collect();
            let empty: Vec<String> = (0..max - score).map(|_| color::dim("○")).collect();
            [filled, empty].concat().join(" ")
        };
        let label = match score {
            n if n == max => color::green(group),
            0             => color::dim(group),
            _             => color::yellow(group),
        };
        println!("    {bar}  {label}  ({score}/{max})");
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
        let passed = is_lesson_passed(save, lesson.number);

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
    if level == NUM_LESSONS {
        println!("  {}", color::bold_cyan("╔══════════════════════════════════════════════════════════╗"));
        println!("  {}",   color::bold_cyan("║        🦀  ALL CASES CLOSED  🦀                        ║"));
        println!("  {}", color::bold_cyan("╚══════════════════════════════════════════════════════════╝"));
        println!();
        println!(
            "  {}",
            color::bold_green(&format!(
                "  You are {}. The whole hospital salutes you.",
                RANKS[RANKS.len() - 1].name
            ))
        );
        println!("  {}",   color::dim(&format!("  Check the README for suggested next topics beyond chapter {}.", NUM_LESSONS)));
    } else {
        let next = next_lesson(save);
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
            println!(
                "    View the dashboard:  {}",
                color::cyan("cargo run --bin dashboard")
            );
        }
    }

    println!();
}

// ─── Help ──────────────────────────────────────────────────────────────────

fn print_help() {
    println!();
    println!("  {}", color::bold("Rust General Hospital — Progress Tracker"));
    println!();
    println!("  {}", color::bold("USAGE:"));
    println!("    cargo run --bin progress              Normal run (incremental scan + display)");
    println!("    cargo run --bin progress -- --rescan  Re-test all {} lessons from scratch", NUM_LESSONS);
    println!("    cargo run --bin progress -- --reset   Delete save file and start fresh");
    println!("    cargo run --bin progress -- --help    Show this help message");
    println!("    cargo run --bin dashboard             Render the HTML dashboard from your save");
    println!();
    println!("  {}", color::bold("HOW IT WORKS:"));
    println!("    1. Complete exercises in src/bin/cXX_exercise.rs");
    println!("    2. Run this tracker to scan your test results");
    println!("    3. Watch your character level up as you progress!");
    println!();
    println!("  {}", color::bold("FILES:"));
    println!("    .rustacean_save.json  Your character save file (auto-created)");
    println!("                          v2 format: also records scan/pass/fail history");
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
        Some(mut s) => {
            if migrate(&mut s) {
                println!();
                println!(
                    "  {}",
                    color::dim(&format!("Save file upgraded to v{SAVE_VERSION} — history tracking enabled."))
                );
            }
            s
        }
        None => {
            let s = create_character();
            write_save(&s);
            s
        }
    };

    // Scan
    let scan_result = scan_lessons(&mut save, rescan);

    // Celebrate
    celebrate_levelup(&mut save, &scan_result);

    // Display
    display_character_sheet(&save);

    // Save
    write_save(&save);
}
