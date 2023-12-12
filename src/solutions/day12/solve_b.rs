use crate::solutions::Solution;
use crate::solutions::day12::common::*;

// The big optimization - applied retroactively to the A part as well - is to
// use memoization. Without this optimization, the A part is fairly fast, but
// the B part is extremely slow (potentially in the order of hours, I didn't
// let it finish). Adding memoization reduces this to just under 10 milli-
// seconds, which is the first runtime over a millisecond this year, but
// still not bad considering the complexity of the problem.
//
// Memoization essentially records the known result per remaining number of
// springs and remaining number of groups at every recursion step, and uses
// this recorded result if available to skip repeating the same recursions.
// As a minor optimization, we store the results in a 1D vector (as opposed
// to e.g. a hashmap) in order to slightly speed up access at the cost of
// increased memory usage.

fn extend_string(input: &str, separator: &str) -> String {
    format!("{input}{separator}{input}{separator}{input}{separator}{input}{separator}{input}")
}

fn solve_line(line: &str) -> i64 {
    let fields: (&str, &str) = line.split_once(' ').unwrap();
    let springs: Vec<char> = parse_springs(&extend_string(fields.0, "?"));
    let groups: Vec<usize> = parse_groups(&extend_string(fields.1, ","));

    let remaining: usize = groups.iter().sum::<usize>();
    let mut memo: Memo = Memo::new(springs.len(), groups.len());
    let result = recurse(&springs, &groups, remaining, &mut memo);
    result
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let result: i64 = lines.iter().map(|line| solve_line(line)).sum();
    return Solution::Integer(result)
}
