/* --------------------------------- Parsing -------------------------------- */

pub fn parse_springs(input: &str) -> Vec<char> {
    input.chars().collect()
}

pub fn parse_groups(input: &str) -> Vec<usize> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

/* ------------------------------- Memoization ------------------------------ */

pub struct Memo {
    results: Vec<i64>,
    nr_groups: usize
}

impl Memo {
    pub fn new(nr_springs: usize, nr_groups: usize) -> Memo {
        Memo { results: vec![-1; (nr_springs + 1) * (nr_groups + 1)], nr_groups: nr_groups }
    }

    fn get(&self, springs_len: usize, groups_len: usize) -> Option<i64> {
        let result: i64 = self.results[springs_len * self.nr_groups + groups_len];
        if result == -1 { None } else { Some(result) }
    }

    fn set(&mut self, springs_len: usize, groups_len: usize, result: i64) {
        self.results[springs_len * self.nr_groups + groups_len] = result;
    }
}

/* --------------------------------- Helpers -------------------------------- */

fn check_group_fits(springs: &[char], size: usize) -> bool {
    let fits: bool = springs[0 .. size].iter().all(|c| *c != '.');
    fits && (springs.len() == size || springs[size] != '#')
}

fn skip_operational(springs: &[char]) -> usize {
    springs.iter().enumerate()
        .find(|(_, &c)| c != '.').map(|(i, _)| i)
        .unwrap_or(springs.len())
}

fn find_group_positions(springs: &[char], size: usize, remaining: usize) -> Vec<usize> {
    if springs.len() < remaining {
        return Vec::new()
    }

    let mut positions: Vec<usize> = Vec::new();

    for start_index in 0 ..= (springs.len() - remaining) {
        let current_spring: char = springs[start_index];

        if current_spring == '.' {
            continue;
        }

        if check_group_fits(&springs[start_index..], size) {
            positions.push(start_index);
        }

        if current_spring == '#' {
            break;
        }
    }

    positions
}

/* -------------------------------- Recursion ------------------------------- */

pub fn recurse(springs: &[char], groups: &[usize], remaining: usize, memo: &mut Memo) -> i64 {
    if groups.is_empty() {
        let valid: bool = springs.iter().all(|c| *c != '#');
        return if valid { 1 } else { 0 };
    }

    let from: usize = skip_operational(springs);

    if from == springs.len() {
        return 0;
    }

    let trimmed_springs: &[char] = &springs[from..];

    if let Some(result) = memo.get(trimmed_springs.len(), groups.len()) {
        return result;
    }

    let next_group: usize = groups[0];
    let next_remaining: usize = remaining - next_group;
    let remaining_with_spaces = remaining + groups.len() - 1;

    let positions: Vec<usize> = find_group_positions(trimmed_springs, next_group, remaining_with_spaces);

    let result: i64 = positions.iter().map(|pos| {
        let start: usize = (pos + next_group + 1).min(trimmed_springs.len());
        recurse(&trimmed_springs[start..], &groups[1..], next_remaining, memo)
    }).sum();

    memo.set(trimmed_springs.len(), groups.len(), result);
    result
}
