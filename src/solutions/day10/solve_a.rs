use crate::solutions::Solution;
use crate::solutions::day10::common::*;

// From the starting position, try following the pipe in each of the four
// directions, counting the number of steps along the way. If we can trace
// the pipe all the way back to the starting position without encountering
// a stopping condition (e.g. moving out of the grid, or two subsequent
// fields not being connected properly), we return the total step count
// divided by two. In each step, we keep track of the field coordinates
// and the cardinal direction we came from, allowing us to determine the
// next field (if any) using the big `match` statement in `resolve_step()`.

fn resolve(grid: &Grid, initial_step: Step) -> Option<i64> {
    let mut step: Step = initial_step;
    let mut step_count: i64 = 0;

    loop {
        let next_step_option: Option<Step> = resolve_step(grid, &step);

        match next_step_option {
            None => return None,
            Some(next_step) => match next_step.from {
                Direction::Done => {
                    step_count += 1;
                    return Some(step_count / 2);
                },
                _ => {
                    step = next_step;
                    step_count += 1;
                }
            }
        }
    }
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let grid: Grid = Grid::from_lines(lines);
    let start_pos: (isize, isize) = grid.find_start();

    let initial_steps: Vec<Step> = vec!(
        Step { coordinates: (start_pos.0 + 1, start_pos.1), from: Direction::North },
        Step { coordinates: (start_pos.0 - 1, start_pos.1), from: Direction::South },
        Step { coordinates: (start_pos.0, start_pos.1 + 1), from: Direction::West  },
        Step { coordinates: (start_pos.0, start_pos.1 - 1), from: Direction::East  },
    );

    let result: i64 = initial_steps.into_iter()
        .find_map(|step| resolve(&grid, step)).unwrap();

    return Solution::Integer(result)
}
