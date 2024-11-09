use std::collections::HashMap;

use crate::solutions::Solution;
use crate::solutions::day08::common::*;

// First create a map of node names to indices, where the index is simply
// the node's position (zero-based) in the original input. Using this list,
// create a list of node links, i.e. for each node create a pair of indices
// indicating the next node index when moving left or right, respectively.
// Once we've got this list of links, and we've found the start and end
// nodes, we simply follow the directions from the start node until
// we've reached the end node, and return the number of steps taken.

fn count_steps(
    node_links: &Vec<(usize, usize)>,
    directions: &Vec<Direction>,
    start_index: usize,
    end_index: usize) -> i64 {
    let mut current_index: usize = start_index;
    let mut step_count: usize = 0;

    loop {
        current_index = match directions[step_count % directions.len()] {
            Direction::Left  => node_links[current_index].0,
            Direction::Right => node_links[current_index].1
        };

        step_count += 1;

        if current_index == end_index {
            return step_count as i64
        }
    }
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let directions: Vec<Direction> = parse_directions(&lines[0]);
    let name_to_index: HashMap<String, usize> = parse_node_names(&lines[2..]);
    let node_links: Vec<(usize, usize)> = parse_node_links(&lines[2..], &name_to_index);
    let start_index: usize = name_to_index["AAA"];
    let end_index: usize = name_to_index["ZZZ"];
    
    let result: i64 = count_steps(&node_links, &directions, start_index, end_index);
    
    return Solution::Integer(result)
}
