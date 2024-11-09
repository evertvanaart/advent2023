use crate::solutions::Solution;
use crate::solutions::day14::common::*;

// For the A part, we simply simulate the round stones rolling north
// as described. We iterate over the grid from top to bottom - which
// is important, as its means that another round stone blocking the
// current one has already moved - and roll it north as long as we
// stay inside the grid and encounter only empty spaces. After we
// hit an obstacle, we move the stone to the new position in the
// grid, and return the load on the north column for that stone,
// which is simply its row index subtracted from the total number
// of rows. Summing this load over all stones gives us the answer.

fn roll_stone_north(grid: &mut Grid, row: isize, col: isize) -> i64 {
    let mut c: (isize, isize) = (row, col);
    let field: &Field = grid.get(&c);

    match field {
        Field::Empty      => { return 0; }
        Field::SquareRock => { return 0; }
        Field::RoundRock  => { }
    }

    let c_start: (isize, isize) = (c.0, c.1);

    loop {
        if c.0 == 0 {
            break;
        }

        let nc: (isize, isize) = (c.0 - 1, c.1);
        let nfield: &Field = grid.get(&nc);

        match nfield {
            Field::RoundRock  => { break; }
            Field::SquareRock => { break; }
            Field::Empty      => { c.0 -= 1; }
        }
    }

    if c != c_start {
        grid.set(&c_start, Field::Empty);
        grid.set(&c, Field::RoundRock);
    }

    return (grid.rows - c.0) as i64;
}

fn roll_row_north(grid: &mut Grid, row: isize) -> i64 {
    (0 .. grid.cols).map(|col| roll_stone_north(grid, row, col)).sum()
}

fn roll_all_north(grid: &mut Grid) -> i64 {
    (0 .. grid.rows).map(|row| roll_row_north(grid, row)).sum()
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let mut grid: Grid = Grid::from_lines(lines);
    let result: i64 = roll_all_north(&mut grid);
    return Solution::Integer(result);
}
