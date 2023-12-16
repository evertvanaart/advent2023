use crate::solutions::Solution;
use crate::solutions::day16::common::*;

// I know there is a nicer way to do this - one which involves memoization,
// allowing us to quickly return the set of tiles that the current beam will
// visit based on its current tile and direction - but I wasn't able to get
// the implementation right. Despite it returning the correct answer for the
// sample, this solution returned a wrong answer for the real input. Having
// already spent a lot of time on this, and not feeling like spending even
// more time chasing down the bug, I opted for a naive solution instead.
//
// In this naive solution, we basically just repeat the A part for every
// possible start position, and then return the maximum of all these runs.
// Since the solution of the A part mutates the grid, we need to reset it
// after every run. Despite the lack of memoization - which would allow us
// to skip a lot of repeated calculations - this solution is still fairly
// fast (less than 100 milliseconds), so I guess I can live with it.

pub fn solve_one(grid: &mut Grid, init_beam: Beam) -> i64 {
    let mut beams: Vec<Beam> = vec!(init_beam);
    let mut index: usize = 0;

    grid.reset();

    while index < beams.len() {
        if let Some(new_beams) = trace(grid, &mut beams, index) {
            new_beams.into_iter().for_each(|b| beams.push(b));
        }

        index += 1;
    }

    grid.energy as i64
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let mut grid: Grid = Grid::parse(lines);

    let mut init_beams: Vec<Beam> = Vec::new();
    (0 .. grid.cols).for_each(|col| init_beams.push(Beam::new((-1, col), Direction::South)));
    (0 .. grid.cols).for_each(|col| init_beams.push(Beam::new((grid.rows, col), Direction::North)));
    (0 .. grid.rows).for_each(|row| init_beams.push(Beam::new((row, grid.cols), Direction::West)));
    (0 .. grid.rows).for_each(|row| init_beams.push(Beam::new((row, -1), Direction::East)));

    let result: i64 = init_beams.into_iter().map(|init_beam| solve_one(&mut grid, init_beam)).max().unwrap();

    return Solution::Integer(result)
}
