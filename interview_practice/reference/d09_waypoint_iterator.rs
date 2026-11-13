//! Stores route waypoints in a binary search tree and walks them in order with an iterator.
//!
//! APIs:
//! - `Waypoints`: a binary search tree of waypoint distances
//! - `Waypoints::new`: an empty tree
//! - `Waypoints::insert`: add a waypoint
//! - `Waypoints::contains`: check for a waypoint
//! - `Waypoints::iter`: walk the waypoints from nearest to farthest
//! - `InOrder`: the in-order iterator
//! - `InOrder::has_next`: whether another waypoint remains

use std::cmp::Ordering;

/// Description: One tree node.
/// Fields:
/// - `meters`: the waypoint's distance along the route.
/// - `left`: the subtree of nearer waypoints.
/// - `right`: the subtree of farther waypoints.
struct Node {
    meters: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

/// Description: A binary search tree of waypoint distances with no duplicates.
/// Fields:
/// - `root`: the root node, or `None` for an empty tree.
#[derive(Default)]
pub struct Waypoints {
    root: Option<Box<Node>>,
}

impl Waypoints {
    /// Description: Builds an empty tree.
    /// Inputs: none
    /// Outputs: the new tree.
    pub fn new() -> Self {
        Self::default()
    }

    /// Description: Adds a waypoint by walking down from the root to an empty link.
    /// Inputs:
    /// - `meters`: the waypoint's distance along the route.
    /// Outputs: `true` when the waypoint was added, or `false` when it was already there.
    /// Constraints: O(height) time, and a loop rather than recursion, so a lopsided tree cannot
    /// overflow the call stack.
    pub fn insert(&mut self, meters: u32) -> bool {
        let mut link = &mut self.root;
        while let Some(node) = link {
            link = match meters.cmp(&node.meters) {
                Ordering::Less => &mut node.left,
                Ordering::Greater => &mut node.right,
                Ordering::Equal => return false,
            };
        }
        *link = Some(Box::new(Node { meters, left: None, right: None }));
        true
    }

    /// Description: Checks whether a waypoint is in the tree.
    /// Inputs:
    /// - `meters`: the distance to look for.
    /// Outputs: `true` when the tree holds that waypoint.
    /// Constraints: O(height) time.
    pub fn contains(&self, meters: u32) -> bool {
        let mut link = &self.root;
        while let Some(node) = link {
            link = match meters.cmp(&node.meters) {
                Ordering::Less => &node.left,
                Ordering::Greater => &node.right,
                Ordering::Equal => return true,
            };
        }
        false
    }

    /// Description: Starts an in-order walk of the waypoints.
    /// Inputs: none
    /// Outputs: an iterator that yields the distances from nearest to farthest.
    pub fn iter(&self) -> InOrder<'_> {
        let mut walk = InOrder { stack: Vec::new() };
        walk.push_left(self.root.as_deref());
        walk
    }
}

/// Description: Walks a tree in increasing order with an explicit stack instead of recursion.
/// Fields:
/// - `stack`: the nodes still to yield, with the next one on top. Each node's right subtree is
///   explored only after the node is yielded.
/// Constraints: the stack holds at most one node per tree level, so memory is O(height).
pub struct InOrder<'a> {
    stack: Vec<&'a Node>,
}

impl<'a> InOrder<'a> {
    /// Description: Pushes a node and then its chain of left children, so the smallest ends up on
    /// top.
    /// Inputs:
    /// - `node`: the subtree to descend, or `None`.
    /// Outputs: none
    fn push_left(&mut self, mut node: Option<&'a Node>) {
        while let Some(current) = node {
            self.stack.push(current);
            node = current.left.as_deref();
        }
    }

    /// Description: Reports whether the walk has another waypoint.
    /// Inputs: none
    /// Outputs: `true` while waypoints remain.
    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

impl<'a> Iterator for InOrder<'a> {
    type Item = u32;

    /// Description: Yields the next waypoint and queues the nodes of its right subtree.
    /// Inputs: none
    /// Outputs: the next distance, or `None` once the walk is done.
    /// Constraints: O(1) amortized, because each node is pushed and popped once.
    fn next(&mut self) -> Option<u32> {
        let node = self.stack.pop()?;
        self.push_left(node.right.as_deref());
        Some(node.meters)
    }
}

/// Description: Inserts waypoints out of order and prints them in route order.
/// Inputs: none
/// Outputs: none
fn main() {
    let mut route = Waypoints::new();
    for meters in [500, 120, 900, 60, 300] {
        route.insert(meters);
    }
    let in_order: Vec<u32> = route.iter().collect();
    println!("waypoints in route order: {in_order:?}");
}
