//! Finds the most frequent fault codes in a robot fleet's error log.
//!
//! APIs:
//! - `top_k_sorted`: the k most frequent codes by counting and sorting everything
//! - `top_k_heap`: the same answer with a size-k heap, for many distinct codes

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// Description: Prints the two most frequent codes in a short log, found both ways.
/// Inputs: none
/// Outputs: none
fn main() {
    let log = [404, 17, 17, 9, 404, 17, 9, 12];

    // Compressed:
    // println!("sorted: {:?}", top_k_sorted(&log, 2));
    let sorted_answer = top_k_sorted(&log, 2);
    println!("sorted: {sorted_answer:?}");

    // Compressed:
    // println!("heap:   {:?}", top_k_heap(&log, 2));
    let heap_answer = top_k_heap(&log, 2);
    println!("heap:   {heap_answer:?}");
}

/// Description: Returns the k most frequent fault codes by counting them and sorting every
/// distinct code.
/// Inputs:
/// - `codes`: the fault codes in the order the fleet reported them.
/// - `k`: how many codes to return.
/// Outputs: up to `k` codes, the most frequent first. A tie goes to the smaller code.
/// Constraints: O(n + d log d) time for n reports and d distinct codes.
pub fn top_k_sorted(codes: &[u16], k: usize) -> Vec<u16> {
    // Compressed:
    // let mut ranked: Vec<(u16, usize)> = count(codes).into_iter().collect();
    let counts                        = count(codes);
    let mut ranked: Vec<(u16, usize)> = counts.into_iter().collect();

    // Compressed:
    // ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    ranked.sort_by(|a, b| {
        let higher_count_first = b.1.cmp(&a.1);
        let smaller_code_first = a.0.cmp(&b.0);
        higher_count_first.then(smaller_code_first)
    });

    // Compressed:
    // ranked.into_iter().take(k).map(|(code, _)| code).collect()
    let best_k        = ranked.into_iter().take(k);
    let mut top_codes = Vec::new();
    for (code, _) in best_k {
        top_codes.push(code);
    }
    top_codes
}

/// Description: Returns the same answer as `top_k_sorted`, but keeps only the best k codes in a
/// min-heap while it scans the counts.
/// Inputs:
/// - `codes`: the fault codes in the order the fleet reported them.
/// - `k`: how many codes to return.
/// Outputs: up to `k` codes, the most frequent first. A tie goes to the smaller code.
/// Constraints: O(n + d log k) time and O(d) memory for the counts.
pub fn top_k_heap(codes: &[u16], k: usize) -> Vec<u16> {
    let mut heap = BinaryHeap::with_capacity(k + 1);
    for (code, n) in count(codes) {
        // Compressed:
        // heap.push(Reverse((n, Reverse(code))));
        let tie_breaker    = Reverse(code);
        let rank           = (n, tie_breaker);
        let min_heap_entry = Reverse(rank);
        heap.push(min_heap_entry);
        if heap.len() > k {
            heap.pop();
        }
    }

    // Compressed:
    // let mut best: Vec<(usize, Reverse<u16>)> = heap.into_iter().map(|Reverse(rank)| rank)
    //     .collect();
    let mut best: Vec<(usize, Reverse<u16>)> = Vec::new();
    for Reverse(rank) in heap {
        best.push(rank);
    }
    best.sort_unstable_by(|a, b| b.cmp(a));

    // Compressed:
    // best.into_iter().map(|(_, Reverse(code))| code).collect()
    let mut top_codes = Vec::new();
    for (_, Reverse(code)) in best {
        top_codes.push(code);
    }
    top_codes
}

/// Description: Counts how often each fault code appears.
/// Inputs:
/// - `codes`: the fault codes in the order the fleet reported them.
/// Outputs: a map from each code to its count.
fn count(codes: &[u16]) -> HashMap<u16, usize> {
    let mut counts = HashMap::new();
    for &code in codes {
        // Compressed:
        // *counts.entry(code).or_insert(0) += 1;
        let code_count = counts.entry(code).or_insert(0);
        *code_count   += 1;
    }
    counts
}
