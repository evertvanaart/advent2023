use crate::solutions::Solution;
use crate::solutions::day09::common::*;

// Same as the A part, the only thing we need to change is the very last line
// of the recursion function; to get the preceding element, we simply return
// the first value of the current array _minus_ the result of the next step.
// There's probably a more efficient way to do this that doesn't allocate
// a new array every step, but sometimes it's nice to knock out a fairly
// performant solution for both parts in less than ten minutes.

fn recurse(values: &Vec<i64>) -> i64 {
    if values.iter().all(|v| *v == 0) {
        return 0
    }

    let next_values: Vec<i64> = (0 .. (values.len() - 1))
        .map(|i| values[i + 1] - values[i]).collect();

    values.first().unwrap() - recurse(&next_values)
}

fn solve_line(line: &str) -> i64 {
    let values: Vec<i64> = parse_values(line);
    recurse(&values)
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let result: i64 = lines.iter().map(|line| solve_line(line)).sum();
    return Solution::Integer(result)
}
