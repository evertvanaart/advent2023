use crate::solutions::Solution;
use crate::solutions::day09::common::*;

// A naive recursive solution, but it's fast enough. At every recursion step,
// first check if we've reached the end condition (all values in the current
// array are zero), and if so return zero. Otherwise, construct the next array
// using the differences between the values in the current array. The return
// value is the last value in the current array, plus the result of recursing
// on this next array. The depth of the recursion is limited by the number of
// values in each line, so we don't have to worry about stack overflows.

fn recurse(values: &Vec<i64>) -> i64 {
    if values.iter().all(|v| *v == 0) {
        return 0
    }

    let next_values: Vec<i64> = (0 .. (values.len() - 1))
        .map(|i| values[i + 1] - values[i]).collect();

    values.last().unwrap() + recurse(&next_values)
}

fn solve_line(line: &str) -> i64 {
    let values: Vec<i64> = parse_values(line);
    recurse(&values)
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let result: i64 = lines.iter().map(|line| solve_line(line)).sum();
    return Solution::Integer(result)
}
