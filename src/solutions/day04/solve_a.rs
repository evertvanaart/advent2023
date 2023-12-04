use std::cmp::Ordering;

use crate::solutions::Solution;

fn parse_numbers(substring: &str) -> Vec<i64> {
    substring.split(' ')
        .filter(|field| field.len() > 0)
        .map(|field| field.trim().parse().unwrap())
        .collect()
}

fn parse_line(line: &String) -> (Vec<i64>, Vec<i64>) {
    let without_prefix: &str = line.split_once(':').unwrap().1;
    let (winning_numbers, my_numbers) = without_prefix.split_once('|').unwrap();
    (parse_numbers(winning_numbers), parse_numbers(my_numbers))
}

fn process_line(line: &String) -> i64 {
    let (mut winning_numbers, mut my_numbers) = parse_line(line);

    winning_numbers.sort();
    my_numbers.sort();

    let mut winning_numbers_iter = winning_numbers.iter().peekable();
    let mut my_numbers_iter = my_numbers.iter().peekable();
    let mut matching: u32 = 0;

    while let (Some(&winning_number), Some(my_number)) = (winning_numbers_iter.peek(), my_numbers_iter.peek()) {
        match winning_number.cmp(my_number) {
            Ordering::Less    => { winning_numbers_iter.next(); }
            Ordering::Greater => { my_numbers_iter.next();      }
            Ordering::Equal   => {
                winning_numbers_iter.next();
                my_numbers_iter.next();
                matching = matching + 1;
            }
        }
    }

    if matching == 0 { 0} else { 2_i64.pow(matching - 1) }
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let result: i64 = lines.iter().map(|line| process_line(line)).sum();
    return Solution::Integer(result)
}
