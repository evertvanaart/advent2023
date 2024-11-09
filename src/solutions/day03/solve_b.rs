use std::collections::HashSet;

use crate::solutions::Solution;
use crate::solutions::day03::common::*;

// Using the grid created in the A part, as well as the helper functions for
// finding the start and end of a numeric string in this grid, this B part became
// fairly straightforward. Instead of all symbols, first find only the coordinates
// of all star symbols, then find the surrounding numbers; parse and multiply these
// numbers if and only if there are exactly two. Finding the surrounding numbers
// is less efficient than it could have been: the conversion to a set in particular
// is only needed to deal with one specific edge-case (two diagonally adjecent
// numbers separate by a period directly above or below the star symbol), and
// explicitly handling this edge case would've been more efficient.

fn compute_gear_ratio(grid: &Grid, star: &(isize, isize)) -> i64 {
    let digit_neighbors: Vec<(isize, isize)> = grid.get_neighbors(star).into_iter()
        .filter(|c| grid.get(c).is_digit(10)).collect();

    let number_starts: HashSet<(isize, isize)> = HashSet::from_iter(digit_neighbors.iter()
        .map(|cn| find_number_start(grid, cn)));

    if number_starts.len() != 2 {
        return 0
    }

    number_starts.iter().map(|start| parse_number(grid, start)).product()
}

fn find_stars(grid: &Grid) -> Vec<(isize, isize)> {
    grid.content.iter().enumerate()
        .filter(|(_, &ch)| ch == '*')
        .map(|(i, _)| grid.to_coordinates(i as isize)).collect()
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let grid: Grid = Grid::from_lines(lines);
    let stars: Vec<(isize, isize)> = find_stars(&grid);
    let result: i64 = stars.iter().map(|star| compute_gear_ratio(&grid, star)).sum();
    return Solution::Integer(result)
}
