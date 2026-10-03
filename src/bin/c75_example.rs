// Bug Hunt drill: counting with a HashMap. To tally repeats, you have to read the running
// count, add to it, and write it back. Overwriting the slot loses the old count.
// entry(k).or_insert(0) gives you a &mut to the existing count, and it inserts 0 only the
// first time it sees a key. Dereferencing that reference with `*` and adding 1 increments
// the count in place.
// Coming from C: insert() is `map[k] = 1;`, which overwrites whatever was there.
// entry().or_insert() is `count = map_get_or_zero(k); map[k] = count + 1;` done in a single
// lookup.
use std::collections::HashMap;

pub fn visit_counts(entries: &[(&str, &str)]) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for &(nurse, _patient) in entries {
        *counts.entry(nurse.to_string()).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let shift = [
        ("Mai", "Mr. Hung"),
        ("Mai", "Mrs. Lan"),
        ("Mai", "Mr. Bao"),
        ("Mai", "Mr. Hung"),
        ("Mai", "Mrs. Lan"),
        ("Mai", "Mr. Hung"),
        ("Linh", "Mrs. Lan"),
        ("Linh", "Mr. Hung"),
        ("Trang", "Mr. Bao"),
    ];
    let counts = visit_counts(&shift);
    println!("Mai:   {:?}", counts.get("Mai"));
    println!("Linh:  {:?}", counts.get("Linh"));
    println!("Trang: {:?}", counts.get("Trang"));
}
