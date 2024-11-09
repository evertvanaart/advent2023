/* -------------------------------- Direction ------------------------------- */

pub enum Direction {
    North,
    East,
    South,
    West,
    Done
}

/* ---------------------------------- Step ---------------------------------- */

pub struct Step {
    pub coordinates: (isize, isize),
    pub from: Direction
}

/* ---------------------------------- Grid ---------------------------------- */

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

    pub fn find_start(&self) -> (isize, isize) {
        let index: usize = self.content.iter().enumerate().find(|(_, &c)| c == 'S').unwrap().0;
        self.to_coordinates(index as isize)
    }

    pub fn contains(&self, c: &(isize, isize)) -> bool {
        c.0 >= 0 && c.0 < self.rows && c.1 >= 0 && c.1 < self.cols
    }

    pub fn to_index(&self, c: &(isize, isize)) -> usize {
        (c.0 * self.cols + c.1) as usize
    }

    pub fn to_coordinates(&self, i: isize) -> (isize, isize) {
        (i / self.cols, i % self.cols)
    }
}

/* ------------------------------- Main logic ------------------------------- */

pub fn resolve_step(grid: &Grid, step: &Step) -> Option<Step> {
    let c: &(isize, isize) = &step.coordinates;
    let from: &Direction = &step.from;

    if !grid.contains(c) {
        return None
    }

    let ch = grid.content[(c.0 * grid.cols + c.1) as usize];

    match (ch, from) {
        ('|', Direction::North) => Some(Step { coordinates: (c.0 + 1, c.1), from: Direction::North }),
        ('|', Direction::South) => Some(Step { coordinates: (c.0 - 1, c.1), from: Direction::South }),
        ('-', Direction::West ) => Some(Step { coordinates: (c.0, c.1 + 1), from: Direction::West  }),
        ('-', Direction::East ) => Some(Step { coordinates: (c.0, c.1 - 1), from: Direction::East  }),
        ('L', Direction::North) => Some(Step { coordinates: (c.0, c.1 + 1), from: Direction::West  }),
        ('L', Direction::East ) => Some(Step { coordinates: (c.0 - 1, c.1), from: Direction::South }),
        ('J', Direction::North) => Some(Step { coordinates: (c.0, c.1 - 1), from: Direction::East  }),
        ('J', Direction::West ) => Some(Step { coordinates: (c.0 - 1, c.1), from: Direction::South }),
        ('7', Direction::South) => Some(Step { coordinates: (c.0, c.1 - 1), from: Direction::East  }),
        ('7', Direction::West ) => Some(Step { coordinates: (c.0 + 1, c.1), from: Direction::North }),
        ('F', Direction::South) => Some(Step { coordinates: (c.0, c.1 + 1), from: Direction::West  }),
        ('F', Direction::East ) => Some(Step { coordinates: (c.0 + 1, c.1), from: Direction::North }),
        ('S', _) => Some(Step { coordinates: (c.0, c.1), from: Direction::Done }),
        (  _, _) => None
    }
}
