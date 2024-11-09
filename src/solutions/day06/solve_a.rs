use crate::solutions::Solution;
use crate::solutions::day06::common::*;

// I don't usually go with mathematical solutions to solve these questions - my
// ready knowledge of math has largely faded away over the years - but I know a
// quadratic equation when I see one. Given the time T and our charging time C,
// the distance traveled D is computed as `D = C * (T - C)`, or `D = TC - C^2`.
// In order to find the range of `C` for which the distance is higher than the
// given threshold (plus 0.5 to deal with edge-cases), we just need to solve:
//
// -1 * C^2 + T * C - (D + 0.5) = 0
//
// With the standard quadratic equation solver, this resolves to:
//
// C = (T ± sqrt(T^2 - 4 * (D + 0.5))) / 2
//
// After computing both values - i.e., the two charge times that cap off the
// winning range - we simply round them up and compute the difference to get
// the number of winning integer values.

fn parse_values(line: &String) -> Vec<i64> {
    line.split_once(':')
        .unwrap().1.split(' ')
        .filter(|field| !field.is_empty())
        .map(|field| field.parse::<i64>().unwrap())
        .collect()
}

fn parse_input(lines: &Vec<String>) -> Vec<Match> {
    let time_values: Vec<i64> = parse_values(&lines[0]);
    let distance_values: Vec<i64> = parse_values(&lines[1]);

    time_values.into_iter().zip(distance_values)
        .map(|(time, distance)| Match { time: time, distance: distance }).collect()
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let matches: Vec<Match> = parse_input(lines);
    let result: i64 = matches.into_iter().map(|m| count_winning_values(m)).product();
    return Solution::Integer(result)
}
