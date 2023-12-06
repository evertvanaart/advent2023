use crate::solutions::Solution;
use crate::solutions::day06::common::*;

// Exactly as difficult as the first part, unless you somehow didn't use the
// quadratic equation solver. The only thing I had to fix was change the type
// used in the calculation from `f32` to `f64`, since the high numbers in the
// B part caused rounding errors when using `f32`.

fn parse_value(line: &String) -> i64 {
    line.split_once(':').unwrap().1
        .replace(' ', "").parse().unwrap()
    }

fn parse_input(lines: &Vec<String>) -> Match {
    let time: i64 = parse_value(&lines[0]);
    let distance: i64 = parse_value(&lines[1]);
    Match { time, distance }
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let m: Match = parse_input(lines);
    let result: i64 = count_winning_values(m);
    return Solution::Integer(result)
}
