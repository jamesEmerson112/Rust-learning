//! Finds the clusters of obstacle cells in a robot's occupancy grid.
//!
//! APIs:
//! - `cluster_sizes`: the size of every obstacle cluster, largest first

use std::collections::VecDeque;

/// Description: Counts the obstacle clusters in a small grid and prints their sizes.
/// Inputs: none
/// Outputs: none
fn main() {
    let grid  = ["##..#", "#...#", "..#..", "....#"];
    let sizes = cluster_sizes(&grid);

    // Compressed:
    // println!("{} clusters, sizes {sizes:?}", sizes.len());
    let cluster_count = sizes.len();
    println!("{cluster_count} clusters, sizes {sizes:?}");
}

/// Description: Finds every cluster of obstacle cells that touch up, down, left or right, with a
/// breadth-first search from each obstacle not yet seen. This is the Number of Islands problem.
/// Inputs:
/// - `grid`: the occupancy grid, one string per row, where `#` is an obstacle and `.` is free.
/// Outputs: the number of cells in each cluster, largest first. The length is the cluster count.
/// Constraints: O(rows × cols) time and memory. Cells that touch only at a corner are separate
/// clusters. Panics when a row is shorter than the first row.
pub fn cluster_sizes(grid: &[&str]) -> Vec<usize> {
    // Compressed:
    // let cells: Vec<&[u8]> = grid.iter().map(|row| row.as_bytes()).collect();
    let mut cells: Vec<&[u8]> = Vec::new();
    for row in grid {
        let row_bytes = row.as_bytes();
        cells.push(row_bytes);
    }

    let rows = cells.len();
    // Compressed:
    // let cols = cells.first().map_or(0, |row| row.len());
    let cols = match cells.first() {
        Some(first_row) => first_row.len(),
        None => 0,
    };

    // Compressed:
    // let mut seen = vec![vec![false; cols]; rows];
    let unseen_row = vec![false; cols];
    let mut seen   = vec![unseen_row; rows];
    let mut sizes  = Vec::new();

    for r in 0..rows {
        for c in 0..cols {
            // Compressed:
            // if cells[r][c] != b'#' || seen[r][c] {
            //     continue;
            // }
            let free_or_seen = cells[r][c] != b'#' || seen[r][c];
            if free_or_seen {
                continue;
            }

            seen[r][c]    = true;
            let mut queue = VecDeque::from([(r, c)]);
            let mut size  = 0;
            while let Some((row, col)) = queue.pop_front() {
                size += 1;
                // Compressed:
                // for (nr, nc) in neighbours(row, col, rows, cols) {
                //     if cells[nr][nc] == b'#' && !seen[nr][nc] {
                //         seen[nr][nc] = true;
                //         queue.push_back((nr, nc));
                //     }
                // }
                let neighbour_cells = neighbours(
                    row,
                    col,
                    rows,
                    cols,
                );
                for (nr, nc) in neighbour_cells {
                    let unseen_obstacle = cells[nr][nc] == b'#' && !seen[nr][nc];
                    if unseen_obstacle {
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
    // Compressed:
    // [
    //     (row.checked_sub(1), Some(col)),
    //     (Some(row + 1).filter(|&r| r < rows), Some(col)),
    //     (Some(row), col.checked_sub(1)),
    //     (Some(row), Some(col + 1).filter(|&c| c < cols)),
    // ]
    // .into_iter()
    // .filter_map(|(r, c)| Some((r?, c?)))
    let row_above = row.checked_sub(1);
    let next_row  = row + 1;
    let row_below = if next_row < rows { Some(next_row) } else { None };
    let col_left  = col.checked_sub(1);
    let next_col  = col + 1;
    let col_right = if next_col < cols { Some(next_col) } else { None };

    let candidates = [
        (row_above, Some(col)),
        (row_below, Some(col)),
        (Some(row), col_left),
        (Some(row), col_right),
    ];
    candidates.into_iter().filter_map(|candidate| match candidate {
        (Some(r), Some(c)) => Some((r, c)),
        _ => None,
    })
}
