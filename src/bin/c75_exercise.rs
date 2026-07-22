// BUG: Mai swears she did 6 clients today, but the ledger insists she did exactly 1.
// So does Linh. So does Trang. Every technician's tally is stuck at 1 no matter how packed
// the day was. The code compiles and runs fine — it just can't count. Find and fix it.
// (This drills c13/c14: HashMap counting. The tests in tests/c75_tests.rs must go green.)
use std::collections::HashMap;

pub fn service_counts(entries: &[(&str, &str)]) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for &(tech, _service) in entries {
        counts.insert(tech.to_string(), 1);
    }
    counts
}

fn main() {
    let day = [
        ("Mai", "Gel Manicure"),
        ("Mai", "Pedicure"),
        ("Mai", "Acrylic Full Set"),
        ("Mai", "Gel Manicure"),
        ("Mai", "Pedicure"),
        ("Mai", "Gel Manicure"),
        ("Linh", "Pedicure"),
        ("Linh", "Gel Manicure"),
        ("Trang", "Acrylic Full Set"),
    ];
    let counts = service_counts(&day);
    println!("Mai served {:?} clients today", counts.get("Mai"));
    println!("Linh served {:?} clients today", counts.get("Linh"));
    println!("Trang served {:?} clients today", counts.get("Trang"));
}
