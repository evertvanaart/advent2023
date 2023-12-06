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

fn solve_equation(t: f32, d: f32, sign: f32) -> f32 {
    (t + sign * (t.powi(2) - 4.0 * (d + 0.5)).sqrt()) / 2.0
}

fn count_winning_values(m: Match) -> i64 {
    let min: f32 = solve_equation(m.time as f32, m.distance as f32, -1.0);
    let max: f32 = solve_equation(m.time as f32, m.distance as f32,  1.0);
    max.ceil() as i64 - min.ceil() as i64
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let matches: Vec<Match> = parse_input(lines);
    let result: i64 = matches.into_iter().map(|m| count_winning_values(m)).product();
    return Solution::Integer(result)
}
