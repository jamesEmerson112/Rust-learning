//! Compacts a lidar scan in place, moving valid ranges to the front and dropouts to the end.
//!
//! APIs:
//! - `compact_scan`: move the zero readings to the end and keep the other readings in order

/// Description: Moves every zero reading to the end of the scan in place, and keeps the valid
/// readings in their original order. This is the Move Zeroes problem.
/// Inputs:
/// - `scan`: range readings in millimeters, where 0 means the beam got no return.
/// Outputs: the number of valid readings, which now fill `scan[..count]`.
/// Constraints: O(n) time and O(1) extra space. `write` never passes `read`, so each swap moves a
/// valid reading back over a zero, or swaps a reading with itself.
pub fn compact_scan(scan: &mut [u16]) -> usize {
    let mut write = 0;
    for read in 0..scan.len() {
        if scan[read] != 0 {
            scan.swap(write, read);
            write += 1;
        }
    }
    write
}

/// Description: Compacts a short scan with three dropouts and prints the result.
/// Inputs: none
/// Outputs: none
fn main() {
    let mut scan = [0, 1_200, 0, 0, 850, 3_100];
    let valid = compact_scan(&mut scan);
    println!("{valid} valid readings: {scan:?}");
}
