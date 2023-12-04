use crate::solutions::Solution;
use crate::solutions::day04::common::*;

// The obvious solution is to dump both lists of numbers into their own set
// and then count the size of the intersection of those two sets, but that's
// _boring_. Instead, I opted to calculate the number of matches using an
// old-fashioned double iterator over sorted lists; if nothing else, this
// turned out to be a good excuse to practice some Rust features such as
// `while let`. I did end up writing a quick set-based solution as well,
// but it was actually slightly slower than the current solution.

fn process_line(line: &String) -> i64 {
    let matches: u32 = count_matching_numbers(line);
    if matches == 0 { 0 } else { 2_i64.pow(matches - 1) }
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let result: i64 = lines.iter().map(|line| process_line(line)).sum();
    return Solution::Integer(result)
}
