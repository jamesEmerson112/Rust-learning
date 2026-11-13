//! Tests for the map-tile LRU cache in `src/bin/d07_lru_cache.rs`.
//!
//! Contents:
//! - basic tests: misses, hits and the size count
//! - eviction tests: least recent goes first, reads and updates refresh recency
//! - model test: a long run checked against a slow but obvious model

#[path = "../src/bin/d07_lru_cache.rs"]
#[allow(dead_code)]
mod d07_lru_cache;

use d07_lru_cache::TileCache;

/// Description: A tile that was never stored is a miss.
/// Inputs: none
/// Outputs: none
#[test]
fn a_missing_tile_is_a_miss() {
    let mut cache = TileCache::new(2);
    assert!(cache.is_empty());
    assert_eq!(cache.get(7), None);
}

/// Description: A stored tile reads back with its cost.
/// Inputs: none
/// Outputs: none
#[test]
fn a_stored_tile_reads_back() {
    let mut cache = TileCache::new(2);
    assert_eq!(cache.put(1, 10), None);
    assert_eq!(cache.get(1), Some(10));
    assert_eq!(cache.len(), 1);
}

/// Description: A third tile in a two-tile cache evicts the tile stored first.
/// Inputs: none
/// Outputs: none
#[test]
fn a_full_cache_evicts_the_least_recent_tile() {
    let mut cache = TileCache::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    assert_eq!(cache.put(3, 30), Some(1));
    assert_eq!(cache.get(1), None);
    assert_eq!(cache.get(2), Some(20));
    assert_eq!(cache.get(3), Some(30));
    assert_eq!(cache.len(), 2);
}

/// Description: Reading the older tile makes the other tile the next one evicted.
/// Inputs: none
/// Outputs: none
#[test]
fn a_read_refreshes_recency() {
    let mut cache = TileCache::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    cache.get(1);
    assert_eq!(cache.put(3, 30), Some(2));
    assert_eq!(cache.get(1), Some(10));
}

/// Description: Storing a cached tile again updates its cost, evicts nothing and refreshes it.
/// Inputs: none
/// Outputs: none
#[test]
fn an_update_refreshes_without_evicting() {
    let mut cache = TileCache::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    assert_eq!(cache.put(1, 11), None);
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.put(3, 30), Some(2));
    assert_eq!(cache.get(1), Some(11));
}

/// Description: A one-tile cache replaces its only tile on every new store.
/// Inputs: none
/// Outputs: none
#[test]
fn a_one_tile_cache_keeps_only_the_newest() {
    let mut cache = TileCache::new(1);
    cache.put(1, 10);
    assert_eq!(cache.put(2, 20), Some(1));
    assert_eq!(cache.get(1), None);
    assert_eq!(cache.get(2), Some(20));
}

/// Description: Runs 1,000 mixed reads and stores against the cache and against a vector kept in
/// recency order, and checks that every answer and every eviction matches.
/// Inputs: none
/// Outputs: none
#[test]
fn matches_a_simple_model_over_a_long_run() {
    let mut cache = TileCache::new(4);
    let mut model: Vec<(u32, u32)> = Vec::new();
    let mut seed: u32 = 7;
    for step in 0..1_000u32 {
        seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        let key = (seed >> 16) % 8;
        if (seed >> 8) % 3 == 0 {
            let expected = model.iter().position(|&(k, _)| k == key).map(|i| {
                let entry = model.remove(i);
                model.insert(0, entry);
                entry.1
            });
            assert_eq!(cache.get(key), expected);
        } else {
            let evicted = if let Some(i) = model.iter().position(|&(k, _)| k == key) {
                model.remove(i);
                None
            } else if model.len() == 4 {
                model.pop().map(|(k, _)| k)
            } else {
                None
            };
            model.insert(0, (key, step));
            assert_eq!(cache.put(key, step), evicted);
        }
        assert_eq!(cache.len(), model.len());
    }
}
