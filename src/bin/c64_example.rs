// RefCell also gives interior mutability, and it works for data that is not Copy. It does
// this by moving the borrow check from compile time to run time. borrow() and borrow_mut()
// keep track of the active borrows, and they panic if a new borrow would break the rules.
// try_borrow_mut() returns an Err instead of panicking.
// Coming from C: this is like a runtime assert that fires when a mutable object is aliased.
// The compiler usually proves this safety before the program runs. RefCell checks it while
// the program runs instead.
//
// RUST GENERAL HOSPITAL: a patient's chart is shared by the whole care team. While a doctor
// holds it open for review, a request to add a note should get an error back instead of
// crashing the charting system. The charting system on the ward must never crash.
use std::cell::RefCell;

struct Chart {
    notes: RefCell<Vec<String>>,
}

impl Chart {
    fn new() -> Self {
        Self { notes: RefCell::new(Vec::new()) }
    }
    fn add_note(&self, note: &str) {
        self.notes.borrow_mut().push(note.to_string());
    }
    fn note_count(&self) -> usize {
        self.notes.borrow().len()
    }
}

fn main() {
    let chart = Chart::new();
    chart.add_note("BP 120/80 at 08:00");

    {
        let review = chart.notes.borrow(); // a doctor holds the chart open
        // Calling borrow_mut() while `review` is still alive would panic at run time:
        //   chart.notes.borrow_mut(); // thread panics: already borrowed
        // try_borrow_mut() returns Err instead of panicking:
        println!("[chart] note during review ok? {}", chart.notes.try_borrow_mut().is_ok()); // false
        println!("[chart] review sees {} notes", review.len());
    } // the borrow held by `review` is released here

    println!("[chart] note after review ok? {}", chart.notes.try_borrow_mut().is_ok()); // true
    chart.add_note("Paracetamol 500 mg given at 09:00");
    println!("[chart] {} notes total", chart.note_count());
}
