use crate::solutions::Solution;
use crate::solutions::day10::common::*;

// While tracing the loop like we did in the A part, we mark fields to
// the _left_ of the potential loop. Assuming we finish the loop - i.e.,
// we end up back at the starting position - these marked fields are all
// either inside the loop, or all outside the loop. We grow this set of 
// initially marked fields by repeatedly marking unmarked neighbors that
// aren't part of the main pipe. If at any point during this growth phase
// we move out of the grid, we know that the marked fields were actually
// _outside_ of the loop, and we stop processing. As an additional opti-
// mization, we can stop tracing the loop as soon as we try to mark any
// field outside of the grid, since this also means that we've been
// marking fields outside of the loop.
//
// If we 1) manage to finish the loop, and 2) manage to grow the marked
// fields as much as possible without ever growing out of the grid, we've
// found the correct starting direction (i.e., all marked fields were on
// the _inside_), and return the total number of marked fields after the
// growth phase. In practice, we can stop tracing the pipe fairly quickly
// for three of the four possible starting directions, either because the
// pipe was interrupted or because we tried to mark outside of the grid.
// For the actual input, only one of the four starting directions makes
// it to the growth phase, making this solution fairly fast.
// 
// I didn't get the logic for marking the fields right on my first try;
// I initially marked only one field to the left of the current step
// instead of two (see the `offset` loop in `mark()`). Frustratingly,
// this initial solution still produced the correct answer for all
// sample inputs, but failed to mark a handful of fields for the
// real input. Introducing the `offset` loop fixed the issue.

struct Tracker {
    pipes: Vec<bool>,
    marked: Vec<bool>
}

impl Tracker {
    fn new(grid: &Grid) -> Tracker {
        let size: usize = (grid.rows * grid.cols) as usize;
        let pipes: Vec<bool> = vec![false; size];
        let marked: Vec<bool> = vec![false; size];
        Tracker { pipes: pipes, marked: marked }
    }

    fn mark(&mut self, grid: &Grid, step: &Step) -> bool {
        let pipe_index: usize = grid.to_index(&step.coordinates);
        self.marked[pipe_index] = false;
        self.pipes[pipe_index] = true;

        for offset in 0 ..= 1 {
            let mark_coordinates: (isize, isize) = match step.from {
                Direction::North => (step.coordinates.0 - offset, step.coordinates.1 + 1),
                Direction::East  => (step.coordinates.0 + 1, step.coordinates.1 + offset),
                Direction::South => (step.coordinates.0 + offset, step.coordinates.1 - 1),
                Direction::West  => (step.coordinates.0 - 1, step.coordinates.1 - offset),
                Direction::Done  => { return true; }
            };

            if !grid.contains(&mark_coordinates) {
                return false;
            }

            let mark_index: usize = grid.to_index(&mark_coordinates);

            if !self.pipes[mark_index] {
                self.marked[mark_index] = true;
            }
        }

        true
    }

    fn find_enclosed_size(&self, grid: &Grid) -> Option<i64> {
        let size: usize = (grid.rows * grid.cols) as usize;
        let mut processed: Vec<bool> = vec![false; size];

        let mut process_index: usize = 0;
        let mut enclosed_count: i64 = 0;
        let mut to_process: Vec<(isize, isize)> = self.marked.iter().enumerate()
            .filter(|(_, &m)| m).map(|(i, _)| grid.to_coordinates(i as isize)).collect();

        while process_index < to_process.len() {
            let current_coordinates: (isize, isize) = to_process[process_index];

            if !grid.contains(&current_coordinates) {
                return None
            }

            let current_index: usize = grid.to_index(&current_coordinates);

            if processed[current_index] || self.pipes[current_index] {
                process_index += 1;
                continue;
            }

            to_process.push((current_coordinates.0 - 1, current_coordinates.1));
            to_process.push((current_coordinates.0 + 1, current_coordinates.1));
            to_process.push((current_coordinates.0, current_coordinates.1 - 1));
            to_process.push((current_coordinates.0, current_coordinates.1 + 1));

            processed[current_index] = true;
            enclosed_count += 1;
            process_index += 1;
        }

        Some(enclosed_count)
    }
}

fn resolve(grid: &Grid, initial_step: Step) -> Option<Tracker> {
    let mut tracker: Tracker = Tracker::new(grid);
    let mut step: Step = initial_step;

    loop {
        let next_step_option: Option<Step> = resolve_step(grid, &step);

        match next_step_option {
            None => return None,
            Some(next_step) => match next_step.from {
                Direction::Done => {
                    tracker.mark(grid, &step);
                    return Some(tracker);
                },
                _ => {
                    let valid: bool = tracker.mark(grid, &step);

                    if !valid {
                        return None
                    }

                    step = next_step;
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
        .filter_map(|step| resolve(&grid, step))
        .find_map(|tracker| tracker.find_enclosed_size(&grid))
        .unwrap();
    
    return Solution::Integer(result)
}
