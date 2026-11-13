//! Tests for the weighted zone picker in `src/bin/d10_weighted_zone.rs`.
//!
//! Contents:
//! - `three_zones`: a helper with zones of area 3, 1 and 6
//! - mapping tests: which rolls land in which zone, and zero-area zones
//! - construction test: no positive area means no picker
//! - distribution test: many random rolls follow the areas

#[path = "../src/bin/d10_weighted_zone.rs"]
#[allow(dead_code)]
mod d10_weighted_zone;

use d10_weighted_zone::{XorShift, ZonePicker};

/// Description: Builds a picker over a dock, an orchard and a field.
/// Inputs: none
/// Outputs: a picker where the zones have areas 3, 1 and 6.
fn three_zones() -> ZonePicker {
    ZonePicker::new(&[("dock", 3), ("orchard", 1), ("field", 6)]).unwrap()
}

/// Description: Rolls 0 to 2 land on the dock, 3 on the orchard and 4 to 9 on the field, and
/// roll 10 wraps back to the dock.
/// Inputs: none
/// Outputs: none
#[test]
fn rolls_map_onto_area_ranges() {
    let picker = three_zones();
    assert_eq!(picker.total_area(), 10);
    assert_eq!(picker.pick(0), "dock");
    assert_eq!(picker.pick(2), "dock");
    assert_eq!(picker.pick(3), "orchard");
    assert_eq!(picker.pick(4), "field");
    assert_eq!(picker.pick(9), "field");
    assert_eq!(picker.pick(10), "dock");
}

/// Description: A zone with zero area owns no rolls.
/// Inputs: none
/// Outputs: none
#[test]
fn a_zero_area_zone_is_never_picked() {
    let picker = ZonePicker::new(&[("a", 2), ("b", 0), ("c", 2)]).unwrap();
    for roll in 0..4 {
        assert_ne!(picker.pick(roll), "b");
    }
    assert_eq!(picker.pick(1), "a");
    assert_eq!(picker.pick(2), "c");
}

/// Description: With no zones, or only zero-area zones, there is nothing to pick from.
/// Inputs: none
/// Outputs: none
#[test]
fn no_positive_area_means_no_picker() {
    assert!(ZonePicker::new(&[]).is_none());
    assert!(ZonePicker::new(&[("a", 0)]).is_none());
}

/// Description: Over 100,000 random rolls each zone's share lands within 2 percent of its share
/// of the area.
/// Inputs: none
/// Outputs: none
#[test]
fn picks_follow_the_areas_over_many_rolls() {
    let picker = three_zones();
    let mut rng = XorShift::new(2026);
    let mut counts = [0u32; 3];
    for _ in 0..100_000 {
        match picker.pick(rng.next_u64()) {
            "dock" => counts[0] += 1,
            "orchard" => counts[1] += 1,
            _ => counts[2] += 1,
        }
    }
    for (count, expected) in counts.into_iter().zip([30_000, 10_000, 60_000]) {
        assert!(count.abs_diff(expected) < 2_000, "got {count}, expected about {expected}");
    }
}
