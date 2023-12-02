use crate::solutions::Solution;
use crate::solutions::day02::common::*;

// Since we already have the logic for parsing a line into a `Game` object,
// the B part is a matter of computing the power per game, which we can easily
// do by getting the maximum amount per color across all rounds and multiplying
// these maximum amounts. Again, we do slightly more work than strictly needed
// (in this B part, there's no reason to parse the game ID), but it's always
// nice to be able to reuse most of the code between parts.

fn compute_power(game: &Game) -> i64 {
    let max_red: i64   = game.rounds.iter().map(|round| round.red).max().unwrap() as i64;
    let max_green: i64 = game.rounds.iter().map(|round| round.green).max().unwrap() as i64;
    let max_blue: i64  = game.rounds.iter().map(|round| round.blue).max().unwrap() as i64;
    return max_red * max_green * max_blue;
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let result: i64 = lines.iter()
        .map(|line| parse_game(line))
        .map(|game| compute_power(&game))
        .sum();

    return Solution::Integer(result)
}
