use crate::solutions::Solution;
use crate::solutions::day04::common::*;

// _Obviously_ we aren't going to repeat the calculation to count the matching
// numbers for every single copy of every card; instead, we keep track of the total
// number of cards per game (original + copies) in a mutable vector, and increment
// the card count for the next X games by Y after each calculation, where X is the
// number of matches for the current game, and Y is the number of cards for the
// current game. In the end, all we have to do is sum up this count array.

fn process_line(line: &String, line_no: usize, counts: &mut Vec<usize>) {
    let matches: usize = count_matching_numbers(line) as usize;

    for offset in 1 ..= matches {
        counts[line_no + offset] += counts[line_no];
    }
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let mut counts: Vec<usize> = vec![1; lines.len()];
    lines.iter().enumerate()
        .for_each(|(line_no, line)| process_line(line, line_no, &mut counts));
    return Solution::Integer(counts.iter().sum::<usize>() as i64)
}
