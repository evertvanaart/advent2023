const NEIGHBOR_OFFSETS: [(isize, isize); 8] = [(-1, -1), (-1,  0), (-1,  1),
                                               ( 0, -1),           ( 0,  1),
                                               ( 1, -1), ( 1,  0), ( 1,  1)];

pub struct Grid {
    pub content: Vec<char>,
    pub rows: isize,
    pub cols: isize
}

impl Grid {
    pub fn from_lines(lines: &Vec<String>) -> Grid {
        Grid {
            content: lines.concat().chars().collect(),
            rows: lines.len() as isize,
            cols: lines.first().unwrap().len() as isize
        }
    }

    pub fn get(&self, c: &(isize, isize)) -> &char {
        self.content.get((c.0 * self.cols + c.1) as usize).unwrap()
    }

    pub fn to_coordinates(&self, i: isize) -> (isize, isize) {
        (i / self.cols, i % self.cols)
    }

    pub fn to_index(&self, c: &(isize, isize)) -> usize {
        (c.0 * self.cols + c.1) as usize
    }

    pub fn contains(&self, c: &(isize, isize)) -> bool {
        c.0 >= 0 && c.0 < self.rows && c.1 >= 0 && c.1 < self.cols
    }

    pub fn get_neighbors(&self, c: &(isize, isize)) -> Vec<(isize, isize)> {
        NEIGHBOR_OFFSETS.iter()
            .map(|o| (c.0 + o.0, c.1 + o.1))
            .filter(|nc| self.contains(nc))
            .collect()
    }
}

pub fn find_number_start(grid: &Grid, c: &(isize, isize)) -> (isize, isize) {
    (0..).map(|o| (c.0, c.1 - o - 1))
        .find(|cx| !grid.contains(cx) || !grid.get(cx).is_digit(10))
        .map(|cx| (cx.0, cx.1 + 1))
        .unwrap()
}

pub fn parse_number(grid: &Grid, start: &(isize, isize)) -> i64 {
    let number_end: (isize, isize) = (1..).map(|o| (start.0, start.1 + o))
        .find(|cx| !grid.contains(cx) || !grid.get(cx).is_digit(10))
        .unwrap();

    let start_index: usize = grid.to_index(start);
    let end_index: usize = grid.to_index(&number_end);
    let numeric_string: String = grid.content[start_index .. end_index].iter().collect();
    numeric_string.parse().unwrap()
}
