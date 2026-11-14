//! Keeps a robot's most recently used map tiles in a fixed-size least-recently-used cache.
//!
//! APIs:
//! - `TileCache`: an LRU cache from tile id to traversal cost
//! - `TileCache::new`: an empty cache with a fixed capacity
//! - `TileCache::len`: the number of cached tiles
//! - `TileCache::is_empty`: whether nothing is cached
//! - `TileCache::get`: read a tile and mark it most recent
//! - `TileCache::put`: store a tile, evicting the least recent one when full

use std::collections::HashMap;

/// Description: One cache entry, stored in a slot of the cache's node vector.
/// Fields:
/// - `key`: the tile id.
/// - `value`: the tile's traversal cost.
/// - `prev`: the slot of the next more recent entry, or `None` for the most recent entry.
/// - `next`: the slot of the next less recent entry, or `None` for the least recent entry.
struct Node {
    key:   u32,
    value: u32,
    prev:  Option<usize>,
    next:  Option<usize>,
}

/// Description: A least-recently-used cache. A hash map finds an entry's slot in O(1), and a
/// doubly linked list threaded through a vector keeps the entries in recency order. The links
/// are vector indices, so the list needs no `unsafe`, `Rc` or `RefCell`.
/// Fields:
/// - `capacity`: the most entries the cache holds.
/// - `slots`: the map from tile id to the slot that holds it.
/// - `nodes`: the entries, linked from most recent to least recent.
/// - `head`: the slot of the most recent entry.
/// - `tail`: the slot of the least recent entry.
pub struct TileCache {
    capacity: usize,
    slots:    HashMap<u32, usize>,
    nodes:    Vec<Node>,
    head:     Option<usize>,
    tail:     Option<usize>,
}

/// Description: Fills a two-tile cache, touches the older tile and shows which tile is evicted.
/// Inputs: none
/// Outputs: none
fn main() {
    let mut cache = TileCache::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    cache.get(1);

    // Compressed:
    // println!("putting tile 3 evicts {:?}", cache.put(3, 30));
    let evicted_tile = cache.put(3, 30);
    println!("putting tile 3 evicts {evicted_tile:?}");

    // Compressed:
    // println!("tile 2 is now {:?}, tile 1 is {:?}", cache.get(2), cache.get(1));
    let tile_2_cost = cache.get(2);
    let tile_1_cost = cache.get(1);
    println!("tile 2 is now {tile_2_cost:?}, tile 1 is {tile_1_cost:?}");
}

impl TileCache {
    /// Description: Builds an empty cache.
    /// Inputs:
    /// - `capacity`: the most tiles the cache holds.
    /// Outputs: the new cache.
    /// Constraints: panics when `capacity` is zero.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "a cache must hold at least one tile");

        TileCache {
            capacity,
            slots: HashMap::new(),
            nodes: Vec::with_capacity(capacity),
            head:  None,
            tail:  None,
        }
    }

    /// Description: Stores a tile as the most recently used. A tile that is already cached gets
    /// its new cost. A new tile in a full cache replaces the least recently used tile.
    /// Inputs:
    /// - `key`: the tile id.
    /// - `value`: the tile's traversal cost.
    /// Outputs: the id of the evicted tile, or `None` when nothing was evicted.
    /// Constraints: O(1) on average.
    pub fn put(&mut self, key: u32, value: u32) -> Option<u32> {
        if let Some(&slot) = self.slots.get(&key) {
            self.nodes[slot].value = value;
            self.move_to_front(slot);
            return None;
        }

        let mut evicted = None;
        let slot        = if self.nodes.len() < self.capacity {
            // Compressed:
            // self.nodes.push(Node { key, value, prev: None, next: None });
            let node = Node {
                key,
                value,
                prev: None,
                next: None,
            };
            self.nodes.push(node);
            self.nodes.len() - 1
        } else {
            let slot = self.tail.expect("a full cache has a least recent entry");
            self.unlink(slot);

            let old_key = self.nodes[slot].key;
            self.slots.remove(&old_key);
            evicted = Some(old_key);

            self.nodes[slot].key   = key;
            self.nodes[slot].value = value;
            slot
        };

        self.push_front(slot);
        self.slots.insert(key, slot);
        evicted
    }

    /// Description: Reads a tile and marks it as the most recently used.
    /// Inputs:
    /// - `key`: the tile id.
    /// Outputs: the tile's traversal cost, or `None` when the tile is not cached.
    /// Constraints: O(1) on average.
    pub fn get(&mut self, key: u32) -> Option<u32> {
        // Compressed:
        // let slot = *self.slots.get(&key)?;
        let cached_slot = self.slots.get(&key)?;
        let slot        = *cached_slot;

        self.move_to_front(slot);
        Some(self.nodes[slot].value)
    }

    /// Description: Makes a cached slot the most recent entry.
    /// Inputs:
    /// - `slot`: the slot that was just used.
    /// Outputs: none
    fn move_to_front(&mut self, slot: usize) {
        if self.head != Some(slot) {
            self.unlink(slot);
            self.push_front(slot);
        }
    }

    /// Description: Removes a slot from the recency list and joins its neighbours together.
    /// Inputs:
    /// - `slot`: the slot to detach.
    /// Outputs: none
    fn unlink(&mut self, slot: usize) {
        let prev = self.nodes[slot].prev;
        let next = self.nodes[slot].next;

        match prev {
            Some(p) => self.nodes[p].next = next,
            None => self.head = next,
        }
        match next {
            Some(n) => self.nodes[n].prev = prev,
            None => self.tail = prev,
        }
    }

    /// Description: Links a detached slot in as the most recent entry.
    /// Inputs:
    /// - `slot`: the slot to attach.
    /// Outputs: none
    fn push_front(&mut self, slot: usize) {
        self.nodes[slot].prev = None;
        self.nodes[slot].next = self.head;

        if let Some(old_head) = self.head {
            self.nodes[old_head].prev = Some(slot);
        }

        self.head = Some(slot);
        if self.tail.is_none() {
            self.tail = Some(slot);
        }
    }

    /// Description: Returns how many tiles are cached.
    /// Inputs: none
    /// Outputs: the number of entries.
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    /// Description: Reports whether the cache holds no tiles.
    /// Inputs: none
    /// Outputs: `true` when the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }
}
