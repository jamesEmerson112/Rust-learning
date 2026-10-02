// RefCell also gives interior mutability, but for non-Copy data, by moving the borrow check from
// compile time to RUNTIME: borrow()/borrow_mut() track active borrows and PANIC on a violation.
// Coming from C: like a runtime assert that catches aliasing a mutable thing — the safety the
// compiler usually proves statically, enforced dynamically instead. try_borrow_mut avoids the panic.
//
// RUST GENERAL HOSPITAL: a patient's chart is shared by the whole care team. While a doctor
// holds it open for review, a new note must be REFUSED gracefully — the charting system must
// never crash on the ward.
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
        // A borrow_mut() *while `review` is alive* would PANIC at runtime:
        //   chart.notes.borrow_mut(); // thread panics: already borrowed
        // try_borrow_mut() returns Err instead of panicking:
        println!("[chart] note during review ok? {}", chart.notes.try_borrow_mut().is_ok()); // false
        println!("[chart] review sees {} notes", review.len());
    } // `review` released here

    println!("[chart] note after review ok? {}", chart.notes.try_borrow_mut().is_ok()); // true
    chart.add_note("Paracetamol 500 mg given at 09:00");
    println!("[chart] {} notes total", chart.note_count());
}
