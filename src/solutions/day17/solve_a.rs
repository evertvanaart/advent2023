use crate::solutions::Solution;
use crate::solutions::day17::common::*;

use std::cmp::min;

// This one took a while - in fact, I didn't solve this until April 2024 - and
// I still had to settle for a relatively slow solution that takes more than a
// second to run. The tough part compared to regular path optimization problems
// is that you cannot just memoize only the best result at every cell after every
// step, you also have to take into account the shape of the path you took to get
// to that cell. Specifically, the number of steps since the last turn (which I
// call 'sslt' in the code) adds another dimension to the grid, which has to be
// taken into account when judging whether the current result at a specific cell
// is better than previous results. This means that for the A part, we memoize
// twelve best results per cell during a breath-first search; four directions,
// and three per direction for each possible value of 'sslt'. Whenever we take
// a step, we update both the direction and the steps since the last turn, and
// if the memoized value for this direction and 'sslt' is less than the current
// loss value, we end the path prematurely. This ensures that the maximum number
// of active paths after each step is limited to (#rows * #columns * 12).
//
// Intuitively, I think this memoization logic could be made more strict as an
// optimization strategy. Currently, we judge wether a path should be stopped by
// looking only at the best result for the same direction and 'sslt' value. It
// should be possible to extend this; for example, a path with (dir = south,
// sslt = 2, loss = 10) is objectively worse than a path with (dir = south,
// sslt = 1, loss = 10) at the same cell; despite the same loss value, the
// possible continuations of the first path are a subset of those of the
// second path due to the lower 'sslt' value, so there is no reason to
// continue propagating the first path. Unfortunately, I wasn't able to 
// implement this correctly, so I had to settle for the slower approach.

/* ---------------------------------- Step ---------------------------------- */

impl Step {
    fn can_move_forward_a(&self) -> bool {
        self.sslt < 3
    }

    fn can_turn_a(&self) -> bool { true }
}

/* ---------------------------------- Memo ---------------------------------- */

impl Memo {
    fn get_best_a(&self) -> usize {
        min(
            *self.best_east.iter().min().unwrap(),
            *self.best_south.iter().min().unwrap()
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

        if step.can_move_forward_a() {
            let forward_step: Step = step.get_forward_step(new_loss);
            next_steps.push(forward_step);
        }

        if step.can_turn_a() {
            let turn_steps: (Step, Step) = step.get_turn_steps(new_loss);
            next_steps.push(turn_steps.0);
            next_steps.push(turn_steps.1);
        }
    }

    next_steps
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let mut grid: Grid = Grid::parse(lines, 3);
    let mut steps: Vec<Step> = Step::initial();

    while steps.len() > 0 {
        steps = get_next_steps(&mut grid, &steps);
    }

    let best_memo: &Memo = grid.get_best_memo();
    let result: usize = best_memo.get_best_a();
    return Solution::Integer(result as i64)
}
