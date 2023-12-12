use crate::solutions::Solution;
use crate::solutions::day12::common::*;

// A recursive approach. In each step, we find all possible positions in the
// remaining list of springs (after skipping any leading operational springs)
// where we could place the next group. This logic is somewhat fiddly, but
// essentially, we iterate over potential start positions (excluding those
// containing operational springs), and check if the next N springs (where
// N is length of the current group) are all either broken or unknown. We
// can stop checking for potential start positions for the current group if
// we've either passed a single broken spring - since the next group of broken
// springs cannot start _after_ the first broken spring - or if the number of
// remaining springs after a start position is less than the total number of
// remaining broken springs, i.e. the sum of the sizes of the groups that
// have not yet been placed. Additionally, the spring immediately after
// this potential group placement (if any) cannot be broken, since this
// would increase the size of the group.
//
// Once we've found all potential starting positions for the current group,
// we recurse on all of them, and return the sum of these recursions. The
// stopping condition is when we have no more groups to place, in which
// case we check whether the end state is valid, i.e. whether the list
// of remaining springs does not contain any known broken springs.

fn solve_line(line: &str) -> i64 {
    let fields: (&str, &str) = line.split_once(' ').unwrap();
    let springs: Vec<char> = parse_springs(fields.0);
    let groups: Vec<usize> = parse_groups(fields.1);

    let remaining: usize = groups.iter().sum::<usize>();
    let mut memo: Memo = Memo::new(springs.len(), groups.len());
    recurse(&springs, &groups, remaining, &mut memo)
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let result: i64 = lines.iter().map(|line| solve_line(line)).sum();
    return Solution::Integer(result)
}
