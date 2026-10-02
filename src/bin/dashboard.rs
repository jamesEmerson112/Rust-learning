// Rust General Hospital — HTML Dashboard
//
// Renders the learner's save file as a single self-contained `dashboard.html` and
// opens it in the default browser. Companion to the terminal `progress` tracker:
// `progress` shows state, this makes coming back feel like a reward.
//
// STRICTLY READ-ONLY on the save file. `progress` owns every write. If a v1 save is
// loaded we migrate the *in-memory* copy so panels render correctly, then print a
// hint that `cargo run --bin progress` is what persists the upgrade.
//
// No new crates: serde/serde_json (via tracker) + std. No network: every byte of CSS
// and every chart is inlined, hand-rolled SVG/CSS. Zero JavaScript.

#[path = "../tracker.rs"]
mod tracker;
use tracker::*;

// Dashboard-only companion data: the wider Rust landscape the 80 lessons sit inside.
// Same no-lib.rs `#[path]` convention as tracker.rs above.
#[path = "../knowledge_tree.rs"]
mod knowledge_tree;
use knowledge_tree::*;

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

const OUT_FILE: &str = "dashboard.html";
/// Weeks shown in the activity heatmap (~6 months).
const HEATMAP_WEEKS: i64 = 26;

// ─── Terminal Colors ───────────────────────────────────────────────────────
// Local mini-palette so this binary stays independent of progress.rs.

mod color {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const CYAN: &str = "\x1b[36m";
    pub fn bold(s: &str) -> String {
        format!("{BOLD}{s}{RESET}")
    }
    pub fn dim(s: &str) -> String {
        format!("{DIM}{s}{RESET}")
    }
    pub fn cyan(s: &str) -> String {
        format!("{CYAN}{s}{RESET}")
    }
    pub fn green(s: &str) -> String {
        format!("{GREEN}{s}{RESET}")
    }
    pub fn yellow(s: &str) -> String {
        format!("{YELLOW}{s}{RESET}")
    }
    pub fn bold_cyan(s: &str) -> String {
        format!("{BOLD}{CYAN}{s}{RESET}")
    }
}

/// `write!` into a String never fails — this keeps the builders readable.
macro_rules! w {
    ($dst:expr, $($arg:tt)*) => {{ let _ = write!($dst, $($arg)*); }};
}

// ─── HTML Escaping ─────────────────────────────────────────────────────────
//
// Not optional. Lesson titles are full of `Option<T>`, `Box<dyn>`, `Rc<RefCell<T>>`
// and `&[T]`, and the character handle is free text the learner typed. Unescaped,
// any of these silently eats the rest of the page.

fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 16);
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

// ─── Local Time Formatting ─────────────────────────────────────────────────
// tracker.rs gives us calendar days; we only need to add hours/minutes.

fn format_stamp(ts: u64) -> String {
    let rem = ts % 86_400;
    format!(
        "{} · {:02}:{:02} UTC",
        format_day(epoch_day(ts)),
        rem / 3600,
        (rem % 3600) / 60
    )
}

/// The moment a lesson first went green, whichever field carries it.
fn passed_at(save: &SaveFile, n: u32) -> Option<u64> {
    lesson_status(save, n).and_then(|s| s.first_passed_at.or(s.completed_at))
}

/// The learner's *working* frontier — where "continue" actually means continue.
///
/// `tracker::next_lesson` returns the first unpassed lesson, which on the current save
/// is c16: one of the nine lessons in `HOMEWORK_GAPS` that were deliberately left open
/// while the learner moved on (nothing from c55 onward depends on those nine). A
/// dashboard whose headline call-to-action sends you
/// 39 lessons backwards is worse than useless, so known-open homework is skipped here
/// and surfaced separately as its own callout, quest-log tag, and trophy.
fn frontier(save: &SaveFile) -> Option<&'static LessonMeta> {
    LESSONS
        .iter()
        .find(|l| !is_lesson_passed(save, l.number) && !HOMEWORK_GAPS.contains(&l.number))
        .or_else(|| next_lesson(save))
}

/// Deliberately-deferred lessons in c01–c54 that are still open.
fn open_homework(save: &SaveFile) -> Vec<u32> {
    HOMEWORK_GAPS
        .iter()
        .copied()
        .filter(|n| !is_lesson_passed(save, *n))
        .collect()
}

// ─── Derived History ───────────────────────────────────────────────────────
//
// THE BACKFILL RULE: entries flagged `backfilled` came from one bulk v1 rescan where
// 45 timestamps collapsed onto a single minute. They count for level, XP, stats,
// abilities, chapters and bosses — but they are poison for anything time-shaped.
// Feeding them to the heatmap renders one absurd "45 lessons in a day" cell and a
// dead grid; feeding them to pace math invents a 300-lessons-per-week velocity.
// So: excluded here, and surfaced honestly as a separate "pre-history" band.

struct History {
    /// UTC day (days since epoch) -> real passes on that day.
    day_counts: BTreeMap<i64, u32>,
    real_passes: u32,
    scans: u32,
    marathon: bool,
    comeback: bool,
    first_day: Option<i64>,
    last_day: Option<i64>,
}

impl History {
    fn is_empty(&self) -> bool {
        self.day_counts.is_empty()
    }
}

fn backfilled_lessons(save: &SaveFile) -> BTreeSet<u32> {
    (1..=NUM_LESSONS)
        .filter(|n| lesson_status(save, *n).is_some_and(|s| s.backfilled && s.passed))
        .collect()
}

fn build_history(save: &SaveFile) -> History {
    let backfilled = backfilled_lessons(save);

    let mut day_counts: BTreeMap<i64, u32> = BTreeMap::new();
    let mut real_passes = 0u32;
    let mut scans = 0u32;
    let mut regressed: BTreeSet<u32> = BTreeSet::new();
    let mut comeback = false;

    let mut events: Vec<&Event> = save.history.iter().collect();
    events.sort_by_key(|e| e.at);

    for ev in events {
        match ev.kind {
            EventKind::Scan => scans += 1,
            EventKind::Regression => {
                if let Some(n) = ev.lesson {
                    regressed.insert(n);
                }
            }
            EventKind::Pass => {
                if let Some(n) = ev.lesson {
                    // A regression seen earlier in the stream, now green again.
                    if regressed.contains(&n) {
                        comeback = true;
                    }
                    if backfilled.contains(&n) {
                        continue; // pre-history, not a real day
                    }
                }
                *day_counts.entry(epoch_day(ev.at)).or_insert(0) += 1;
                real_passes += 1;
            }
            _ => {}
        }
    }

    let marathon = day_counts.values().any(|&c| c >= 5);
    let first_day = day_counts.keys().next().copied();
    let last_day = day_counts.keys().next_back().copied();

    History {
        day_counts,
        real_passes,
        scans,
        marathon,
        comeback,
        first_day,
        last_day,
    }
}

/// (current, longest). A "day" is a UTC day with >= 1 real pass. The current streak
/// only counts if it reaches today or yesterday — otherwise it's history, not a streak.
fn streaks(hist: &History, today: i64) -> (u32, u32) {
    let days: Vec<i64> = hist.day_counts.keys().copied().collect();
    if days.is_empty() {
        return (0, 0);
    }

    let mut longest = 0u32;
    let mut run = 0u32;
    let mut prev: Option<i64> = None;
    for d in &days {
        run = if prev == Some(d - 1) { run + 1 } else { 1 };
        longest = longest.max(run);
        prev = Some(*d);
    }

    let set: BTreeSet<i64> = days.iter().copied().collect();
    let anchor = if set.contains(&today) {
        Some(today)
    } else if set.contains(&(today - 1)) {
        Some(today - 1)
    } else {
        None
    };
    let mut current = 0u32;
    if let Some(a) = anchor {
        let mut d = a;
        while set.contains(&d) {
            current += 1;
            d -= 1;
        }
    }

    (current, longest)
}

/// The bulk-rescan block: how many lessons, and when they landed.
struct PreHistory {
    lessons: Vec<u32>,
    day: Option<i64>,
}

fn pre_history(save: &SaveFile) -> PreHistory {
    let lessons: Vec<u32> = backfilled_lessons(save).into_iter().collect();
    let day = lessons
        .iter()
        .filter_map(|n| passed_at(save, *n))
        .map(epoch_day)
        .max();
    PreHistory { lessons, day }
}

// ─── Trophies ──────────────────────────────────────────────────────────────

struct Trophy {
    icon: &'static str,
    name: String,
    /// What it takes. Locked cards live or die on this line being legible.
    criterion: String,
    earned: bool,
    when: Option<u64>,
    progress: Option<(u32, u32)>,
    /// True for achievements that can only be judged from v2 event history.
    needs_history: bool,
}

fn trophy(icon: &'static str, name: &str, criterion: &str, earned: bool, when: Option<u64>) -> Trophy {
    Trophy {
        icon,
        name: name.to_string(),
        criterion: criterion.to_string(),
        earned,
        when,
        progress: None,
        needs_history: false,
    }
}

