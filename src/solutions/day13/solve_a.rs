use crate::solutions::Solution;
use crate::solutions::day13::common::*;

// For each of the two directions - horizontal and vertical - we start by creating
// a list of candidates; for example, for horizontal, this list contains all column
// indices except for the last one. Sticking with the horizontal direction as an
// example, we then iterate over all rows, and for each row convert the list of
// candidates to pivots, i.e. mapping the row index and the candidate's column
// index to a single set of cell coordinates. We then check if the mirror can be
// placed directly to the right of this pivot (or below it for vertical) by moving
// left and right from this pivot step by step, and at each step checking if these
// two cells contain the same value. If this iteration leaves the grid before we've
// found a difference, the candidate is valid, at least for this particular row.
//
// After each row, we remove the candidates that were invalid for that row, and
// continue with the next row. This does mean that we have to check every row,
// but since the number of potential candidates shrinks very quickly, we have
// less and less work to do per row. In most cases, the list of candidates is
// reduced to just one after two or three rows. If we've still got exactly one
// valid candidate after checking all rows, we've found our answer; otherwise,
// we check the vertical direction by iterating over all columns.

fn is_symmetric(grid: &Grid, pivot: (isize, isize), direction: &Direction) -> bool {
    for offset in 0 .. grid.cols {
        let coordinates_a: (isize, isize) = match direction {
            Direction::Horizontal => (pivot.0, pivot.1 - offset),
            Direction::Vertical   => (pivot.0 - offset, pivot.1),
        };

        let coordinates_b: (isize, isize) =match direction {
            Direction::Horizontal => (pivot.0, pivot.1 + offset + 1),
            Direction::Vertical   => (pivot.0 + offset + 1, pivot.1),
        };
    
        if !grid.contains(&coordinates_b) || !grid.contains(&coordinates_a) {
            return true;
        }

        let value_a = grid.get(&coordinates_a);
        let value_b = grid.get(&coordinates_b);

        if value_a != value_b {
            return false;
        }
    }

    return true;
}

fn filter_candidates(grid: &Grid, index: isize, candidates: &Vec<isize>, direction: &Direction) -> Vec<isize> {
    candidates.into_iter().filter(|&candidate| {
        let pivot: (isize, isize) = match direction {
            Direction::Horizontal => (index, *candidate),
            Direction::Vertical   => (*candidate, index),
        };

        return is_symmetric(grid, pivot, direction);
    }).map(|c| *c).collect()
}

fn find_mirror_line(grid: &Grid, direction: Direction) -> Option<isize> {
    let index_range: std::ops::Range<isize> = grid.index_range(&direction);
    let other_range: std::ops::Range<isize> = grid.index_range(&direction.other());
    let mut candidates: Vec<isize> = (other_range.start .. other_range.end - 1).collect();

    for index in index_range {
        candidates = filter_candidates(grid, index, &candidates, &direction);
        
        if candidates.is_empty() {
            return None;
        }
    }

    if candidates.len() == 1 {
        return Some(candidates[0])
    } else {
        return None
    }
}

fn solve_block(lines: &[String]) -> i64 {
    let grid: Grid = Grid::from_lines(lines);
    
    if let Some(col) = find_mirror_line(&grid, Direction::Horizontal) {
        return (col + 1) as i64;
    } else if let Some(row) = find_mirror_line(&grid, Direction::Vertical) {
        return (row + 1) as i64 * 100;
    }
    
    panic!("No symmetry found");
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let blocks: Vec<&[String]> = lines.split(|line| line.is_empty()).collect();
    let result: i64 = blocks.into_iter().map(|block| solve_block(block)).sum();
    return Solution::Integer(result)
}
