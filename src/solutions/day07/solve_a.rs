use crate::solutions::Solution;
use crate::solutions::day07::common::*;

// This mostly comes down to parsing the input and determining what kind of
// hand we've got, and then sorting the list of hands in ascending order of
// power. Once we've got the sorted list, all we need to do is iterate over
// it, multiply the bid by the index in the sorted array (plus one, of
// course), and finally compute the sum of all hand values.
//
// Determining the hand is done by first counting the number of occurrences
// for each card in a fixed-length array (using the knowledge that each input
// character was mapped to an index in the range `0 ..= 12`), and then sorting
// this array in descending order. We can then quite easily determine the hand
// type by looking at the first two values, i.e. the sizes of the two largest
// groups of cards in our hand.
//
// Sorting is also straightforward. The `Hand` objects are compared first
// based on the previously determined hand type, and then by comparing the
// first two non-equal card values in each hand. The default Rust sorting
// algorithm takes less than half a millisecond to sort all 1000 hands in
// this way, so I wasn't tempted to try and improve on this.

fn char_to_value(c: char) -> usize {
    match c {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,

        _ => panic!("Invalid character '{c}'")
    }
}

fn count_cards(cards: &[usize]) -> Vec<usize> {
    let mut counts: [usize; 13] = [0; 13];
    cards.iter().for_each(|c| counts[*c] += 1);
    counts.sort_by(|a, b| b.cmp(a));
    counts.to_vec()
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let properties: HandProperties = HandProperties { char_to_value, count_cards };
    let mut rows: Vec<Row> = lines.iter().map(|line| Row::parse(line, &properties)).collect();
    rows.sort_by(|a, b| Row::compare(a, b));

    let result: i64 = rows.iter().enumerate()
        .map(|(index, row)| row.compute_value(index)).sum();

    return Solution::Integer(result)
}
