//! Finds the clusters of obstacle cells in a robot's occupancy grid.
//!
//! APIs:
//! - `cluster_sizes`: the size of every obstacle cluster, largest first

use std::collections::VecDeque;

/// Description: Lists the cells above, below, left of and right of a cell that lie inside the
/// grid.
/// Inputs:
/// - `row`, `col`: the cell.
/// - `rows`, `cols`: the grid size.
/// Outputs: an iterator over up to four neighbouring cells.
/// Constraints: `checked_sub` keeps a cell in row or column 0 from wrapping around to
/// `usize::MAX`.
fn neighbours(
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) -> impl Iterator<Item = (usize, usize)> {
    [
        (row.checked_sub(1), Some(col)),
        (Some(row + 1).filter(|&r| r < rows), Some(col)),
        (Some(row), col.checked_sub(1)),
        (Some(row), Some(col + 1).filter(|&c| c < cols)),
    ]
    .into_iter()
    .filter_map(|(r, c)| Some((r?, c?)))
}

/// Description: Finds every cluster of obstacle cells that touch up, down, left or right, with a
/// breadth-first search from each obstacle not yet seen. This is the Number of Islands problem.
/// Inputs:
/// - `grid`: the occupancy grid, one string per row, where `#` is an obstacle and `.` is free.
/// Outputs: the number of cells in each cluster, largest first. The length is the cluster count.
/// Constraints: O(rows × cols) time and memory. Cells that touch only at a corner are separate
/// clusters. Panics when a row is shorter than the first row.
pub fn cluster_sizes(grid: &[&str]) -> Vec<usize> {
    let cells: Vec<&[u8]> = grid.iter().map(|row| row.as_bytes()).collect();
    let rows = cells.len();
    let cols = cells.first().map_or(0, |row| row.len());
    let mut seen = vec![vec![false; cols]; rows];
    let mut sizes = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            if cells[r][c] != b'#' || seen[r][c] {
                continue;
            }
            seen[r][c] = true;
            let mut queue = VecDeque::from([(r, c)]);
            let mut size = 0;
            while let Some((row, col)) = queue.pop_front() {
                size += 1;
                for (nr, nc) in neighbours(row, col, rows, cols) {
                    if cells[nr][nc] == b'#' && !seen[nr][nc] {
                        seen[nr][nc] = true;
                        queue.push_back((nr, nc));
                    }
                }
            }
            sizes.push(size);
        }
    }
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes
}

/// Description: Counts the obstacle clusters in a small grid and prints their sizes.
/// Inputs: none
/// Outputs: none
fn main() {
    let grid = ["##..#", "#...#", "..#..", "....#"];
    let sizes = cluster_sizes(&grid);
    println!("{} clusters, sizes {sizes:?}", sizes.len());
}
