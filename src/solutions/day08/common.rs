use std::collections::HashMap;

pub enum Direction {
    Left,
    Right
}

pub fn parse_directions(line: &str) -> Vec<Direction> {
    line.chars().map(|c| {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!()
        }
    }).collect()
}

pub fn parse_node_names(lines: &[String]) -> HashMap<String, usize> {
    lines.iter().enumerate()
        .map(|(index, line)| (String::from(&line[0..3]), index))
        .collect()
}

pub fn parse_node_links(lines: &[String], name_to_index: &HashMap<String, usize>) -> Vec<(usize, usize)> {
    lines.iter()
        .map(|line| (&line[7..10], &line[12..15]))
        .map(|(l, r)| (name_to_index[l], name_to_index[r]))
        .collect()
}
