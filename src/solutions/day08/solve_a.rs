use std::collections::HashMap;

use crate::solutions::Solution;

enum Direction {
    Left,
    Right
}

fn parse_directions(line: &str) -> Vec<Direction> {
    line.chars().map(|c| {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!()
        }
    }).collect()
}

fn parse_node_names(lines: &[String]) -> HashMap<String, usize> {
    lines.iter().enumerate()
        .map(|(index, line)| (String::from(&line[0..3]), index))
        .collect()
}

fn parse_node_links(lines: &[String], name_to_index: &HashMap<String, usize>) -> Vec<(usize, usize)> {
    lines.iter()
        .map(|line| (&line[7..10], &line[12..15]))
        .map(|(l, r)| (name_to_index[l], name_to_index[r]))
        .collect()
}

fn count_steps(node_links: &Vec<(usize, usize)>, directions: &Vec<Direction>, start_index: usize, end_index: usize) -> i64 {
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
