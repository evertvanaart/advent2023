use crate::solutions::Solution;
use crate::solutions::day02::common::*;

// This one mostly just comes down to parsing the input strings. I took a couple
// of shortcuts, like assuming that the game ID always starts at index 5 (although
// strictly speaking, we don't even need to parse the game ID, since it's always
// equal to the line number), but otherwise the parsing logic should be pretty
// robust. After parsing a line to a `Game` object, we can check if it's valid
// simply by checking that none of the rounds had any color amounts higher than
// the stated limits. It would've been more efficient to check this during parsing
// (and stop parsing a game as soon as we find an invalid round), but parsing the
// full line into a game is needed for the B part anyway.

fn is_valid(game: &Game) -> bool {
    return game.rounds.iter().all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14);
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let result: i64 = lines.iter()
        .map(|line| parse_game(line))
        .filter(|game| is_valid(game))
        .map(|game| game.id)
        .sum();

    return Solution::Integer(result)
}
