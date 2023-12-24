use std::collections::HashMap;
use std::collections::HashSet;

use crate::solutions::Solution;

#[derive(Eq, PartialEq, Hash)]
enum Seed {
    TL,
    TR,
    BL,
    BR,
    C,
    L,
    R,
    B,
    T
}

struct Grid {
    tiles: Vec<bool>,
    rows: isize,
    cols: isize
}

impl Grid {
    fn parse(lines: &Vec<String>) -> Grid {
        let tiles: Vec<bool> = lines.concat().chars().map(|c| c != '#').collect();
        Grid { tiles: tiles, rows: lines.len() as isize, cols: lines[0].len() as isize }
    }

    fn contains(&self, pos: &(isize, isize)) ->  bool {
        pos.0 >= 0 && pos.0 < self.rows && pos.1 >= 0 && pos.1 < self.cols
    }

    fn is_empty(&self, pos: &(isize, isize)) -> bool {
        let index: usize = (pos.0 * self.cols + pos.1) as usize;
        self.tiles[index]
    }
}

fn add_next(grid: &Grid, next: &mut HashSet<(isize, isize)>, pos: (isize, isize)) {
    if grid.contains(&pos) && grid.is_empty(&pos) {
        next.insert(pos);
    }
}

fn step(grid: &Grid, current: HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
    let mut next: HashSet<(isize, isize)> = HashSet::new();

    for pos in current {
        add_next(grid, &mut next, (pos.0 + 1, pos.1));
        add_next(grid, &mut next, (pos.0 - 1, pos.1));
        add_next(grid, &mut next, (pos.0, pos.1 + 1));
        add_next(grid, &mut next, (pos.0, pos.1 - 1));
    }

    next
}

fn expand(grid: &Grid, seed: (isize, isize)) -> Vec<usize> {
    let mut current: HashSet<(isize, isize)> = HashSet::from([seed]);
    let mut counts: Vec<usize> = Vec::new();

    loop {
        current = step(grid, current);
        counts.push(current.len());
        
        if counts.len() % 2 == 0 && counts.len() >= 4 &&
            counts[counts.len() - 1] == counts[counts.len() - 3] &&
            counts[counts.len() - 2] == counts[counts.len() - 4] {
                println!("{:?}", counts);
                return counts;
        }
    }
}

fn get_count(counts: &Vec<usize>, steps: usize) -> usize {
    if steps >= counts.len() {
        if steps % 2 == 0 {
            counts[counts.len() - 1]
        } else { counts[counts.len() - 2] }
    } else {
        counts[steps]
    }
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let target_steps: usize = 123456;
    let grid: Grid = Grid::parse(lines);

    let min_row: isize = 0;
    let min_col: isize = 0;
    let max_row: isize = grid.rows - 1;
    let max_col: isize = grid.cols - 1;
    let mid_row: isize = max_row / 2;
    let mid_col: isize = max_col / 2;

    let mut seed_counts: HashMap<Seed, Vec<usize>> = HashMap::new();

    seed_counts.insert(Seed::TL, expand(&grid, (min_row, min_col)));
    seed_counts.insert(Seed::BL, expand(&grid, (max_row, min_col)));
    seed_counts.insert(Seed::TR, expand(&grid, (min_row, max_col)));
    seed_counts.insert(Seed::BR, expand(&grid, (max_row, max_col)));
    seed_counts.insert(Seed::C,  expand(&grid, (mid_row, mid_col)));
    seed_counts.insert(Seed::T,  expand(&grid, (min_row, mid_col)));
    seed_counts.insert(Seed::B,  expand(&grid, (max_row, mid_col)));
    seed_counts.insert(Seed::L,  expand(&grid, (mid_row, min_col)));
    seed_counts.insert(Seed::R,  expand(&grid, (mid_row, max_col)));

    let mut count: usize = get_count(seed_counts.get(&Seed::C).unwrap(), target_steps);

    return Solution::Integer(count as i64)
}

// -----