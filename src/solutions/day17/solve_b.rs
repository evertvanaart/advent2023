use crate::solutions::Solution;
use crate::solutions::day17::common::*;

use std::cmp::min;

// Quite similar in complexity and runtime. I added a function to judge if
// the crucible can turn - which I retroactively also added to the A part to
// keep implementations similar - that returns true if the crucible has moved
// four or more steps since its last turn. The size of the 'sslt' dimension
// grew from three to ten, which explains why this part is slower than the
// A part, though luckily by a constant factor. The only other real change
// is that we have to discard the first three results - for 'sslt' of one,
// two, or three - when getting the best value, since the crucible cannot
// stop at the end cell if it has recently turned.

/* ---------------------------------- Step ---------------------------------- */

impl Step {
    fn can_move_forward_b(&self) -> bool {
        self.sslt < 10
    }

    fn can_turn_b(&self) -> bool {
        self.sslt > 3
    }
}

/* ---------------------------------- Memo ---------------------------------- */

impl Memo {
    fn get_best_b(&self) -> usize {
        min(
            // the first three values are not valid, since the crucible cannot
            // stop right after a turn; ignore them and take the minimum of the
            // remaining best results for east and south directions
            *self.best_east.iter().skip(3).min().unwrap(),
            *self.best_south.iter().skip(3).min().unwrap()
        )
    }
}

/* ---------------------------------- Main ---------------------------------- */

fn get_next_steps(grid: &mut Grid, steps: &Vec<Step>) -> Vec<Step> {
    let mut next_steps: Vec<Step> = Vec::new();

    for step in steps {
        if !grid.contains(&step.pos) {
            continue;
        }

        let value: usize = grid.value(&step.pos);
        let new_loss: usize = step.loss + value;

        if !grid.update(step, new_loss) {
            continue;
        }

        if step.can_move_forward_b() {
            let forward_step: Step = step.get_forward_step(new_loss);
            next_steps.push(forward_step);
        }

        if step.can_turn_b() {
            let turn_steps: (Step, Step) = step.get_turn_steps(new_loss);
            next_steps.push(turn_steps.0);
            next_steps.push(turn_steps.1);
        }
    }

    next_steps
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let mut grid: Grid = Grid::parse(lines, 10);
    let mut steps: Vec<Step> = Step::initial();

    while steps.len() > 0 {
        steps = get_next_steps(&mut grid, &steps);
    }

    let best_memo: &Memo = grid.get_best_memo();
    let result: usize = best_memo.get_best_b();
    return Solution::Integer(result as i64)
}
