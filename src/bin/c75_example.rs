// Bug Hunt drill: counting with HashMap. To tally repeats you must READ-MODIFY-WRITE the
// running count, not overwrite the slot. entry(k).or_insert(0) hands you a &mut to the
// existing count (inserting 0 only the first time); *= += 1 bumps it.
// Coming from C: insert() is `map[k] = 1;` — it stomps whatever was there. entry().or_insert()
// is `count = map_get_or_zero(k); map[k] = count + 1;` done atomically in one lookup.
use std::collections::HashMap;

pub fn service_counts(entries: &[(&str, &str)]) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for &(tech, _service) in entries {
        *counts.entry(tech.to_string()).or_insert(0) += 1;
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
    println!("Mai:   {:?}", counts.get("Mai"));
    println!("Linh:  {:?}", counts.get("Linh"));
    println!("Trang: {:?}", counts.get("Trang"));
}
