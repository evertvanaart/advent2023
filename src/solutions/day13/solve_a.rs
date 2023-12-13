use crate::solutions::Solution;

struct Grid {
    content: Vec<bool>,
    rows: isize,
    cols: isize
}

impl Grid {
    pub fn from_lines(lines: &[String]) -> Grid {
        Grid {
            content: lines.concat().chars().map(|c| c == '#').collect(),
            rows: lines.len() as isize,
            cols: lines.first().unwrap().len() as isize
        }
    }
}

fn solve_block(lines: &[String]) -> i64 {
    let grid: Grid = Grid::from_lines(lines);
    let h_candidates: Vec<(isize, isize)> = (0 .. grid.cols - 1).map(|col| (col, col + 1)).collect();
    
    let v_candidates: Vec<(isize, isize)> = (0 .. grid.rows - 1).map(|row| (row, row + 1)).collect();
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let blocks: Vec<&[String]> = lines.split(|line| line.is_empty()).collect();
    let result: i64 = blocks.into_iter().map(|block| solve_block(block)).sum();
    return Solution::Integer(result)
}
