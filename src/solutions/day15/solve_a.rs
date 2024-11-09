use crate::solutions::Solution;
use crate::solutions::day15::common::*;

// A surprisingly easy A part, considering we're past the halfway
// point. Nothing to say about this, just followed the instructions.


pub fn solve(lines: &Vec<String>) -> Solution {
    let result: usize = lines[0].split(',')
        .map(|field| compute_hash(field)).sum();

    return Solution::Integer(result as i64)
}
