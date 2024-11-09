use crate::solutions::Solution;
use crate::solutions::day11::common::*;

// Actually expanding the grid along the empty rows and columns would be very
// inefficient. Instead, we find all the stars, as well as the indices of the
// empty rows and columns. Then, we compute and sum up the pairwise distances
// between the stars. When computing the distance between two stars, we take the
// base Manhattan distance, and add to that the number of empty rows or columns
// that separate these stars. There's probably room for optimization here, like
// using a different data structure that allows us to count the empty rows or
// columns in a range more efficiently, but this is already fast enough.

fn count_empty(a: &usize, b: &usize, empty: &Vec<usize>) -> i64 {
    let min: usize = *a.min(b);
    let max: usize = *a.max(b);

    if max - min <= 1 {
        return 0;
    }

    empty.iter().filter(|&index| *index > min && *index < max).count() as i64
}

fn compute_distance(
    star_a: &(usize, usize),
    star_b: &(usize, usize),
    empty_rows: &Vec<usize>,
    empty_cols: &Vec<usize>) -> i64 {
    let nr_empty_rows: i64 = count_empty(&star_a.0, &star_b.0, &empty_rows);
    let nr_empty_cols: i64 = count_empty(&star_a.1, &star_b.1, &empty_cols);
    let base_distance: usize = star_a.0.abs_diff(star_b.0) + star_a.1.abs_diff(star_b.1);
    base_distance as i64 + nr_empty_rows + nr_empty_cols
}

fn count_distances(stars: Vec<(usize, usize)>, empty_rows: Vec<usize>, empty_cols: Vec<usize>) -> i64 {
    let mut sum: i64 = 0;

    for i in 0 .. stars.len() - 1 {
        for j in i .. stars.len() {
            let star_a: &(usize, usize) = &stars[i];
            let star_b: &(usize, usize) = &stars[j];
            sum += compute_distance(star_a, star_b, &empty_rows, &empty_cols);
        }
    }

    sum
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let stars: Vec<(usize, usize)> = find_stars(lines);
    let empty_rows: Vec<usize> = find_empty_rows(lines);
    let empty_cols: Vec<usize> = find_empty_cols(lines, &stars);
    let result: i64 = count_distances(stars, empty_rows, empty_cols);
    return Solution::Integer(result)
}
