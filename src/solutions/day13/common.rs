/* -------------------------------- Direction ------------------------------- */

pub enum Direction {
    Horizontal,
    Vertical
}

impl Direction {
    pub fn other(&self) -> Direction {
        match self {
            Direction::Horizontal => Direction::Vertical,
            Direction::Vertical   => Direction::Horizontal,
        }
    }
}

/* ---------------------------------- Grid ---------------------------------- */

pub struct Grid {
    pub content: Vec<bool>,
    pub rows: isize,
    pub cols: isize
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

    pub fn contains(&self, c: &(isize, isize)) -> bool {
        c.0 >= 0 && c.0 < self.rows && c.1 >= 0 && c.1 < self.cols
    }

    pub fn index_range(&self, direction: &Direction) -> std::ops::Range<isize> {
        match direction {
            Direction::Horizontal => 0 .. self.rows,
            Direction::Vertical   => 0 .. self.cols,
        }
    }
}
