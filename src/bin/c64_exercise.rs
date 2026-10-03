// RUST GENERAL HOSPITAL — Shared Care — ★ BUG HUNT ★
//
// BUG: The charting system crashes on the ward with an "already borrowed" panic, which is a
// BorrowMutError. It happens when a nurse adds a note while a doctor still holds the chart
// open for review. The compiler does not catch this, because RefCell checks borrows at run
// time. A crashed chart in the middle of a shift is worse than a delayed note.
//
// Find the bug and fix it, then run: cargo test --test c64_tests
use std::cell::RefCell;

pub struct Chart {
    pub notes: RefCell<Vec<String>>,
}

impl Chart {
    pub fn new() -> Self {
        Self { notes: RefCell::new(Vec::new()) }
    }

    pub fn add_note(&self, note: &str) {
        self.notes.borrow_mut().push(note.to_string());
    }

    pub fn note_count(&self) -> usize {
        self.notes.borrow().len()
    }

    // A note that arrives during a review must come back as an Err, not a panic.
    pub fn note_during_review(&self) -> Result<usize, String> {
        let review = self.notes.borrow(); // the doctor holds the chart open
        self.notes.borrow_mut().push("pulse 72 at 08:15".to_string());
        Ok(review.len())
    }

    // After the review releases the chart, notes can be added again.
    pub fn note_after_review(&self) -> usize {
        {
            let _review = self.notes.borrow();
        } // the review is released here
        self.add_note("pulse 74 at 08:30");
        self.note_count()
    }
}

fn main() {
    let chart = Chart::new();
    chart.add_note("BP 120/80 at 08:00");
    println!("[chart] note during review: {:?} (want Err(..), not a panic)", chart.note_during_review());
    println!("[chart] note after review: {} notes (want 2)", chart.note_after_review());
    println!("══ when the chart survives the review, Shared Care is complete ══");
}
