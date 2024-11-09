use std::collections::HashSet;

use crate::solutions::Solution;
use crate::solutions::day03::common::*;

// First find the coordinates of all symbols in the grid, then for each symbol find
// all surrounding digits. For each of those digit coordinates, find the start coordinates
// of the corresponding number, and (after eliminating duplicate number start coordinates)
// find the corresponding number end coordinates, and parse the numeric substrings. The
// initial conversion to a grid makes this solution rather slow, but having a proper
// grid implementation that supports both 1D indices and 2D coordinates (via tuples)
// makes life easier overall. My original approach was to first find the numbers and
// check if they had any surrounding symbols; while potentially faster (especially
// when using a set for the symbol coordinates), getting all positions surrounding
// the numeric strings felt a bit fiddly; iterating over the symbols is cleaner,
// and allows for more code reuse with the B part.

// ADDENDUM: Immediately after committing the first solution, I figured out that it
//   was probably slow because I was storing the grid contents as a string, and getting
//   the Nth character of a string is an O(N) operation. Sure enough, doing a one-time
//   conversion from string to vector of `char`s reduced runtime roughly 200 times.

fn is_symbol(c: &char) -> bool { !(c.is_digit(10) || *c == '.') }

fn find_part_numbers(grid: &Grid, symbols: &Vec<(isize, isize)>) -> HashSet<(isize, isize)> {
    let mut number_starts: HashSet<(isize, isize)> = HashSet::new();

    for symbol in symbols {
        grid.get_neighbors(symbol).into_iter()
            .filter(|c| grid.get(c).is_digit(10))
            .map(|cn| find_number_start(grid, &cn))
            .for_each(|start| { number_starts.insert(start); });
    }

    number_starts
}

fn find_symbols(grid: &Grid) -> Vec<(isize, isize)> {
    grid.content.iter().enumerate()
        .filter(|(_, ch)| is_symbol(ch))
        .map(|(i, _)| grid.to_coordinates(i as isize)).collect()
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let grid: Grid = Grid::from_lines(lines);
    let symbols: Vec<(isize, isize)> = find_symbols(&grid);
    let number_starts: HashSet<(isize, isize)> = find_part_numbers(&grid, &symbols);
    let result: i64 = number_starts.iter().map(|start| parse_number(&grid, start)).sum();
    
    return Solution::Integer(result)
}
