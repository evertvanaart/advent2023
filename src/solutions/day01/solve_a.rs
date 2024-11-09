use crate::solutions::Solution;

// Very straightforward, simply iterate over the characters in both directions
// and find the first one that can be converted to a digit. The `find_map()`
// function is very useful here, though I didn't test whether it's faster
// than first checking with `is_digit()` and converting the first match.

fn to_value(line: &String) -> i64 {
    let first_digit: i64 = line.chars().find_map(|c| c.to_digit(10)).unwrap() as i64;
    let last_digit: i64 = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap() as i64;
    return 10 * first_digit + last_digit;
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let result: i64 = lines.iter().map(to_value).sum();
    return Solution::Integer(result)
}
