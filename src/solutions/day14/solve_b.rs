use std::collections::HashMap;

use crate::solutions::Solution;
use crate::solutions::day14::common::*;

// Clearly we're not going to simulate every single step - even at a brisk
// ten microseconds per step this would take hours - so instead we look for
// loops (or "cycles", but the term "cycle" is already used in this problem
// to mean a sequence of four roll actions, one in each direction). We assume
// that eventually, the state of the grid will start to loop; if we can find
// such a loop, we can skip most of the remaining cycles, and only have to
// simulate a relatively small remainder to get us to the target count.
//
// This is straightforward in theory - extending the A part to support rolling
// in all four directions wasn't too difficult - but I got hung up on how to
// detect these loops. The correct way would be to create a map with the grid
// state as key and the current cycle count as value, and then check if a new
// grid state was already encountered; if so, the stored cycle count and the
// current cycle count would define the boundaries of our loop.
//
// The issue here is that this key - the grid state - is very large, so I was
// concerned about performance and memory usage; storing and looking up a key
// consisting of 10k+ characters seemed like it would be slow. The total load
// on the north pillar provided a potential alternative for the state, but it
// wasn't unique enough; in the sample, the north pillar load is the same for
// the second, third, and fourth cycle, even though the actual state differs.
//
// Instead, I ended up using a combination of the north pillar load and the
// west pillar load - which is trivially easy to calculate - as an indicator
// of the grid state. I'm not sure if this always correct; it's entirely
// possible that two different grid states produce the exact same set of
// north and west loads, potentially resulting in a false positive when
// searching for loops. However, it does work for both the sample and the
// actual input. As an added measure (which wouldn't add too much to the
// runtime), we could consider verifying the loop before calculating the
// result, i.e. after finding a potential loop of size N, first check if
// the next N load results all match the preceding N results.

const TARGET_CYCLES: usize = 1000000000;

/* -------------------------------- Direction ------------------------------- */

enum Direction {
    North,
    East,
    South,
    West
}

/* ---------------------------------- Loads --------------------------------- */

struct Loads {
    north: i64,
    west: i64
}

impl Loads {
    fn zero() -> Loads {
        Loads { north: 0, west: 0 }
    }
}

impl std::iter::Sum for Loads {
    fn sum<I: Iterator<Item = Loads>>(iter: I) -> Self {
        iter.fold(Loads { north: 0, west: 0 }, |acc, val| {
            Loads { north: acc.north + val.north, west: acc.west + val.west }
        })
    }
}

/* -------------------------------- CycleLoop ------------------------------- */

struct CycleLoop {
    start_index: usize,
    end_index: usize
}

/* ------------------------------- Main logic ------------------------------- */

fn move_stone(c: &(isize, isize), direction: &Direction) -> (isize, isize) {
    match direction {
        Direction::North => (c.0 - 1, c.1),
        Direction::East  => (c.0, c.1 + 1),
        Direction::South => (c.0 + 1, c.1),
        Direction::West  => (c.0, c.1 - 1)
    }
}

fn roll_stone(grid: &mut Grid, row: isize, col: isize, direction: &Direction) -> Loads {
    let mut c: (isize, isize) = (row, col);
    let field: &Field = grid.get(&c);

    match field {
        Field::Empty      => { return Loads::zero(); }
        Field::SquareRock => { return Loads::zero(); }
        Field::RoundRock  => { }
    }

    let c_start: (isize, isize) = (c.0, c.1);

    loop {
        let nc: (isize, isize) = move_stone(&c, direction);

        if !grid.contains(&nc) {
            break;
        }

        let nfield: &Field = grid.get(&nc);

        match nfield {
            Field::RoundRock  => { break; }
            Field::SquareRock => { break; }
            Field::Empty      => { c = (nc.0, nc.1); }
        }
    }

    if c != c_start {
        grid.set(&c_start, Field::Empty);
        grid.set(&c, Field::RoundRock);
    }

    let north_load: i64 = (grid.rows - c.0) as i64;
    let west_load: i64  = (grid.cols - c.1) as i64;
    Loads { north: north_load, west: west_load }
}

fn roll_line(grid: &mut Grid, index: isize, direction: &Direction) -> Loads{
    match direction {
        Direction::North | Direction::South => (0 .. grid.cols).map(|col| roll_stone(grid, index, col, direction)).sum(),
        Direction::East  | Direction::West  => (0 .. grid.rows).map(|row| roll_stone(grid, row, index, direction)).sum()
    }

}

fn roll_all(grid: &mut Grid, direction: &Direction) -> Loads {
    match direction {
        Direction::North => (0 .. grid.rows).map(|row| roll_line(grid, row, direction)).sum(),
        Direction::East  => (0 .. grid.cols).rev().map(|col| roll_line(grid, col, direction)).sum(),
        Direction::South => (0 .. grid.rows).rev().map(|row| roll_line(grid, row, direction)).sum(),
        Direction::West  => (0 .. grid.cols).map(|col| roll_line(grid, col, direction)).sum(),
    }
}

fn roll_cycle(grid: &mut Grid) -> Loads {
    roll_all(grid, &Direction::North);
    roll_all(grid, &Direction::West);
    roll_all(grid, &Direction::South);
    roll_all(grid, &Direction::East)
}

fn find_cycle_loop(grid: &mut Grid) -> Option<CycleLoop> {
    let mut previous: HashMap<(i64, i64), usize> = HashMap::new();
    let mut cycle_count: usize = 0;

    loop {
        let loads: Loads = roll_cycle(grid);
        let tuple: (i64, i64) = (loads.north, loads.west);

        if let Some(previous_cycle_count) = previous.get(&tuple) {
            return Some(CycleLoop {
                start_index: *previous_cycle_count,
                end_index: cycle_count
            })
        }

        previous.insert(tuple, cycle_count);
        cycle_count += 1;
    }
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let mut grid: Grid = Grid::from_lines(lines);

    if let Some(cycle_loop) = find_cycle_loop(&mut grid) {
        let cycle_loop_length: usize = cycle_loop.end_index - cycle_loop.start_index;
        let remaining_cycles: usize = (TARGET_CYCLES - cycle_loop.end_index - 1) % cycle_loop_length;

        for _ in 0 .. remaining_cycles - 1 {
            roll_cycle(&mut grid);
        }

        let final_loads: Loads = roll_cycle(&mut grid);
        return Solution::Integer(final_loads.north);
    }
    
    panic!("No cycle found!");
}
