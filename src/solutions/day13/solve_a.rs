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
            cols: lines.first().unwrap().len() as isize,
            rows: lines.len() as isize
        }
    }

    pub fn get(&self, c: &(isize, isize)) -> bool {
        self.content[(c.0 * self.cols + c.1) as usize]
    }

    fn contains(&self, c: &(isize, isize)) -> bool {
        c.0 >= 0 && c.0 < self.rows && c.1 >= 0 && c.1 < self.cols
    }
}

fn is_symmetric(grid: &Grid, pivot: (isize, isize), horizontal: bool) -> bool {
    for offset in 0 .. grid.cols {
        let coordinates_a: (isize, isize) = if horizontal {
            (pivot.0, pivot.1 - offset)
        } else {
            (pivot.0 - offset, pivot.1)
        };

        if !grid.contains(&coordinates_a) {
            return true;
        }

        let coordinates_b: (isize, isize) = if horizontal {
            (pivot.0, pivot.1 + offset + 1)
        } else {
            (pivot.0 + offset + 1, pivot.1)
        };
    
        if !grid.contains(&coordinates_b) {
            return true;
        }

        let value_a = grid.get(&coordinates_a);
        let value_b = grid.get(&coordinates_b);

        if value_a != value_b {
            return false;
        }
    }

    return true;
}

fn filter_candidates(grid: &Grid, index: isize, candidates: &Vec<isize>, horizontal: bool) -> Vec<isize> {
    candidates.into_iter().filter(|&candidate| {
        let pivot: (isize, isize) = if horizontal {
            (index, *candidate)
        } else {
            (*candidate, index)
        };

        return is_symmetric(grid, pivot, horizontal);
    }).map(|c| *c).collect()
}

fn check(grid: &Grid, horizontal: bool) -> Option<isize> {
    let mut candidates: Vec<isize> = if horizontal {
        (0 .. grid.cols - 1).collect()
    } else {
        (0 .. grid.rows - 1).collect()
    };

    let index_range: std::ops::Range<isize> = if horizontal {
        0 .. grid.rows
    } else {
        0 .. grid.cols
    };

    for index in index_range {
        candidates = filter_candidates(grid, index, &candidates, horizontal);
        
        if candidates.is_empty() {
            return None;
        }
    }

    if candidates.len() == 1 {
        return Some(candidates[0])
    } else {
        return None
    }
}

fn solve_block(lines: &[String]) -> i64 {
    let grid: Grid = Grid::from_lines(lines);
    
    if let Some(col) = check(&grid, true) {
        return (col + 1) as i64;
    } else if let Some(row) = check(&grid, false) {
        return (row + 1) as i64 * 100;
    }
    
    panic!("No symmetry found");
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let blocks: Vec<&[String]> = lines.split(|line| line.is_empty()).collect();
    let result: i64 = blocks.into_iter().map(|block| solve_block(block)).sum();
    return Solution::Integer(result)
}
