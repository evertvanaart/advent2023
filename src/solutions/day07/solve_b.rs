use crate::solutions::Solution;
use crate::solutions::day07::common::*;

// Compared to the A part, there are only two things we really need to change
// (which I've exposed somewhat awkwardly through the `HandProperties` struct):
// the values of the cards ('J' now being the lowest, and all thirteen still
// mapping to the range `0 ..= 12`), and the way in which we determine the
// hand type. This second part is actually rather straightforward, since in
// order to get the best possible hand, the group of jokers always needs to
// be added to the largest non-joker group. This means that all we need to
// do is sort the array of card counts _without_ the joker count (which is
// conveniently stored at the very start, since 'J' maps to index 0), and
// then add this joker count to the highest value in this sorted array.

fn char_to_value(c: char) -> usize {
    match c {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,

        _ => panic!("Invalid character '{c}'")
    }
}

fn count_cards(cards: &[usize]) -> Vec<usize> {
    let mut counts: [usize; 13] = [0; 13];
    cards.iter().for_each(|c| counts[*c] += 1);

    let joker_count: usize = counts[0];
    let mut remaining_counts: Vec<usize> = counts[1..].to_vec();
    remaining_counts.sort_by(|a, b| b.cmp(a));
    remaining_counts[0] += joker_count;
    remaining_counts
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let properties: HandProperties = HandProperties { char_to_value, count_cards };
    let mut rows: Vec<Row> = lines.iter().map(|line| Row::parse(line, &properties)).collect();
    rows.sort_by(|a, b| Row::compare(a, b));

    let result: i64 = rows.iter().enumerate()
        .map(|(index, row)| row.compute_value(index)).sum();

    return Solution::Integer(result)
}