fn build_trophies(save: &SaveFile, hist: &History, current_streak: u32, longest_streak: u32) -> Vec<Trophy> {
    let level = lessons_passed(save);
    let mut out = Vec::new();

    // ── Milestones ─────────────────────────────────────────────────────────
    out.push(trophy(
        "◆",
        "First Patient",
        "Pass c01 — Hello Variables.",
        is_lesson_passed(save, 1),
        passed_at(save, 1),
    ));
    out.push(trophy(
        "◈",
        "Ownership Initiate",
        "Pass c07 — Ownership and Borrowing.",
        is_lesson_passed(save, 7),
        passed_at(save, 7),
    ));
    out.push(trophy(
        "▲",
        "Pharmacy Online",
        "Pass c74 — CSV to Sled, the pharmacy capstone.",
        is_lesson_passed(save, 74),
        passed_at(save, 74),
    ));
    out.push(trophy(
        "★",
        RANKS[RANKS.len() - 1].name,
        &format!("Pass all {NUM_LESSONS} lessons."),
        level == NUM_LESSONS,
        None,
    ));

    // ── Homework ───────────────────────────────────────────────────────────
    let hw_done = HOMEWORK_GAPS.iter().filter(|n| is_lesson_passed(save, **n)).count() as u32;
    let hw_list: Vec<String> = HOMEWORK_GAPS.iter().map(|n| format!("c{n:02}")).collect();
    let mut hw = trophy(
        "✦",
        "Homework Done",
        &format!("Close every open lesson in c01–c54: {}.", hw_list.join(", ")),
        hw_done == HOMEWORK_GAPS.len() as u32,
        HOMEWORK_GAPS.iter().filter_map(|n| passed_at(save, *n)).max(),
    );
    hw.progress = Some((hw_done, HOMEWORK_GAPS.len() as u32));
    out.push(hw);

    // ── Bug hunting ────────────────────────────────────────────────────────
    let bugs_done = BUG_LESSONS.iter().filter(|n| is_lesson_passed(save, **n)).count() as u32;
    let mut bh = trophy(
        "☣",
        "Bug Hunter",
        "Fix any 3 of the 10 ★ BUG HUNT lessons.",
        bugs_done >= 3,
        None,
    );
    bh.progress = Some((bugs_done.min(3), 3));
    out.push(bh);

    let ext_done = (75..=80).filter(|n| is_lesson_passed(save, *n)).count() as u32;
    let mut ext = trophy(
        "✚",
        "Clean Bill of Health",
        "Fix all six hospital side jobs, c75–c80.",
        ext_done == 6,
        (75..=80).filter_map(|n| passed_at(save, n)).max(),
    );
    ext.progress = Some((ext_done, 6));
    out.push(ext);

    // ── Arc Cleared ×4 ─────────────────────────────────────────────────────
    for boss in &BOSSES {
        let (done, total) = range_progress(save, boss.guards_first, boss.guards_last);
        let mut t = trophy(
            "⬡",
            &format!("Arc Cleared — {}", boss.name),
            &format!(
                "Pass every lesson in c{:02}–c{:02}.",
                boss.guards_first, boss.guards_last
            ),
            done == total,
            (boss.guards_first..=boss.guards_last)
                .filter_map(|n| passed_at(save, n))
                .max(),
        );
        t.progress = Some((done, total));
        out.push(t);
    }

    // ── History-dependent ──────────────────────────────────────────────────
    // These need v2 event data. With a freshly migrated v1 save there is nothing to
    // judge them on, so they're flagged rather than silently shown as "locked".
    let no_history = hist.is_empty() && save.history.is_empty();

    for (icon, n) in [("〓", 3u32), ("≡", 7), ("⌘", 14)] {
        let earned = longest_streak >= n;
        let mut t = trophy(
            icon,
            &format!("Streak {n}"),
            &format!("Pass at least one lesson on {n} consecutive days."),
            earned,
            None,
        );
        t.progress = Some((longest_streak.max(current_streak).min(n), n));
        t.needs_history = no_history;
        out.push(t);
    }

    let mut m = trophy(
        "⚡",
        "Marathon",
        "Pass 5 or more lessons in a single UTC day.",
        hist.marathon,
        None,
    );
    m.needs_history = no_history;
    out.push(m);

    let one_shot = LESSONS.iter().any(|l| {
        lesson_status(save, l.number)
            .is_some_and(|s| s.passed && !s.backfilled && s.attempts <= 1)
    });
    let mut os = trophy(
        "◎",
        "One-Shot",
        "Pass a lesson on the first try — no failed scan before it.",
        one_shot,
        None,
    );
    os.needs_history = no_history;
    out.push(os);

    let persistent = LESSONS.iter().any(|l| {
        lesson_status(save, l.number).is_some_and(|s| s.passed && s.attempts >= 5)
    });
    let mut p = trophy(
        "⛏",
        "Persistent",
        "Pass a lesson that took 5 or more attempts. Grinding counts.",
        persistent,
        None,
    );
    p.needs_history = no_history;
    out.push(p);

    let mut c = trophy(
        "↻",
        "Comeback",
        "Break a passing lesson, then get it green again.",
        hist.comeback,
        None,
    );
    c.needs_history = no_history;
    out.push(c);

    out
}

// ─── Pace ──────────────────────────────────────────────────────────────────

struct Pace {
    per_week: Option<f64>,
    eta_day: Option<i64>,
    span_days: Option<i64>,
    avg_attempts: Option<f64>,
    hardest: Option<(u32, u32)>, // (lesson number, attempts)
    scans: u32,
}

fn build_pace(save: &SaveFile, hist: &History, today: i64) -> Pace {
    let level = lessons_passed(save);

    // Velocity needs at least two real passes spread over at least two days.
    // One day of history would extrapolate to a fantasy velocity; better to say so.
    let (per_week, span_days) = match (hist.first_day, hist.last_day) {
        (Some(f), Some(l)) => {
            let span = (l - f + 1).max(1);
            if hist.real_passes >= 2 && span >= 2 {
                (
                    Some(hist.real_passes as f64 / span as f64 * 7.0),
                    Some(span),
                )
            } else {
                (None, Some(span))
            }
        }
        _ => (None, None),
    };

    let eta_day = per_week.and_then(|pw| {
        let remaining = NUM_LESSONS.saturating_sub(level);
        if remaining == 0 || pw <= 0.0 {
            return None;
        }
        let days = (remaining as f64 / pw * 7.0).ceil();
        if !days.is_finite() || days > 36_500.0 {
            return None;
        }
        Some(today + days as i64)
    });

    let attempted: Vec<u32> = LESSONS
        .iter()
        .filter_map(|l| lesson_status(save, l.number))
        .filter(|s| s.passed && s.attempts > 0)
        .map(|s| s.attempts)
        .collect();
    let avg_attempts = if attempted.is_empty() {
        None
    } else {
        Some(attempted.iter().sum::<u32>() as f64 / attempted.len() as f64)
    };

    let hardest = LESSONS
        .iter()
        .filter_map(|l| lesson_status(save, l.number).map(|s| (l.number, s.attempts)))
        .filter(|(_, a)| *a > 1)
        .max_by_key(|(n, a)| (*a, std::cmp::Reverse(*n)));

    Pace {
        per_week,
        eta_day,
        span_days,
        avg_attempts,
        hardest,
        scans: hist.scans,
    }
}

// ─── Styles ────────────────────────────────────────────────────────────────

const CSS: &str = r##"
*,*::before,*::after{box-sizing:border-box}
:root{
  --bg:#05080b; --panel:#0b1118; --panel2:#0e1620; --sunk:#070c11;
  --line:#1a2531; --line2:#26374a;
  --txt:#c2d2e2; --dim:#67788c; --dimmer:#3b4a5b;
  --cy:#2ee6f6; --mg:#ff45c8; --gr:#3ef2a1; --ye:#ffd447; --rd:#ff5f74;
  --s1:6px; --s2:10px; --s3:16px; --s4:24px; --s5:36px;
  --mono:ui-monospace,"Cascadia Code","JetBrains Mono","Fira Code",Consolas,"SF Mono",Menlo,monospace;
}
html{-webkit-text-size-adjust:100%}
body{
  margin:0; background:var(--bg); color:var(--txt);
  font-family:var(--mono); font-size:14px; line-height:1.5;
  background-image:
    radial-gradient(900px 500px at 15% -10%, rgba(46,230,246,.07), transparent 60%),
    radial-gradient(800px 500px at 95% 5%, rgba(255,69,200,.06), transparent 60%),
    linear-gradient(rgba(255,255,255,.014) 1px, transparent 1px),
    linear-gradient(90deg, rgba(255,255,255,.014) 1px, transparent 1px);
  background-size:auto,auto,44px 44px,44px 44px;
  background-attachment:fixed;
}
body::after{
  content:""; position:fixed; inset:0; pointer-events:none; z-index:99;
  background:repeating-linear-gradient(180deg, rgba(255,255,255,.020) 0 1px, transparent 1px 3px);
}
a{color:var(--cy)}
h1,h2,h3{margin:0; font-weight:700; letter-spacing:.06em}

.wrap{max-width:1240px; margin:0 auto; padding:var(--s4) var(--s3) var(--s5)}
.grid{display:grid; grid-template-columns:repeat(12,1fr); gap:var(--s3)}
.c12{grid-column:span 12}.c8{grid-column:span 8}.c7{grid-column:span 7}
.c6{grid-column:span 6}.c5{grid-column:span 5}.c4{grid-column:span 4}
@media(max-width:980px){.c8,.c7,.c6,.c5,.c4{grid-column:span 12}}

/* ── Cards ───────────────────────────────────────────────────────── */
.card{
  position:relative; background:linear-gradient(180deg,var(--panel2),var(--panel));
  border:1px solid var(--line); border-radius:10px; padding:var(--s3) var(--s3) var(--s4);
}
.card::before{
  content:""; position:absolute; left:14px; right:14px; top:-1px; height:1px;
  background:linear-gradient(90deg,transparent,rgba(46,230,246,.55),transparent);
}
.card>h2{
  font-size:11px; letter-spacing:.22em; color:var(--cy); text-transform:uppercase;
  padding-bottom:var(--s2); margin-bottom:var(--s3);
  border-bottom:1px solid var(--line); display:flex; align-items:baseline; gap:var(--s2);
  flex-wrap:wrap;
}
.card>h2 .sub{color:var(--dimmer); letter-spacing:.1em; text-transform:none; font-size:10px; margin-left:auto}
.note{font-size:11px; color:var(--dimmer); line-height:1.6; margin-top:var(--s3)}
.note b{color:var(--dim); font-weight:600}

