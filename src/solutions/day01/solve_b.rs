use std::collections::HashMap;

use crate::solutions::Solution;

// I initially figured I'd simply do a find-and-replace for textual numeric values
// ("one", "two", etc) to digits, and then repeat the A part. However, this doesn't
// work since the textual values can overlap, e.g. "twone" is a two when looking from
// the start, but a one when looking from the end. In the end, I opted to simply move
// an index along the string, and at each position check whether the current character
// is a digit, or whether the substring starting at this index starts with one of the
// textual values. There's room for optimization here, but it's still fast enough.

fn find_digit(line: &String, index: usize, word_to_value: &HashMap<&str, i64>) -> Option<i64> {
    if let Ok(parse_result) = line[index .. index + 1].parse::<i64>() {
        return Some(parse_result);
    }

    let substring: &str = &line[index..];

    for (&word, value) in word_to_value {
        if substring.starts_with(word) {
            return Some(*value);
        }
    }

    None
}

fn to_value(line: &String, word_to_value: &HashMap<&str, i64>) -> i64 {
    let first_digit: i64 = (0..line.len()).find_map(|i| find_digit(line, i, word_to_value)).unwrap();
    let last_digit: i64 = (0..line.len()).rev().find_map(|i| find_digit(line, i, word_to_value)).unwrap();
    return 10 * first_digit + last_digit;
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let word_to_value: HashMap<&str, i64> = HashMap::from([
        ("one",   1),
        ("two",   2),
        ("three", 3),
        ("four",  4),
        ("five",  5),
        ("six",   6),
        ("seven", 7),
        ("eight", 8),
        ("nine",  9),
    ]);

    let result: i64 = lines.iter().map(|line| to_value(line, &word_to_value)).sum();
    return Solution::Integer(result)
}
