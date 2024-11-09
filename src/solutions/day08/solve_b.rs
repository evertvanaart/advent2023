use std::collections::HashMap;

use crate::solutions::Solution;
use crate::solutions::day08::common::*;

// This one was tough. I tried the naive approach first - simply trace all paths
// starting at the start nodes step by step, and stop if all paths are at an end
// node at the same time - but this is extremely slow, considering the fact that
// we need more than 10 trillion steps.
//
// Instead, we look for loops. If we trace a path from any start node, it will
// inevitable start to loop; more specifically, it will enter a loop after `O`
// steps, and will loop back after every addition `P` steps; both `O` and `P`
// will be multiples of the number of directions `D`. We search for these loops
// in the `find_loops()` function by checking every `D` steps if the current node
// was already visited at an earlier multiple of `D` steps. In addition, we track
// at which step counts along the way we encountered end nodes (every loop should
// contain at least one end node). Once we've found the loop, we record the offset
// of each encountered end node, as well as the period of the full loop.
//
// (In the actual input, each loop contains exactly one end node, meaning we
// could theoretically change the `offsets` field of `Loop` to a single value.
// However, this just happens to be the case for this particular input; the
// sample of the B part, for example, encounters the same end node twice in
// one loop, resulting in two offset values.)
//
// Once we've found the loop for each of the start nodes, we need to find the
// lowest value common to all these loops. There is probably a smart mathemical
// way to do this, but I couldn't figure it out for the life of me. Instead, I
// start at one offset, and increase the step count by the period of this loop
// until I find a step count for which one of the other loops is also at an end
// node. At this point, I increase the step size to the LCM of both loop periods;
// this allows me to make larger steps, while ensuring that both loops will be
// at an end node after every step. I then repeat this for the other loops
// (growing the step size every time) until I've incorperated all of them,
// at which point I return the current step count.
//
// Note that this doesn't quite work for the sample; since the loop with offset
// 2 and period 2 never intersects with the one with offset 3 and period 6, we
// end up in an infinite loop. It does work for the actual input though (and
// pretty fast too; against all odds I managed to keep it under one milli-
// second), and I've already spent way too much time on this, so whatever.

#[derive(Clone)]
struct Loop {
    offsets: Vec<usize>,
    period: usize
}

impl Loop {
    fn is_end(&self, steps: usize) -> bool {
        self.offsets.iter().any(|o| steps >= *o && (steps - o) % self.period == 0)
    }
}

fn gcd(a: usize, b: usize) -> usize {
    let mut aa = a;
    let mut bb = b;

    while bb != 0 {
        let t: usize = bb;
        bb = aa % bb;
        aa = t;
    }

    return aa
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

pub fn find_nodes(name_to_index: &HashMap<String, usize>, last_char: char) -> Vec<usize> {
    name_to_index.iter()
        .filter(|(name, _)| name.ends_with(last_char))
        .map(|(_, index)| *index)
        .collect()
}

fn find_loops(
    node_links: Vec<(usize, usize)>,
    directions: Vec<Direction>,
    start_indices: Vec<usize>,
    end_indices: Vec<usize>) -> Vec<Loop> {
    start_indices.into_iter().map(|start_index| {
        let mut loop_start_indices: Vec<Option<usize>> = vec![None; node_links.len()];
        let mut end_nodes_steps: Vec<usize> = Vec::new();
        let mut current_index: usize = start_index;
        let mut direction_index: usize = 0;
        let mut step_count: usize = 0;

        loop {
            if direction_index == 0 {
                if let Some(previous_step_count) = loop_start_indices[current_index] {
                    let offsets: Vec<usize> = end_nodes_steps.into_iter()
                        .filter(|steps| *steps >= previous_step_count).collect();
                    return Loop { offsets: offsets, period: step_count - previous_step_count }
                }

                loop_start_indices[current_index] = Some(step_count);
            }

            if end_indices.contains(&current_index) {
                end_nodes_steps.push(step_count);
            }

            current_index = match directions[direction_index] {
                Direction::Left  => node_links[current_index].0,
                Direction::Right => node_links[current_index].1
            };

            direction_index += 1;
            step_count += 1;

            if direction_index == directions.len() {
                direction_index = 0;
            }
        }
    }).collect()
}

fn find_min_steps(loops: Vec<Loop>) -> i64 {
    let first_loop: &Loop = loops.first().unwrap();
    
    first_loop.offsets.iter().map(|offset| {
        let mut other_loops: Vec<Loop> = loops[1..].to_vec();
        let mut period: usize = first_loop.period;
        let mut steps: usize = *offset;

        loop {
            if let Some((index, other_loop)) = other_loops.iter().enumerate()
                .find(|(_, other_loop)| other_loop.is_end(steps)) {
                period = lcm(period, other_loop.period);
                other_loops.remove(index);
                continue;
            }

            if other_loops.is_empty() {
                return steps;
            }

            steps += period;
        }
    }).min().unwrap() as i64
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let directions: Vec<Direction> = parse_directions(&lines[0]);
    let name_to_index: HashMap<String, usize> = parse_node_names(&lines[2..]);
    let node_links: Vec<(usize, usize)> = parse_node_links(&lines[2..], &name_to_index);

    let start_indices: Vec<usize> = find_nodes(&name_to_index, 'A');
    let end_indices: Vec<usize> = find_nodes(&name_to_index, 'Z');

    let loops: Vec<Loop> = find_loops(node_links, directions, start_indices, end_indices);
    let result: i64 = find_min_steps(loops);

    return Solution::Integer(result)
}
