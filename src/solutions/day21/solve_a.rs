use std::collections::HashSet;

use crate::solutions::Solution;
use crate::solutions::day21::common::*;

// A straightforward solution, simply perform the simulation as described,
// and after 64 steps count the number of active cells. It's not particu-
// larly fast, but there doesn't seem to be a lot of room for optimization.

pub fn solve(lines: &Vec<String>) -> Solution {
    let grid: Grid = Grid::parse(lines);
    let start: (isize, isize) = grid.find_start();
    let mut current: HashSet<(isize, isize)> = HashSet::from([start]);

    for _ in 0 .. 64 {
        current = step(&grid, current);
    }

    return Solution::Integer(current.len() as i64)
}
