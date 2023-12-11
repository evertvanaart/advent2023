pub fn find_stars(lines: &Vec<String>) -> Vec<(usize, usize)> {
    lines.iter().enumerate().map(move |(row, line)| {
        line.chars().enumerate()
            .filter(|(_, ch)| *ch == '#')
            .map(move |(col, _)| (row, col))
    }).flatten().collect()
}

pub fn find_empty_rows(lines: &Vec<String>) -> Vec<usize> {
    lines.iter().enumerate()
        .filter(|(_, line)| !line.contains('#'))
        .map(|(row, _)| row)
        .collect()
}

pub fn find_empty_cols(lines: &Vec<String>, stars: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut is_empty: Vec<bool> = vec![true; lines[0].len()];
    stars.iter().for_each(|(_, col)| is_empty[*col] = false);

    is_empty.into_iter().enumerate()
        .filter(|(_, empty)| *empty)
        .map(|(col, _)| col)
        .collect()
}