/* ── Hero ────────────────────────────────────────────────────────── */
.hero{
  display:flex; gap:var(--s4); align-items:center; flex-wrap:wrap;
  margin-bottom:var(--s3);
  background:
    linear-gradient(180deg,#0f1a24,#0a1017),
    radial-gradient(600px 200px at 20% 0%, rgba(255,69,200,.10), transparent 70%);
  border-color:var(--line2);
}
.hero-id{flex:1 1 320px; min-width:250px}
.eyebrow{font-size:10px; letter-spacing:.34em; color:var(--dimmer); text-transform:uppercase}
.handle{font-size:clamp(24px,4.4vw,38px); line-height:1.15; color:#eaf4ff; letter-spacing:.02em; margin:var(--s1) 0 2px; text-shadow:0 0 24px rgba(46,230,246,.28)}
.handle .cls{color:var(--dim); font-size:.45em; letter-spacing:.16em; display:block; margin-top:6px}
.rankline{margin-top:var(--s3); font-size:16px; color:var(--mg); letter-spacing:.14em; text-shadow:0 0 18px rgba(255,69,200,.35)}
.quote{color:var(--dim); font-size:12.5px; font-style:italic; margin-top:var(--s1); max-width:52ch}
.hero-num{display:flex; gap:var(--s4); align-items:center; flex-wrap:wrap}
.ring-wrap{position:relative; width:150px; height:150px; flex:none}
.ring-txt{position:absolute; inset:0; display:flex; flex-direction:column; align-items:center; justify-content:center}
.ring-txt b{font-size:32px; color:#eaf4ff; line-height:1}
.ring-txt span{font-size:10px; letter-spacing:.2em; color:var(--dimmer)}
.hero-meta{flex:1 1 230px; min-width:200px}
.kv{display:flex; justify-content:space-between; gap:var(--s3); font-size:12px; padding:5px 0; border-bottom:1px dashed var(--line)}
.kv:last-child{border-bottom:0}
.kv span{color:var(--dim)}
.kv b{color:var(--txt); font-weight:600}
.xpbar{height:9px; border-radius:5px; background:var(--sunk); border:1px solid var(--line); overflow:hidden; margin-top:var(--s2)}
.xpbar i{display:block; height:100%; background:linear-gradient(90deg,var(--cy),var(--gr)); box-shadow:0 0 14px rgba(62,242,161,.5)}

/* ── Generic bits ────────────────────────────────────────────────── */
.tag{font-size:9.5px; letter-spacing:.14em; padding:2px 7px; border-radius:4px; border:1px solid currentColor; white-space:nowrap}
.t-pass{color:var(--gr)} .t-next{color:var(--ye)} .t-lock{color:var(--dimmer)} .t-mag{color:var(--mg)}
.t-open{color:#c08a3e}
.scrollx{overflow-x:auto; overflow-y:hidden; padding-bottom:var(--s2)}
.scrollx::-webkit-scrollbar{height:8px}
.scrollx::-webkit-scrollbar-thumb{background:var(--line2); border-radius:4px}
.scrollx::-webkit-scrollbar-track{background:var(--sunk)}
code{font-family:var(--mono)}

/* ── Chapter map ─────────────────────────────────────────────────── */
/* padding-top gives the YOU badge room: .scrollx clips overflow-y. */
.map{display:flex; align-items:stretch; gap:0; min-width:min-content; padding-top:11px}
.node{
  flex:1 0 124px; background:var(--sunk); border:1px solid var(--line); border-radius:8px;
  padding:var(--s2); position:relative;
}
.node.here{border-color:var(--ye); box-shadow:0 0 0 1px rgba(255,212,71,.28), 0 0 22px rgba(255,212,71,.14)}
.node.full{border-color:rgba(62,242,161,.5)}
.node .idx{font-size:9px; color:var(--dimmer); letter-spacing:.2em}
.node .nm{font-size:12px; color:var(--txt); margin:3px 0 1px; line-height:1.25}
.node.full .nm{color:var(--gr)}
.node .st{font-size:10px; color:var(--dim); line-height:1.35; min-height:2.7em}
.node .rg{font-size:9.5px; color:var(--dimmer); margin-top:4px}
.node .mini{height:5px; border-radius:3px; background:#0d141b; border:1px solid var(--line); overflow:hidden; margin-top:6px}
.node .mini i{display:block; height:100%}
.node .pct{font-size:10px; margin-top:4px; color:var(--dim)}
.node .youarehere{position:absolute; top:-9px; right:8px; font-size:9px; letter-spacing:.12em; color:#0a0f14; background:var(--ye); padding:1px 6px; border-radius:3px}
.link{flex:0 0 20px; align-self:center; height:1px; background:linear-gradient(90deg,var(--line2),var(--line2)); position:relative}
.link.on{background:linear-gradient(90deg,var(--gr),rgba(62,242,161,.35))}
.link::after{content:""; position:absolute; right:0; top:-3px; border-left:6px solid var(--line2); border-top:3.5px solid transparent; border-bottom:3.5px solid transparent}
.link.on::after{border-left-color:rgba(62,242,161,.55)}

/* ── Next step ───────────────────────────────────────────────────── */
.next-hd{font-size:clamp(17px,2.4vw,21px); color:var(--ye); letter-spacing:.02em; text-shadow:0 0 18px rgba(255,212,71,.25)}
.next-sub{color:var(--dim); font-size:12px; margin:4px 0 var(--s3)}
.cmd{margin-bottom:var(--s2)}
.cmd label{display:block; font-size:9.5px; letter-spacing:.18em; color:var(--dimmer); margin-bottom:4px; text-transform:uppercase}
.cmd code{
  display:block; background:var(--sunk); border:1px solid var(--line); border-left:2px solid var(--cy);
  border-radius:5px; padding:8px 10px; font-size:12.5px; color:var(--cy);
  overflow-x:auto; white-space:pre; user-select:all;
}
.endgame{text-align:center; padding:var(--s4) var(--s3)}
.endgame .big{font-size:clamp(20px,3.6vw,30px); color:var(--gr); letter-spacing:.14em; text-shadow:0 0 26px rgba(62,242,161,.4)}

/* ── Stat grid ───────────────────────────────────────────────────── */
.stats{display:grid; grid-template-columns:repeat(auto-fill,minmax(196px,1fr)); gap:var(--s2) var(--s3)}
.stat{border-bottom:1px dashed var(--line); padding-bottom:6px}
.stat .lbl{font-size:11.5px; display:flex; justify-content:space-between; gap:var(--s2)}
.stat .num{color:var(--dimmer); font-size:10px}
.dots{letter-spacing:.22em; font-size:13px; line-height:1.2; margin-top:2px}
.s-full .lbl{color:var(--gr)} .s-part .lbl{color:var(--ye)} .s-zero .lbl{color:var(--dimmer)}
.s-full .dots{color:var(--gr); text-shadow:0 0 8px rgba(62,242,161,.4)}
.s-part .dots{color:var(--ye)} .s-zero .dots{color:#33455a}
.dots .off{color:#33455a; text-shadow:none}

/* ── Heatmap ─────────────────────────────────────────────────────── */
.hm-wrap{display:inline-block; min-width:min-content}
.hm-months{display:grid; height:15px; font-size:9.5px; color:var(--dim); letter-spacing:.1em; margin-left:26px}
.hm-row{display:flex; gap:4px}
.hm-dow{display:grid; grid-template-rows:repeat(7,12px); gap:3px; width:22px; font-size:8.5px; color:var(--dimmer); align-items:center}
.hm-grid{display:grid; grid-template-rows:repeat(7,12px); grid-auto-flow:column; gap:3px}
.hm-grid i{display:block; width:12px; height:12px; border-radius:2.5px; background:#15202b; border:1px solid rgba(255,255,255,.045)}
.hm-grid i.l1{background:#14513a} .hm-grid i.l2{background:#1c8058}
.hm-grid i.l3{background:#2ac07c} .hm-grid i.l4{background:var(--gr); box-shadow:0 0 8px rgba(62,242,161,.5)}
.hm-grid i.fut{background:transparent; border-color:transparent}
.hm-legend{display:flex; align-items:center; gap:5px; font-size:10px; color:var(--dimmer); margin-top:var(--s2)}
.hm-legend i{display:inline-block; width:11px; height:11px; border-radius:2.5px; background:#15202b; border:1px solid rgba(255,255,255,.045)}
.streaks{display:flex; gap:var(--s3); flex-wrap:wrap; margin-bottom:var(--s3)}
.streak{background:var(--sunk); border:1px solid var(--line); border-radius:8px; padding:var(--s2) var(--s3); min-width:120px}
.streak b{display:block; font-size:24px; color:var(--gr); line-height:1.1}
.streak.cold b{color:var(--dimmer)}
.streak span{font-size:9.5px; letter-spacing:.16em; color:var(--dimmer); text-transform:uppercase}

/* ── Pre-history band ────────────────────────────────────────────── */
.prehist{
  margin-top:var(--s3); border:1px dashed rgba(255,69,200,.35); border-radius:8px;
  padding:var(--s2) var(--s3); background:rgba(255,69,200,.045);
}
.prehist h3{font-size:10px; letter-spacing:.2em; color:var(--mg); text-transform:uppercase}
.prehist p{margin:5px 0 var(--s2); font-size:11px; color:var(--dim); line-height:1.55}
.prehist .blocks{display:flex; flex-wrap:wrap; gap:3px}
.prehist .blocks i{width:11px; height:11px; border-radius:2px; background:rgba(255,69,200,.45); display:block}

/* ── Bosses ──────────────────────────────────────────────────────── */
.boss{background:var(--sunk); border:1px solid var(--line); border-radius:8px; padding:var(--s2) var(--s3); margin-bottom:var(--s2)}
.boss:last-child{margin-bottom:0}
.boss-hd{display:flex; align-items:baseline; gap:var(--s2); flex-wrap:wrap}
.boss-nm{font-size:13px; letter-spacing:.1em; color:var(--rd)}
.boss.dead .boss-nm{color:var(--gr)}
.boss .tl{font-size:11px; color:var(--dim); margin:3px 0 7px}
.hp{height:12px; border-radius:3px; background:#0c1319; border:1px solid var(--line); overflow:hidden; position:relative}
.hp i{display:block; height:100%; background:linear-gradient(90deg,var(--rd),#ff9a5a)}
.boss.dead .hp i{background:linear-gradient(90deg,#1c4a38,#2a6a50)}
.hp-lbl{display:flex; justify-content:space-between; font-size:10px; color:var(--dimmer); margin-top:4px}

/* ── Pace tiles ──────────────────────────────────────────────────── */
.tiles{display:grid; grid-template-columns:repeat(auto-fit,minmax(150px,1fr)); gap:var(--s2)}
.tile{background:var(--sunk); border:1px solid var(--line); border-radius:8px; padding:var(--s2) var(--s3)}
.tile span{display:block; font-size:9.5px; letter-spacing:.15em; color:var(--dimmer); text-transform:uppercase}
.tile b{display:block; font-size:21px; color:var(--cy); line-height:1.25; margin-top:3px; overflow-wrap:anywhere}
.tile b.na{color:var(--dimmer)}
.tile em{display:block; font-style:normal; font-size:10px; color:var(--dim); margin-top:2px}

/* ── Trophies ────────────────────────────────────────────────────── */
.trophies{display:grid; grid-template-columns:repeat(auto-fill,minmax(230px,1fr)); gap:var(--s2)}
.tr{background:var(--sunk); border:1px solid var(--line); border-radius:8px; padding:var(--s2) var(--s3); display:flex; gap:var(--s2)}
.tr .ic{font-size:17px; color:var(--dimmer); line-height:1.3; flex:none}
.tr .nm{font-size:12.5px; color:var(--dim)}
.tr .cr{font-size:10.5px; color:var(--dimmer); line-height:1.5; margin-top:2px}
.tr.got{border-color:rgba(255,212,71,.45); background:linear-gradient(180deg,rgba(255,212,71,.07),rgba(255,212,71,.02))}
.tr.got .ic{color:var(--ye); text-shadow:0 0 12px rgba(255,212,71,.5)}
.tr.got .nm{color:var(--ye)}
.tr.got .cr{color:var(--dim)}
.tr .prog{margin-top:5px; height:4px; border-radius:2px; background:#0d141b; border:1px solid var(--line); overflow:hidden}
.tr .prog i{display:block; height:100%; background:var(--ye)}
.tr .meta{font-size:9.5px; color:var(--dimmer); margin-top:4px; letter-spacing:.06em}
.tr .v2{color:var(--mg)}

/* ── Quest log ───────────────────────────────────────────────────── */
.chap{margin-bottom:var(--s3)}
.chap-hd{display:flex; align-items:baseline; gap:var(--s2); font-size:11px; letter-spacing:.16em;
  color:var(--dim); text-transform:uppercase; border-bottom:1px solid var(--line); padding-bottom:5px; margin-bottom:var(--s2); flex-wrap:wrap}
.chap-hd .n{color:var(--txt)}
.chap-hd .s{color:var(--dimmer); letter-spacing:.06em; text-transform:none; font-size:10.5px}
.chap-hd .p{margin-left:auto; color:var(--dimmer); font-size:10px}
.qgrid{display:grid; grid-template-columns:repeat(auto-fill,minmax(255px,1fr)); gap:2px var(--s3)}
.q{display:flex; align-items:baseline; gap:7px; font-size:11.5px; padding:2px 0; min-width:0}
.q .id{color:var(--dimmer); flex:none}
.q .ti{overflow:hidden; text-overflow:ellipsis; white-space:nowrap}
.q .at{color:var(--mg); font-size:9.5px; flex:none}
.q.pass .ti{color:var(--gr)} .q.next .ti{color:var(--ye)} .q.lock .ti{color:var(--dimmer)}
.q.open .ti{color:#c08a3e}
.q.next{background:rgba(255,212,71,.07); border-radius:3px; padding-left:4px; margin-left:-4px}
.q .bug{color:var(--mg); font-size:9px; flex:none}

/* ── Knowledge tree ──────────────────────────────────────────────── */
/* Native details/summary disclosure only — the page ships zero JavaScript. */
.kt-root{border-top:1px solid var(--line); padding-top:var(--s2); margin-top:var(--s2)}
.kt-root:first-child{border-top:0; padding-top:0; margin-top:0}
.kt-kids{border-left:1px solid var(--line); margin-left:8px; padding-left:12px}
.kt-sum{cursor:pointer; list-style:none; display:flex; align-items:baseline; gap:7px;
  flex-wrap:wrap; padding:3px 0; min-width:0}
.kt-sum::-webkit-details-marker{display:none}
.kt-sum::marker{content:""}
.kt-sum::before{content:"▸"; color:var(--dimmer); font-size:10px; flex:none; width:10px; transition:none}
details[open]>.kt-sum::before{content:"▾"; color:var(--cy)}
.kt-sum:hover{background:rgba(46,230,246,.05); border-radius:3px}
/* 17px = the summary's 10px marker + its 7px gap, so leaf dots line up with branch dots. */
.kt-leaf{display:flex; align-items:baseline; gap:7px; flex-wrap:wrap; padding:3px 0 3px 17px; min-width:0}
.kt-nm{font-size:12px; color:var(--txt)}
.kt-blurb{font-size:10.5px; color:var(--dimmer); line-height:1.55; flex:1 1 340px; min-width:0}
.kt-count{margin-left:auto; font-size:9.5px; letter-spacing:.1em; color:var(--dimmer); white-space:nowrap; flex:none}
.kt-count b{font-weight:600}
.kt-count .kl{color:var(--gr)} .kt-count .kq{color:var(--ye)} .kt-count .ku{color:var(--dim)}
.kt-dot{flex:none; font-size:9px; line-height:1.6}
.kt-cells{font-size:9px; letter-spacing:.08em; color:var(--gr); flex:none; opacity:.85}
.kt-learned>.kt-sum .kt-nm,.kt-learned.kt-leaf .kt-nm{color:var(--gr)}
.kt-learned>.kt-sum .kt-dot,.kt-learned.kt-leaf .kt-dot{color:var(--gr)}
.kt-queued>.kt-sum .kt-nm,.kt-queued.kt-leaf .kt-nm{color:var(--ye)}
.kt-queued>.kt-sum .kt-dot,.kt-queued.kt-leaf .kt-dot{color:var(--ye)}
.kt-open>.kt-sum .kt-nm,.kt-open.kt-leaf .kt-nm{color:var(--dim)}
.kt-open>.kt-sum .kt-dot,.kt-open.kt-leaf .kt-dot{color:var(--dimmer)}
/* Roots last: the eight branch headers stay bright whatever their own state is. */
.kt-root>.kt-sum .kt-nm{font-size:12.5px; letter-spacing:.06em; color:#eaf4ff}
.kt-root>.kt-sum .kt-dot{color:var(--cy)}
.kt-root>.kt-sum .kt-blurb{color:var(--dim)}
.kt-key{display:flex; gap:var(--s3); flex-wrap:wrap; font-size:10px; color:var(--dimmer); margin-bottom:var(--s2)}
.kt-key span{white-space:nowrap}
@media(max-width:560px){
  .kt-blurb{flex-basis:100%}
  .kt-count{margin-left:0}
}

/* ── Footer ──────────────────────────────────────────────────────── */
footer{margin-top:var(--s4); padding-top:var(--s3); border-top:1px solid var(--line);
  font-size:10.5px; color:var(--dimmer); display:flex; justify-content:space-between; gap:var(--s3); flex-wrap:wrap}

@media(max-width:560px){
  .wrap{padding:var(--s3) var(--s2) var(--s4)}
  .card{padding:var(--s3) var(--s2) var(--s3)}
  .hero{gap:var(--s3)}
  .qgrid{grid-template-columns:1fr}
}
"##;

// ─── SVG: Level Ring ───────────────────────────────────────────────────────

/// Linear interpolation between two RGB triples, emitted as `#rrggbb`.
///
/// Used instead of an SVG `<linearGradient>` because referencing one requires
/// `fill="url(#id)"`, and this page holds a hard "no `url(` anywhere" line so the
/// offline-safety grep stays trivially auditable. Lerped segments get the same look
/// with nothing to resolve.
fn lerp_hex(a: (u8, u8, u8), b: (u8, u8, u8), t: f64) -> String {
    let t = t.clamp(0.0, 1.0);
    let mix = |x: u8, y: u8| (x as f64 + (y as f64 - x as f64) * t).round() as u8;
    format!("#{:02x}{:02x}{:02x}", mix(a.0, b.0), mix(a.1, b.1), mix(a.2, b.2))
}

const CY_RGB: (u8, u8, u8) = (0x2e, 0xe6, 0xf6);
const GR_RGB: (u8, u8, u8) = (0x3e, 0xf2, 0xa1);

fn ring_svg(level: u32) -> String {
    use std::f64::consts::{FRAC_PI_2, TAU};
    let frac = level as f64 / NUM_LESSONS as f64;
    let (cx, cy, r) = (75.0_f64, 75.0_f64, 62.0_f64);

    let mut s = String::new();
    w!(
        s,
        r#"<svg viewBox="0 0 150 150" width="150" height="150" role="img" aria-label="Level {level} of {NUM_LESSONS}">"#
    );
    w!(
        s,
        r##"<circle cx="75" cy="75" r="{r}" fill="none" stroke="#16202b" stroke-width="9"/>"##
    );

    // Tick marks: one every ten lessons.
    for i in 0..8 {
        let ang = (i as f64 / 8.0) * TAU - FRAC_PI_2;
        let (x1, y1) = (cx + ang.cos() * 70.0, cy + ang.sin() * 70.0);
        let (x2, y2) = (cx + ang.cos() * 74.0, cy + ang.sin() * 74.0);
        w!(
            s,
            r##"<line x1="{x1:.1}" y1="{y1:.1}" x2="{x2:.1}" y2="{y2:.1}" stroke="#26374a" stroke-width="1.5"/>"##
        );
    }

    // Progress arc, drawn as colour-lerped segments (cyan -> green).
    if frac > 0.0 {
        const SEG: usize = 48;
        let sweep = frac * TAU;
        let at = |a: f64| (cx + a.cos() * r, cy + a.sin() * r);
        for k in 0..SEG {
            let a0 = -FRAC_PI_2 + sweep * (k as f64 / SEG as f64);
            let a1 = -FRAC_PI_2 + sweep * ((k + 1) as f64 / SEG as f64);
            let (x0, y0) = at(a0);
            let (x1, y1) = at(a1);
            let col = lerp_hex(CY_RGB, GR_RGB, k as f64 / (SEG - 1) as f64);
            w!(
                s,
                r#"<path d="M{x0:.2} {y0:.2} A{r} {r} 0 0 1 {x1:.2} {y1:.2}" fill="none" stroke="{col}" stroke-width="9" stroke-linecap="round"/>"#
            );
        }
    }

    w!(s, "</svg>");
    s
}

// ─── SVG: Chapter Radar ────────────────────────────────────────────────────

fn radar_svg(save: &SaveFile) -> String {
    let cx = 200.0_f64;
    let cy = 176.0_f64;
    let r = 108.0_f64;
    let n = CHAPTERS.len();

    let vertex = |i: usize, frac: f64| -> (f64, f64) {
        let ang = (i as f64 / n as f64) * std::f64::consts::TAU - std::f64::consts::FRAC_PI_2;
        (cx + ang.cos() * r * frac, cy + ang.sin() * r * frac)
    };

    let mut s = String::new();
    w!(
        s,
        r#"<svg viewBox="0 0 400 358" style="width:100%;height:auto;display:block" role="img" aria-label="Completion by chapter">"#
    );
    w!(
        s,
        r#"<title>Completion by chapter</title>"#
    );

    // Concentric grid rings at 25 / 50 / 75 / 100 %.
    for step in 1..=4 {
        let frac = step as f64 / 4.0;
        let pts: Vec<String> = (0..n)
            .map(|i| {
                let (x, y) = vertex(i, frac);
                format!("{x:.1},{y:.1}")
            })
            .collect();
        let stroke = if step == 4 { "#2b3d51" } else { "#1b2836" };
        w!(
            s,
            r#"<polygon points="{}" fill="none" stroke="{stroke}" stroke-width="1"/>"#,
            pts.join(" ")
        );
    }

    // Spokes.
    for i in 0..n {
        let (x, y) = vertex(i, 1.0);
        w!(
            s,
            r##"<line x1="{cx}" y1="{cy}" x2="{x:.1}" y2="{y:.1}" stroke="#1b2836" stroke-width="1"/>"##
        );
    }

    // Data polygon.
    let fracs: Vec<f64> = CHAPTERS
        .iter()
        .map(|c| {
            let (done, total) = range_progress(save, c.first, c.last);
            if total == 0 { 0.0 } else { done as f64 / total as f64 }
        })
        .collect();
    let pts: Vec<String> = fracs
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let (x, y) = vertex(i, *f);
            format!("{x:.1},{y:.1}")
        })
        .collect();
    w!(
        s,
        r##"<polygon points="{}" fill="#2ee6f6" fill-opacity=".17" stroke="#2ee6f6" stroke-width="2" stroke-linejoin="round"/>"##,
        pts.join(" ")
    );

    // Vertex dots — zero axes collapse to the centre, which reads as "unexplored".
    for (i, f) in fracs.iter().enumerate() {
        let (x, y) = vertex(i, *f);
        let fill = if *f >= 1.0 {
            "#3ef2a1"
        } else if *f > 0.0 {
            "#2ee6f6"
        } else {
            "#3b4a5b"
        };
        w!(
            s,
            r#"<circle cx="{x:.1}" cy="{y:.1}" r="3" fill="{fill}"/>"#
        );
    }

    // Axis labels.
    for (i, chap) in CHAPTERS.iter().enumerate() {
        let ang = (i as f64 / n as f64) * std::f64::consts::TAU - std::f64::consts::FRAC_PI_2;
        let lx = cx + ang.cos() * (r + 26.0);
        let ly = cy + ang.sin() * (r + 26.0);
        let anchor = if ang.cos() > 0.15 {
            "start"
        } else if ang.cos() < -0.15 {
            "end"
        } else {
            "middle"
        };
        let dy = if ang.sin() < -0.7 {
            -4.0
        } else if ang.sin() > 0.7 {
            10.0
        } else {
            2.0
        };
        let (done, total) = range_progress(save, chap.first, chap.last);
        let col = if done == total { "#3ef2a1" } else if done > 0 { "#c2d2e2" } else { "#4d5f72" };
        w!(
            s,
            r#"<text x="{lx:.1}" y="{:.1}" text-anchor="{anchor}" font-family="ui-monospace,monospace" font-size="11" fill="{col}" letter-spacing=".04em">{}</text>"#,
            ly + dy,
            esc(chap.name)
        );
        w!(
            s,
            r##"<text x="{lx:.1}" y="{:.1}" text-anchor="{anchor}" font-family="ui-monospace,monospace" font-size="9.5" fill="#4d5f72">{done}/{total}</text>"##,
            ly + dy + 12.0
        );
    }

    w!(s, "</svg>");
    s
}

// ─── Panel: Hero ───────────────────────────────────────────────────────────

fn panel_hero(save: &SaveFile) -> String {
    let level = lessons_passed(save);
    let xp = level * 100;
    let rank = get_rank(level);
    let next_rank = RANKS.iter().find(|r| r.min_level > level);
    let pct = (level as f64 / NUM_LESSONS as f64 * 100.0).round() as u32;

    let mut s = String::new();
    w!(s, r#"<header class="card hero">"#);

    w!(s, r#"<div class="hero-id">"#);
    w!(s, r#"<div class="eyebrow">Rust General Hospital // staff record</div>"#);
    w!(
        s,
        r#"<h1 class="handle">{}<span class="cls">the {}</span></h1>"#,
        esc(&save.character.name),
        esc(&save.character.class)
    );
    w!(s, r#"<div class="rankline">{}</div>"#, esc(&rank.name.to_uppercase()));
    w!(s, r#"<div class="quote">&ldquo;{}&rdquo;</div>"#, esc(rank.quote));
    w!(s, "</div>");

    w!(s, r#"<div class="hero-num">"#);
    w!(s, r#"<div class="ring-wrap">{}"#, ring_svg(level));
    w!(
        s,
        r#"<div class="ring-txt"><b>{level}</b><span>LVL / {NUM_LESSONS}</span></div></div>"#
    );

    w!(s, r#"<div class="hero-meta">"#);
    w!(
        s,
        r#"<div class="kv"><span>XP</span><b>{xp} / {MAX_XP}</b></div>"#
    );
    w!(
        s,
        r#"<div class="xpbar"><i style="width:{}%"></i></div>"#,
        format!("{:.2}", xp as f64 / MAX_XP as f64 * 100.0)
    );
    w!(
        s,
        r#"<div class="kv" style="margin-top:10px"><span>Cases closed</span><b>{pct}%</b></div>"#
    );
    match next_rank {
        Some(nr) => {
            let need = nr.min_level - level;
            w!(
                s,
                r#"<div class="kv"><span>Next rank</span><b>{}</b></div>"#,
                esc(nr.name)
            );
            w!(
                s,
                r#"<div class="kv"><span>Lessons to rank</span><b class="t-next">{need}</b></div>"#
            );
        }
        None => {
            w!(
                s,
                r#"<div class="kv"><span>Next rank</span><b class="t-pass">MAX &mdash; top of the ladder</b></div>"#
            );
        }
    }
    w!(
        s,
        r#"<div class="kv"><span>Last scan</span><b>{}</b></div>"#,
        esc(&format_stamp(save.last_scan))
    );
    w!(s, "</div></div></header>");
    s
}

// ─── Panel: Next Step ──────────────────────────────────────────────────────

fn panel_next(save: &SaveFile) -> String {
    let mut s = String::new();
    w!(s, r#"<section class="card c5">"#);
    w!(s, r#"<h2>Next Step<span class="sub">frontier</span></h2>"#);

    match frontier(save) {
        None => {
            w!(s, r#"<div class="endgame">"#);
            w!(s, r#"<div class="big">ALL CASES CLOSED</div>"#);
            w!(
                s,
                r#"<p class="next-sub" style="margin-top:12px">All {NUM_LESSONS} lessons passed. You are <b style="color:var(--mg)">{}</b> &mdash; the whole hospital salutes you.</p>"#,
                esc(RANKS[RANKS.len() - 1].name)
            );
            w!(
                s,
                r#"<p class="note">Check the README for suggested topics beyond chapter {NUM_LESSONS}.</p>"#
            );
            w!(s, "</div>");
        }
        Some(lesson) => {
            let n = lesson.number;
            let is_bug = BUG_LESSONS.contains(&n);
            let chap = CHAPTERS
                .iter()
                .find(|c| n >= c.first && n <= c.last)
                .map(|c| c.name)
                .unwrap_or("");
            w!(
                s,
                r#"<div class="eyebrow">c{n:02} &middot; {}</div>"#,
                esc(chap)
            );
            w!(s, r#"<div class="next-hd">{}</div>"#, esc(lesson.title));
            w!(
                s,
                r#"<div class="next-sub">{}{}</div>"#,
                esc(lesson.stat_group),
                if is_bug {
                    r#" &middot; <span class="t-mag">★ BUG HUNT &mdash; the code compiles, the tests don&#39;t</span>"#
                } else {
                    ""
                }
            );
            for (label, cmd) in [
                ("Read the example", format!("cargo run --bin c{n:02}_example")),
                ("Edit the exercise", format!("src/bin/c{n:02}_exercise.rs")),
                ("Test your solution", format!("cargo test --test c{n:02}_tests")),
                ("Then come back", "cargo run --bin progress".to_string()),
            ] {
                w!(
                    s,
                    r#"<div class="cmd"><label>{label}</label><code>{}</code></div>"#,
                    esc(&cmd)
                );
            }
            if let Some(ability) = ABILITIES.iter().find(|a| a.lesson == n) {
                w!(
                    s,
                    r#"<p class="note"><b>Unlocks:</b> {} &mdash; {}</p>"#,
                    esc(ability.name),
                    esc(ability.description)
                );
            }

            // Known-open homework sits *behind* the frontier — worth naming, not worth
            // hijacking the call-to-action for.
            let hw = open_homework(save);
            if !hw.is_empty() {
                let list: Vec<String> = hw.iter().map(|n| format!("c{n:02}")).collect();
                w!(
                    s,
                    r#"<p class="note"><span class="tag t-open">OPEN</span> <b>{} earlier lesson{}</b> still unfinished: {}. Nothing ahead depends on them &mdash; the <b>Homework Done</b> trophy is waiting whenever you circle back.</p>"#,
                    hw.len(),
                    if hw.len() == 1 { "" } else { "s" },
                    esc(&list.join(", "))
                );
            }
        }
    }
    w!(s, "</section>");
    s
}

// ─── Panel: Chapter Map ────────────────────────────────────────────────────

fn panel_map(save: &SaveFile) -> String {
    let next = frontier(save).map(|l| l.number);
    let here = next.and_then(|n| CHAPTERS.iter().position(|c| n >= c.first && n <= c.last));

    let mut s = String::new();
    w!(s, r#"<section class="card c12">"#);
    w!(
        s,
        r#"<h2>Chapter Map<span class="sub">reading order &rarr;</span></h2>"#
    );
    w!(s, r#"<div class="scrollx"><div class="map">"#);

    for (i, chap) in CHAPTERS.iter().enumerate() {
        if i > 0 {
            let prev_full = {
                let (d, t) = range_progress(save, CHAPTERS[i - 1].first, CHAPTERS[i - 1].last);
                d == t
            };
            w!(
                s,
                r#"<div class="link{}"></div>"#,
                if prev_full { " on" } else { "" }
            );
        }
        let (done, total) = range_progress(save, chap.first, chap.last);
        let pct = if total == 0 { 0 } else { done * 100 / total };
        let full = done == total;
        let cls = if here == Some(i) {
            " here"
        } else if full {
            " full"
        } else {
            ""
        };
        let fill = if full {
            "var(--gr)"
        } else if done > 0 {
            "var(--ye)"
        } else {
            "transparent"
        };
        w!(s, r#"<div class="node{cls}">"#);
        if here == Some(i) {
            w!(s, r#"<div class="youarehere">YOU</div>"#);
        }
        w!(s, r#"<div class="idx">CH {:02}</div>"#, i + 1);
        w!(s, r#"<div class="nm">{}</div>"#, esc(chap.name));
        w!(s, r#"<div class="st">{}</div>"#, esc(chap.subtitle));
        w!(
            s,
            r#"<div class="rg">c{:02}&ndash;c{:02}</div>"#,
            chap.first, chap.last
        );
        w!(
            s,
            r#"<div class="mini"><i style="width:{pct}%;background:{fill}"></i></div>"#
        );
        w!(s, r#"<div class="pct">{done}/{total} &middot; {pct}%</div>"#);
        w!(s, "</div>");
    }
    w!(s, "</div></div>");

    if let Some(n) = next {
        let lesson = &LESSONS[(n - 1) as usize];
        w!(
            s,
            r#"<p class="note"><span class="tag t-next">NEXT</span> &nbsp;<b>c{n:02} {}</b> &mdash; your position on the map. Everything to its right is still dark.</p>"#,
            esc(lesson.title)
        );
    } else {
        w!(
            s,
            r#"<p class="note">Every chapter is lit. The map is fully mapped.</p>"#
        );
    }
    w!(s, "</section>");
    s
}

// ─── Panel: Radar ──────────────────────────────────────────────────────────

fn panel_radar(save: &SaveFile) -> String {
    let mut s = String::new();
    w!(s, r#"<section class="card c5">"#);
    w!(
        s,
        r#"<h2>Chapter Radar<span class="sub">completion per arc</span></h2>"#
    );
    w!(s, "{}", radar_svg(save));
    w!(
        s,
        r#"<p class="note">Each spoke is one story arc; the hull reaches the outer ring when the arc is fully passed. A vertex pinned at the centre means the arc is untouched.</p>"#
    );
    w!(s, "</section>");
    s
}

// ─── Panel: Bosses ─────────────────────────────────────────────────────────

fn panel_bosses(save: &SaveFile) -> String {
    let mut s = String::new();
    w!(s, r#"<section class="card c7">"#);
    w!(
        s,
        r#"<h2>Boss Fights<span class="sub">HP = lessons left in the block</span></h2>"#
    );

    for boss in &BOSSES {
        let (done, total) = range_progress(save, boss.guards_first, boss.guards_last);
        let hp = total - done;
        let defeated = is_lesson_passed(save, boss.lesson);
        let hp_pct = if total == 0 { 0 } else { hp * 100 / total };
        let status = if defeated && hp == 0 {
            r#"<span class="tag t-pass">DEFEATED &middot; BLOCK CLEAR</span>"#.to_string()
        } else if defeated {
            format!(
                r#"<span class="tag t-pass">DEFEATED</span> <span class="tag t-next">{hp} STRAGGLER{}</span>"#,
                if hp == 1 { "" } else { "S" }
            )
        } else {
            r#"<span class="tag" style="color:var(--rd)">ACTIVE</span>"#.to_string()
        };

        w!(
            s,
            r#"<div class="boss{}">"#,
            if defeated { " dead" } else { "" }
        );
        w!(s, r#"<div class="boss-hd"><span class="boss-nm">{}</span>{status}</div>"#, esc(boss.name));
        w!(s, r#"<div class="tl">{}</div>"#, esc(boss.tagline));
        w!(s, r#"<div class="hp"><i style="width:{hp_pct}%"></i></div>"#);
        w!(
            s,
            r#"<div class="hp-lbl"><span>HP {hp}/{total}</span><span>capstone c{:02} &middot; guards c{:02}&ndash;c{:02} &middot; {done}/{total} cleared</span></div>"#,
            boss.lesson, boss.guards_first, boss.guards_last
        );
        w!(s, "</div>");
    }

    w!(
        s,
        r#"<p class="note">A boss takes damage every time you pass a lesson in the block it guards. Beating the capstone marks it <b>DEFEATED</b> even if a few lessons in the block are still open &mdash; those show as stragglers.</p>"#
    );
    w!(s, "</section>");
    s
}

// ─── Panel: Activity Heatmap ───────────────────────────────────────────────

fn panel_heatmap(save: &SaveFile, hist: &History, today: i64) -> String {
    let (current, longest) = streaks(hist, today);
    let pre = pre_history(save);

    // Align the grid to whole Sun..Sat weeks ending with the current week.
    let end_of_week = today + (6 - weekday(today) as i64);
    let start = end_of_week - (HEATMAP_WEEKS * 7 - 1);

    let mut s = String::new();
    w!(s, r#"<section class="card c7">"#);
    w!(
        s,
        r#"<h2>Activity<span class="sub">last {HEATMAP_WEEKS} weeks &middot; UTC days</span></h2>"#
    );

    w!(s, r#"<div class="streaks">"#);
    w!(
        s,
        r#"<div class="streak{}"><b>{current}</b><span>Current streak</span></div>"#,
        if current == 0 { " cold" } else { "" }
    );
    w!(
        s,
        r#"<div class="streak{}"><b>{longest}</b><span>Longest streak</span></div>"#,
        if longest == 0 { " cold" } else { "" }
    );
    w!(
        s,
        r#"<div class="streak{}"><b>{}</b><span>Tracked passes</span></div>"#,
        if hist.real_passes == 0 { " cold" } else { "" },
        hist.real_passes
    );
    w!(
        s,
        r#"<div class="streak{}"><b>{}</b><span>Active days</span></div>"#,
        if hist.day_counts.is_empty() { " cold" } else { "" },
        hist.day_counts.len()
    );
    w!(s, "</div>");

    // ── Month labels ───────────────────────────────────────────────────────
    w!(
        s,
        r#"<div class="scrollx"><div class="hm-wrap"><div class="hm-months" style="grid-template-columns:repeat({HEATMAP_WEEKS},15px)">"#
    );
    let mut last_month: Option<u32> = None;
    for wk in 0..HEATMAP_WEEKS {
        let (_, m, _) = civil_from_days(start + wk * 7);
        if last_month != Some(m) {
            // Clamp the span to the columns actually left, so the label can never
            // create implicit grid columns and drag the row wider than the heatmap.
            let room = HEATMAP_WEEKS - wk;
            if room >= 2 {
                w!(
                    s,
                    r#"<div style="grid-column:{}/span {}">{}</div>"#,
                    wk + 1,
                    room.min(4),
                    MONTH_NAMES[(m - 1) as usize]
                );
            }
            last_month = Some(m);
        }
    }
    w!(s, "</div>");

    // ── Grid ───────────────────────────────────────────────────────────────
    w!(s, r#"<div class="hm-row">"#);
    w!(s, r#"<div class="hm-dow">"#);
    for lbl in ["", "Mon", "", "Wed", "", "Fri", ""] {
        w!(s, "<div>{lbl}</div>");
    }
    w!(s, "</div>");
    w!(
        s,
        r#"<div class="hm-grid" style="grid-template-columns:repeat({HEATMAP_WEEKS},12px)">"#
    );
    for wk in 0..HEATMAP_WEEKS {
        for dow in 0..7 {
            let day = start + wk * 7 + dow;
            if day > today {
                w!(s, r#"<i class="fut"></i>"#);
                continue;
            }
            let count = hist.day_counts.get(&day).copied().unwrap_or(0);
            let cls = match count {
                0 => "",
                1 => " l1",
                2 => " l2",
                3..=4 => " l3",
                _ => " l4",
            };
            let label = if count == 0 {
                format!("{} — no passes", format_day(day))
            } else {
                format!(
                    "{} — {count} lesson{}",
                    format_day(day),
                    if count == 1 { "" } else { "s" }
                )
            };
            w!(s, r#"<i class="{}" title="{}"></i>"#, cls.trim(), esc(&label));
        }
    }
    w!(s, "</div></div>");

    w!(s, r#"<div class="hm-legend"><span>Less</span>"#);
    for c in ["", "l1", "l2", "l3", "l4"] {
        w!(s, r#"<i class="{c}"></i>"#);
    }
    w!(s, r#"<span>More</span></div>"#);
    w!(s, "</div></div>");

    // ── Pre-history band ───────────────────────────────────────────────────
    if !pre.lessons.is_empty() {
        w!(s, r#"<div class="prehist">"#);
        w!(s, r#"<h3>▚ Pre-history &mdash; backfilled block</h3>"#);
        w!(
            s,
            r#"<p><b>{}</b> lesson{} arrived in one bulk <code>--rescan</code>{}, so their timestamps all collapsed onto a single minute. They still count for your level, XP, stats, abilities, chapters and bosses &mdash; but plotting them would render one absurd spike and a dead grid, so the heatmap, streaks and pace math leave them out.</p>"#,
            pre.lessons.len(),
            if pre.lessons.len() == 1 { "" } else { "s" },
            pre.day
                .map(|d| format!(" on {}", esc(&format_day(d))))
                .unwrap_or_default()
        );
        w!(s, r#"<div class="blocks">"#);
        for n in &pre.lessons {
            let lesson = &LESSONS[(n - 1) as usize];
            w!(
                s,
                r#"<i title="c{n:02} {}"></i>"#,
                esc(lesson.title)
            );
        }
        w!(s, "</div></div>");
    }

    if hist.is_empty() {
        w!(
            s,
            r#"<p class="note"><b>No real history yet.</b> The grid fills in from your next <code>cargo run --bin progress</code> onward &mdash; every pass gets its own timestamped event from here on. Streaks start at 1 the day you close your next lesson.</p>"#
        );
    } else {
        w!(
            s,
            r#"<p class="note">A day counts as active when at least one lesson goes green on it. The current streak has to reach today or yesterday &mdash; otherwise it is history, not a streak.</p>"#
        );
    }

    w!(s, "</section>");
    s
}

// ─── Panel: Pace ───────────────────────────────────────────────────────────

fn panel_pace(save: &SaveFile, pace: &Pace, hist: &History) -> String {
    let level = lessons_passed(save);
    let remaining = NUM_LESSONS.saturating_sub(level);

    let mut s = String::new();
    w!(s, r#"<section class="card c12">"#);
    w!(
        s,
        r#"<h2>Pace<span class="sub">real history only &mdash; backfill excluded</span></h2>"#
    );
    w!(s, r#"<div class="tiles">"#);

    let na = r#"<b class="na">&mdash;</b>"#;

    w!(s, r#"<div class="tile"><span>Lessons / week</span>"#);
    match pace.per_week {
        Some(pw) => w!(s, "<b>{pw:.1}</b><em>over {} days tracked</em>", pace.span_days.unwrap_or(0)),
        None => w!(s, "{na}<em>not enough history yet</em>"),
    }
    w!(s, "</div>");

    w!(s, r#"<div class="tile"><span>ETA to level {NUM_LESSONS}</span>"#);
    match pace.eta_day {
        Some(d) => w!(s, "<b>{}</b><em>{remaining} lessons left</em>", esc(&format_day(d))),
        None if remaining == 0 => w!(s, r#"<b class="na">DONE</b><em>all cases closed</em>"#),
        None => w!(s, "{na}<em>{remaining} lessons left &middot; need more history</em>"),
    }
    w!(s, "</div>");

    w!(s, r#"<div class="tile"><span>Avg attempts</span>"#);
    match pace.avg_attempts {
        Some(a) => w!(s, "<b>{a:.2}</b><em>per passed lesson</em>"),
        None => w!(s, "{na}<em>no attempt data</em>"),
    }
    w!(s, "</div>");

    w!(s, r#"<div class="tile"><span>Most attempted</span>"#);
    match pace.hardest {
        Some((n, a)) => {
            let lesson = &LESSONS[(n - 1) as usize];
            w!(
                s,
                "<b>c{n:02}</b><em>{} &middot; {a} attempts</em>",
                esc(lesson.title)
            )
        }
        None => w!(s, "{na}<em>nothing has fought back yet</em>"),
    }
    w!(s, "</div>");

    w!(
        s,
        r#"<div class="tile"><span>Total scans</span><b>{}</b><em>{}</em></div>"#,
        pace.scans,
        if pace.scans == 0 {
            "logged from the first v2 scan"
        } else {
            "recorded in event history"
        }
    );

    w!(
        s,
        r#"<div class="tile"><span>Tracked passes</span><b>{}</b><em>{} events in log</em></div>"#,
        hist.real_passes,
        save.history.len()
    );

    w!(s, "</div>");

    if pace.per_week.is_none() {
        w!(
            s,
            r#"<p class="note"><b>Not enough history yet.</b> Velocity needs at least two passes spread over two different days. Everything currently on the sheet came from one bulk rescan, and extrapolating from a single minute would invent a number rather than measure one.</p>"#
        );
    }
    w!(s, "</section>");
    s
}

// ─── Panel: Stat Grid ──────────────────────────────────────────────────────

fn panel_stats(save: &SaveFile) -> String {
    let maxed = STAT_GROUPS
        .iter()
        .filter(|g| stat_max(g) > 0 && stat_score(save, g) == stat_max(g))
        .count();

    let mut s = String::new();
    w!(s, r#"<section class="card c12">"#);
    w!(
        s,
        r#"<h2>Subsystems<span class="sub">{maxed}/{} maxed</span></h2>"#,
        STAT_GROUPS.len()
    );
    w!(s, r#"<div class="stats">"#);

    for group in &STAT_GROUPS {
        let score = stat_score(save, group);
        let max = stat_max(group);
        let cls = if max > 0 && score == max {
            "s-full"
        } else if score == 0 {
            "s-zero"
        } else {
            "s-part"
        };
        w!(s, r#"<div class="stat {cls}">"#);
        w!(
            s,
            r#"<div class="lbl"><span>{}</span><span class="num">{score}/{max}</span></div>"#,
            esc(group)
        );
        w!(s, r#"<div class="dots">"#);
        for _ in 0..score {
            w!(s, "●");
        }
        if score < max {
            w!(s, r#"<span class="off">"#);
            for _ in score..max {
                w!(s, "○");
            }
            w!(s, "</span>");
        }
        w!(s, "</div></div>");
    }

    w!(s, "</div>");
    w!(
        s,
        r#"<p class="note">Same semantics as the terminal sheet: <b style="color:var(--gr)">●</b> filled and green when a subsystem is maxed, <b style="color:var(--ye)">●</b> yellow while partial, hollow when the lesson is still locked.</p>"#
    );
    w!(s, "</section>");
    s
}

// ─── Panel: Trophies ───────────────────────────────────────────────────────

fn panel_trophies(trophies: &[Trophy]) -> String {
    let earned = trophies.iter().filter(|t| t.earned).count();

    let mut s = String::new();
    w!(s, r#"<section class="card c12">"#);
    w!(
        s,
        r#"<h2>Trophy Case<span class="sub">{earned}/{} earned</span></h2>"#,
        trophies.len()
    );
    w!(s, r#"<div class="trophies">"#);

    for t in trophies {
        w!(
            s,
            r#"<div class="tr{}">"#,
            if t.earned { " got" } else { "" }
        );
        w!(s, r#"<div class="ic">{}</div><div style="min-width:0">"#, t.icon);
        w!(s, r#"<div class="nm">{}</div>"#, esc(&t.name));
        w!(s, r#"<div class="cr">{}</div>"#, esc(&t.criterion));
        if let Some((done, total)) = t.progress {
            if !t.earned && total > 0 {
                w!(
                    s,
                    r#"<div class="prog"><i style="width:{}%"></i></div>"#,
                    done * 100 / total
                );
                w!(s, r#"<div class="meta">{done}/{total}</div>"#);
            }
        }
        if t.earned {
            match t.when {
                Some(ts) => w!(s, r#"<div class="meta">UNLOCKED &middot; {}</div>"#, esc(&format_day(epoch_day(ts)))),
                None => w!(s, r#"<div class="meta">UNLOCKED</div>"#),
            }
        } else if t.needs_history {
            w!(
                s,
                r#"<div class="meta v2">NEEDS EVENT HISTORY &middot; earnable from your next scan</div>"#
            );
        }
        w!(s, "</div></div>");
    }

    w!(s, "</div>");
    w!(
        s,
        r#"<p class="note">Locked cards state exactly what earns them &mdash; that is the point. The ones marked <b class="v2" style="color:var(--mg)">NEEDS EVENT HISTORY</b> depend on timestamped events that only a v2 save records; they start counting from your next <code>progress</code> run.</p>"#
    );
    w!(s, "</section>");
    s
}

// ─── Panel: Quest Log ──────────────────────────────────────────────────────

fn panel_quests(save: &SaveFile) -> String {
    let next = frontier(save).map(|l| l.number);

    let mut s = String::new();
    w!(s, r#"<section class="card c12">"#);
    w!(
        s,
        r#"<h2>Case Log<span class="sub">all {NUM_LESSONS} lessons</span></h2>"#
    );

    for chap in &CHAPTERS {
        let (done, total) = range_progress(save, chap.first, chap.last);
        w!(s, r#"<div class="chap">"#);
        w!(s, r#"<div class="chap-hd"><span class="n">{}</span><span class="s">{}</span><span class="p">{done}/{total}</span></div>"#, esc(chap.name), esc(chap.subtitle));
        w!(s, r#"<div class="qgrid">"#);

        for n in chap.first..=chap.last {
            let lesson = &LESSONS[(n - 1) as usize];
            let status = lesson_status(save, n);
            let passed = status.is_some_and(|s| s.passed);
            let attempts = status.map(|s| s.attempts).unwrap_or(0);

            let (cls, tag) = if passed {
                ("pass", r#"<span class="tag t-pass">PASS</span>"#)
            } else if next == Some(n) {
                ("next", r#"<span class="tag t-next">NEXT</span>"#)
            } else if HOMEWORK_GAPS.contains(&n) {
                // Behind the frontier and skipped on purpose — not locked, just owed.
                ("open", r#"<span class="tag t-open">OPEN</span>"#)
            } else {
                ("lock", r#"<span class="tag t-lock">LOCK</span>"#)
            };

            let ability = ABILITIES
                .iter()
                .find(|a| a.lesson == n)
                .map(|a| format!("{} — {}", a.name, a.description))
                .unwrap_or_default();
            let title = format!("c{n:02} {} · {}\n{}", lesson.title, lesson.stat_group, ability);

            w!(s, r#"<div class="q {cls}" title="{}">"#, esc(&title));
            w!(s, "{tag}");
            w!(s, r#"<span class="id">c{n:02}</span>"#);
            w!(s, r#"<span class="ti">{}</span>"#, esc(lesson.title));
            if BUG_LESSONS.contains(&n) {
                w!(s, r#"<span class="bug">★</span>"#);
            }
            if attempts > 1 {
                w!(s, r#"<span class="at">&times;{attempts}</span>"#);
            }
            w!(s, "</div>");
        }

        w!(s, "</div></div>");
    }

    w!(
        s,
        r#"<p class="note"><span class="tag t-open">OPEN</span> is a lesson you passed over on purpose and still owe &mdash; unlike <span class="tag t-lock">LOCK</span>, nothing ahead is waiting on it. <span class="t-mag">★</span> marks a BUG HUNT lesson (ships compiling-but-wrong code). <span class="t-mag">&times;n</span> is the attempt count where a lesson took more than one run. Hover any row for the ability it unlocks.</p>"#
    );
    w!(s, "</section>");
    s
}

// ─── Panel: Knowledge Tree ─────────────────────────────────────────────────
//
// The quest log answers "what have I passed?". This answers the bigger question the
// quest log cannot: "how much of Rust is that?" KNOWLEDGE_TREE is deliberately wider
// than the curriculum, so most nodes are UNCHARTED — real Rust the 80 lessons never
// touch. That contrast is the panel's whole point.
//
// Rendered with native <details>/<summary>. No JavaScript, by hard constraint — the
// browser owns the open/closed state, which also means it survives Ctrl+F and print.

#[derive(Clone, Copy, PartialEq, Eq)]
enum KtState {
    /// Taught here, and at least one of its lessons is green.
    Learned,
    /// Taught here, not passed yet.
    Queued,
    /// Outside the 80-lesson track entirely.
    Uncharted,
}

impl KtState {
    fn class(self) -> &'static str {
        match self {
            KtState::Learned => "kt-learned",
            KtState::Queued => "kt-queued",
            KtState::Uncharted => "kt-open",
        }
    }
    fn dot(self) -> &'static str {
        match self {
            KtState::Learned => "●",
            KtState::Queued => "◐",
            KtState::Uncharted => "○",
        }
    }
}

fn kt_state(save: &SaveFile, node: &KTNode) -> KtState {
    if node.lessons.is_empty() {
        KtState::Uncharted
    } else if node.lessons.iter().any(|n| is_lesson_passed(save, *n)) {
        KtState::Learned
    } else {
        KtState::Queued
    }
}

fn kt_children(id: &str) -> Vec<&'static KTNode> {
    KNOWLEDGE_TREE
        .iter()
        .filter(|n| n.parent == Some(id))
        .collect()
}

/// (learned, queued, uncharted) for a node *and its whole subtree*.
///
/// Naive re-scan of the array per level — O(n²) over ~220 nodes, which is nothing at
/// generation time and keeps the data file a plain flat `const` with no index to build.
fn kt_rollup(save: &SaveFile, node: &KTNode) -> (u32, u32, u32) {
    let mut acc = match kt_state(save, node) {
        KtState::Learned => (1, 0, 0),
        KtState::Queued => (0, 1, 0),
        KtState::Uncharted => (0, 0, 1),
    };
    for kid in kt_children(node.id) {
        let (l, q, u) = kt_rollup(save, kid);
        acc.0 += l;
        acc.1 += q;
        acc.2 += u;
    }
    acc
}

/// The shared row body. EVERY dynamic field goes through `esc()` — this data set is
/// wall-to-wall `Option<T>`, `Box<dyn Trait>` and `extern "C"`, and one missed escape
/// turns the rest of the panel into an unclosed unknown tag.
fn kt_row(save: &SaveFile, node: &KTNode, has_kids: bool) -> String {
    let st = kt_state(save, node);
    let mut s = String::new();
    w!(s, r#"<span class="kt-dot">{}</span>"#, st.dot());
    w!(s, r#"<span class="kt-nm">{}</span>"#, esc(node.name));

    // Traceable back to the quest log: which lessons actually taught this.
    let passed: Vec<String> = node
        .lessons
        .iter()
        .filter(|n| is_lesson_passed(save, **n))
        .map(|n| format!("c{n:02}"))
        .collect();
    if !passed.is_empty() {
        w!(
            s,
            r#"<span class="kt-cells">{}</span>"#,
            esc(&passed.join(" "))
        );
    }

    w!(s, r#"<span class="kt-blurb">{}</span>"#, esc(node.blurb));

    if has_kids {
        let (l, q, u) = kt_rollup(save, node);
        w!(
            s,
            r#"<span class="kt-count"><b class="kl">{l} learned</b> &middot; <b class="kq">{q} queued</b> &middot; <b class="ku">{u} uncharted</b></span>"#
        );
    }
    s
}

fn kt_render(save: &SaveFile, node: &'static KTNode, depth: usize, out: &mut String) {
    let cls = kt_state(save, node).class();
    let kids = kt_children(node.id);

    // A leaf is a plain row: an empty <details> would be a disclosure triangle that
    // opens onto nothing.
    if kids.is_empty() {
        w!(out, r#"<div class="kt-leaf {cls}">{}</div>"#, kt_row(save, node, false));
        return;
    }

    // Roots open by default so the page lands on a full table of contents; every
    // nested level starts closed so ~220 nodes don't bury the rest of the dashboard.
    let (extra, open) = if depth == 0 {
        (" kt-root", " open")
    } else {
        ("", "")
    };
    w!(out, r#"<details class="kt-branch {cls}{extra}"{open}>"#);
    w!(
        out,
        r#"<summary class="kt-sum">{}</summary>"#,
        kt_row(save, node, true)
    );
    w!(out, r#"<div class="kt-kids">"#);
    for kid in kids {
        kt_render(save, kid, depth + 1, out);
    }
    w!(out, "</div></details>");
}

fn panel_knowledge(save: &SaveFile) -> String {
    let roots: Vec<&'static KTNode> = KNOWLEDGE_TREE.iter().filter(|n| n.parent.is_none()).collect();
    let (mut learned, mut queued, mut uncharted) = (0u32, 0u32, 0u32);
    for r in &roots {
        let (l, q, u) = kt_rollup(save, r);
        learned += l;
        queued += q;
        uncharted += u;
    }

    let mut s = String::new();
    w!(s, r#"<section class="card c12">"#);
    w!(
        s,
        r#"<h2>Knowledge Tree<span class="sub">{} nodes across the Rust landscape</span></h2>"#,
        KNOWLEDGE_TREE.len()
    );

    w!(s, r#"<div class="kt-key">"#);
    w!(
        s,
        r#"<span class="kt-learned kt-leaf" style="padding:0"><span class="kt-dot">●</span> <b style="color:var(--gr)">{learned} learned</b></span>"#
    );
    w!(
        s,
        r#"<span class="kt-queued kt-leaf" style="padding:0"><span class="kt-dot">◐</span> <b style="color:var(--ye)">{queued} queued</b></span>"#
    );
    w!(
        s,
        r#"<span class="kt-open kt-leaf" style="padding:0"><span class="kt-dot">○</span> <b style="color:var(--dim)">{uncharted} uncharted</b></span>"#
    );
    w!(s, "</div>");

    for root in &roots {
        kt_render(save, root, 0, &mut s);
    }

    w!(
        s,
        r#"<p class="note"><b style="color:var(--gr)">● learned</b> means the curriculum teaches this node and at least one of its lessons is green; <b style="color:var(--ye)">◐ queued</b> means it is taught here but you have not passed it yet; <b>○ uncharted</b> is real Rust that lies outside the {NUM_LESSONS}-lesson track entirely &mdash; no lesson claims it. Each branch header rolls up its whole subtree, and the small <span class="kt-cells">c07</span>-style tags name the exact lessons behind a learned node.</p>"#
    );
    w!(s, "</section>");
    s
}

// ─── Page ──────────────────────────────────────────────────────────────────

fn render_page(save: &SaveFile, migrated: bool) -> String {
    let today = epoch_day(now_epoch());
    let hist = build_history(save);
    let (current, longest) = streaks(&hist, today);
    let trophies = build_trophies(save, &hist, current, longest);
    let pace = build_pace(save, &hist, today);
    let level = lessons_passed(save);

    let mut s = String::with_capacity(96 * 1024);
    w!(s, r#"<meta charset="utf-8">"#);
    w!(
        s,
        r#"<meta name="viewport" content="width=device-width,initial-scale=1">"#
    );
    w!(
        s,
        "<title>{} — Rust General Hospital Dashboard</title>",
        esc(&save.character.name)
    );
    w!(s, "<style>{CSS}</style>");

    w!(s, r#"<main class="wrap">"#);
    w!(s, "{}", panel_hero(save));

    if migrated {
        w!(
            s,
            r#"<div class="card c12" style="margin-bottom:16px;border-color:rgba(255,212,71,.4)"><p style="margin:0;font-size:12px;color:var(--ye)">▲ This save was written by an older version of the tracker. The dashboard upgraded it in memory to render correctly, but it never writes the file &mdash; run <code style="color:var(--cy)">cargo run --bin progress</code> to persist the upgrade.</p></div>"#
        );
    }

    w!(s, r#"<div class="grid">"#);
    // The map is a horizontal strip — full width or it leaves a void beside it.
    w!(s, "{}", panel_map(save));
    w!(s, "{}", panel_next(save));
    w!(s, "{}", panel_bosses(save));
    w!(s, "{}", panel_radar(save));
    w!(s, "{}", panel_heatmap(save, &hist, today));
    w!(s, "{}", panel_pace(save, &pace, &hist));
    w!(s, "{}", panel_stats(save));
    w!(s, "{}", panel_trophies(&trophies));
    w!(s, "{}", panel_quests(save));
    w!(s, "{}", panel_knowledge(save));
    w!(s, "</div>");

    w!(s, "<footer>");
    w!(
        s,
        "<span>Rust General Hospital &middot; {} / {NUM_LESSONS} cases closed</span>",
        level
    );
    w!(
        s,
        "<span>Generated {} &middot; cargo run --bin dashboard</span>",
        esc(&format_stamp(now_epoch()))
    );
    w!(s, "</footer>");
    w!(s, "</main>");
    s
}

// ─── Browser ───────────────────────────────────────────────────────────────

/// Best effort. A failed opener is a shrug, never a panic — the file is already written.
fn open_in_browser(path: &std::path::Path) -> std::io::Result<()> {
    use std::process::Command;
    let p = path.to_string_lossy().to_string();

    #[cfg(target_os = "windows")]
    let mut cmd = {
        let mut c = Command::new("cmd");
        c.args(["/C", "start", "", &p]);
        c
    };
    #[cfg(target_os = "macos")]
    let mut cmd = {
        let mut c = Command::new("open");
        c.arg(&p);
        c
    };
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut cmd = {
        let mut c = Command::new("xdg-open");
        c.arg(&p);
        c
    };

    cmd.spawn().map(|_| ())
}

// ─── Help ──────────────────────────────────────────────────────────────────

fn print_help() {
    println!();
    println!("  {}", color::bold("Rust General Hospital — HTML Dashboard"));
    println!();
    println!("  {}", color::bold("USAGE:"));
    println!("    cargo run --bin dashboard                Write {OUT_FILE} and open it");
    println!("    cargo run --bin dashboard -- --no-open   Write the file only");
    println!("    cargo run --bin dashboard -- --help      Show this help message");
    println!();
    println!("  {}", color::bold("WHAT IT DOES:"));
    println!("    1. Reads .rustacean_save.json (read-only — never writes it)");
    println!("    2. Renders a single self-contained HTML page: rank, radar, heatmap,");
    println!("       bosses, trophies, pace and all {NUM_LESSONS} quests");
    println!("    3. Opens it in your default browser");
    println!();
    println!("  {}", color::bold("NOTES:"));
    println!("    The page is fully offline — no CDN, no fonts, no scripts.");
    println!("    Only `cargo run --bin progress` scans tests and writes your save.");
    println!();
    println!("  {}", color::bold("FILES:"));
    println!("    .rustacean_save.json  Your character save file (read-only here)");
    println!("    {OUT_FILE}        Generated dashboard (gitignored)");
    println!();
}

// ─── Main ──────────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }
    let no_open = args.iter().any(|a| a == "--no-open");
    if let Some(bad) = args
        .iter()
        .find(|a| !matches!(a.as_str(), "--no-open" | "--help" | "-h"))
    {
        println!();
        println!("  {} unknown argument: {bad}", color::yellow("WARN:"));
        println!("  {}", color::dim("Try: cargo run --bin dashboard -- --help"));
    }

    // ── Load (read-only) ───────────────────────────────────────────────────
    let mut save = match load_save() {
        Some(s) => s,
        None => {
            println!();
            if save_path().exists() {
                println!(
                    "  {} {} exists but could not be parsed.",
                    color::yellow("SAVE UNREADABLE:"),
                    save_path().display()
                );
                println!(
                    "  {}",
                    color::dim("It may be from an incompatible or truncated write.")
                );
                println!(
                    "  Run {} to repair it, or {} to start fresh.",
                    color::cyan("cargo run --bin progress"),
                    color::cyan("cargo run --bin progress -- --reset")
                );
            } else {
                println!(
                    "  {} no save file at {}.",
                    color::yellow("NO SAVE:"),
                    save_path().display()
                );
                println!(
                    "  Run {} first — it creates your character and scans your tests.",
                    color::cyan("cargo run --bin progress")
                );
            }
            println!(
                "  {}",
                color::dim("The dashboard is read-only and will not create a save for you.")
            );
            println!();
            std::process::exit(1);
        }
    };

    // In-memory only. `progress` owns every write to disk.
    let migrated = migrate(&mut save);

    let html = render_page(&save, migrated);
    let out = std::path::PathBuf::from(OUT_FILE);

    if let Err(e) = std::fs::write(&out, html.as_bytes()) {
        println!();
        println!("  {} could not write {OUT_FILE}: {e}", color::yellow("WRITE FAILED:"));
        println!();
        std::process::exit(1);
    }

    let level = lessons_passed(&save);
    let rank = get_rank(level);

    println!();
    println!("  {}", color::bold_cyan("╔══════════════════════════════════════════════════════════╗"));
    println!("  {}  {}", color::bold_cyan("║"), color::bold("DASHBOARD RENDERED"));
    println!("  {}", color::bold_cyan("╚══════════════════════════════════════════════════════════╝"));
    println!();
    println!(
        "    {} {} the {}",
        color::dim("Staff:   "),
        color::bold(&save.character.name),
        save.character.class
    );
    println!(
        "    {} {}  {} {}/{}",
        color::dim("Rank:    "),
        color::yellow(rank.name),
        color::dim("Level:"),
        color::green(&level.to_string()),
        NUM_LESSONS
    );
    println!(
        "    {} {} {}",
        color::dim("File:    "),
        color::cyan(&out.display().to_string()),
        color::dim(&format!("({} bytes)", html.len()))
    );

    if migrated {
        println!();
        println!(
            "    {} save file is from an older tracker version. Rendered from an",
            color::yellow("NOTE:")
        );
        println!(
            "          in-memory upgrade; run {} to persist it.",
            color::cyan("cargo run --bin progress")
        );
    }

    if no_open {
        println!();
        println!("    {}", color::dim("--no-open: skipping browser launch."));
        println!();
        return;
    }

    println!();
    match open_in_browser(&out) {
        Ok(()) => println!("    {}", color::dim("Opening in your default browser…")),
        Err(e) => {
            println!(
                "    {} could not launch a browser ({e}).",
                color::yellow("HEADS UP:")
            );
            println!(
                "    Open it yourself: {}",
                color::cyan(&out.display().to_string())
            );
        }
    }
    println!();
}
