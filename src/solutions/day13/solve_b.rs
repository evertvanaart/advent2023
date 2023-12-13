use crate::solutions::Solution;
use crate::solutions::day13::common::*;

// Similar to the A part. Instead of the candidates consisting only of an
// index (row index or column index), we now also keep track of the number
// of errors encountered for this candidate during the iteration. Whenever
// we find an error (i.e., a spot where the two potentially mirrored cells
// do not contain the same value), we increase this counter, and we do not
// reset this counter between rows. If the error counter of a candidate ever
// exceeds one, this candidate can be eliminated. At the end, we check if
// any of the remaining candidates had exactly one error (ignoring the one
// with zero errors, if any). If there is exactly one such candidate, we've
// found the answer, otherwise we check the other direction.

struct Candidate {
    index: isize,
    errors: usize
}

impl Candidate {
    fn new(index: isize) -> Candidate {
        Candidate { index: index, errors: 0 }
    }
}

fn count_errors(grid: &Grid, pivot: (isize, isize), direction: &Direction) -> usize {
    let mut errors: usize = 0;

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
            return errors;
        }

        let value_a = grid.get(&coordinates_a);
        let value_b = grid.get(&coordinates_b);

        if value_a != value_b {
            errors += 1;

            if errors >= 2 {
                return errors;
            }
        }
    }

    return errors;
}

fn update_candidates(candidates: &mut Vec<Candidate>, grid: &Grid, index: isize, direction: &Direction) {
    for candidate in candidates {
        let pivot: (isize, isize) = match direction {
            Direction::Horizontal => (index, candidate.index),
            Direction::Vertical   => (candidate.index, index),
        };

        candidate.errors += count_errors(grid, pivot, direction);
    }
}

fn find_secondary_line(grid: &Grid, direction: Direction) -> Option<isize> {
    let index_range: std::ops::Range<isize> = grid.index_range(&direction);
    let other_range: std::ops::Range<isize> = grid.index_range(&direction.other());

    let mut candidates: Vec<Candidate> = (other_range.start .. other_range.end - 1)
        .map(|index| Candidate::new(index)).collect();

    for index in index_range {
        update_candidates(&mut candidates, grid, index, &direction);

        candidates = candidates.into_iter()
            .filter(|c| c.errors <= 1).collect();
        
        if candidates.is_empty() {
            return None;
        }
    }

    candidates = candidates.into_iter()
        .filter(|c| c.errors == 1).collect();

    if candidates.len() == 1 {
        return Some(candidates[0].index)
    } else {
        return None
    }
}

fn solve_block(lines: &[String]) -> i64 {
    let grid: Grid = Grid::from_lines(lines);
    
    if let Some(col) = find_secondary_line(&grid, Direction::Horizontal) {
        return (col + 1) as i64;
    } else if let Some(row) = find_secondary_line(&grid, Direction::Vertical) {
        return (row + 1) as i64 * 100;
    }
    
    panic!("No symmetry found");
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let blocks: Vec<&[String]> = lines.split(|line| line.is_empty()).collect();
    let result: i64 = blocks.into_iter().map(|block| solve_block(block)).sum();
    return Solution::Integer(result)
}
