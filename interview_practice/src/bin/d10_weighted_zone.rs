//! Picks a patrol zone at random, with each zone's chance proportional to its area.
//!
//! APIs:
//! - `ZonePicker`: the zones and their running area totals
//! - `ZonePicker::new`: build a picker from zone names and areas
//! - `ZonePicker::total_area`: the sum of all areas
//! - `ZonePicker::pick`: map a random roll to a zone
//! - `XorShift`: a small reproducible random number generator
//! - `XorShift::new`: a generator from a seed
//! - `XorShift::next_u64`: the next random number

use std::cmp::max;

/// Description: A weighted picker over patrol zones. Zone i owns the rolls from the running total
/// before it up to, but not including, the running total after it.
/// Fields:
/// - `names`: the zone names, in the order given.
/// - `cumulative`: the running totals of area, so `cumulative[i]` is the area of zones 0 to i.
pub struct ZonePicker {
    names:      Vec<String>,
    cumulative: Vec<u64>,
}

/// Description: A xorshift random number generator. It is fast and reproducible from its seed,
/// but it is not safe for anything secret.
/// Fields:
/// - `0`: the generator state, which is never zero.
pub struct XorShift(u64);

/// Description: Picks eight patrol zones from three zones of different areas and prints them.
/// Inputs: none
/// Outputs: none
fn main() {
    // Compressed:
    // let picker = ZonePicker::new(&[("dock", 3), ("orchard", 1), ("field", 6)]).unwrap();
    let zones   = [("dock", 3), ("orchard", 1), ("field", 6)];
    let picker  = ZonePicker::new(&zones).unwrap();
    let mut rng = XorShift::new(42);

    // Compressed:
    // let picks: Vec<&str> = (0..8).map(|_| picker.pick(rng.next_u64())).collect();
    let mut picks: Vec<&str> = Vec::new();
    for _ in 0..8 {
        let roll = rng.next_u64();
        let zone = picker.pick(roll);
        picks.push(zone);
    }
    println!("{picks:?}");
}

impl ZonePicker {
    /// Description: Builds a picker from zone names and areas.
    /// Inputs:
    /// - `zones`: each zone's name and its area in square meters.
    /// Outputs: the picker, or `None` when no zone has a positive area.
    pub fn new(zones: &[(&str, u64)]) -> Option<Self> {
        let mut names      = Vec::with_capacity(zones.len());
        let mut cumulative = Vec::with_capacity(zones.len());
        let mut total      = 0;
        for &(name, area) in zones {
            // Compressed:
            // total += area;
            // names.push(name.to_string());
            total         += area;
            let owned_name = name.to_string();
            names.push(owned_name);
            cumulative.push(total);
        }

        if total == 0 {
            return None;
        }

        // Compressed:
        // Some(ZonePicker { names, cumulative })
        let picker = ZonePicker { names, cumulative };
        Some(picker)
    }

    /// Description: Maps a random roll to a zone with a binary search over the running totals.
    /// Inputs:
    /// - `roll`: a random number, such as one from `XorShift::next_u64`.
    /// Outputs: the name of the chosen zone.
    /// Constraints: O(log n) time. A zone with zero area is never chosen. Taking the roll modulo
    /// the total area adds a bias too small to matter while the total is far below 2^64.
    pub fn pick(&self, roll: u64) -> &str {
        // Compressed:
        // let target = roll % self.total_area();
        // let index = self.cumulative.partition_point(|&end| end <= target);
        let total_area = self.total_area();
        let target     = roll % total_area;
        let index      = self.cumulative.partition_point(|&end| end <= target);
        &self.names[index]
    }

    /// Description: Returns the area of all zones together.
    /// Inputs: none
    /// Outputs: the total area in square meters.
    pub fn total_area(&self) -> u64 {
        // Compressed:
        // *self.cumulative.last().expect("a picker always holds a zone")
        match self.cumulative.last() {
            Some(total) => *total,
            None => panic!("a picker always holds a zone"),
        }
    }
}

impl XorShift {
    /// Description: Builds a generator from a seed.
    /// Inputs:
    /// - `seed`: any number. Zero becomes one, because a zero state would stay zero forever.
    /// Outputs: the generator.
    pub fn new(seed: u64) -> Self {
        // Compressed:
        // XorShift(seed.max(1))
        let nonzero_seed = max(seed, 1);
        XorShift(nonzero_seed)
    }

    /// Description: Advances the generator by one step.
    /// Inputs: none
    /// Outputs: the next pseudo-random number.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;

        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;

        self.0 = x;
        x
    }
}
