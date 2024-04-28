/* -------------------------------- Direction ------------------------------- */

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    East,
    South,
    West,
    North
}

impl Direction {
    pub fn left(&self) -> Direction {
        match self {
            Direction::East  => Direction::North,
            Direction::South => Direction::East,
            Direction::West  => Direction::South,
            Direction::North => Direction::West
        }
    }

    pub fn right(&self) -> Direction {
        match self {
            Direction::East  => Direction::South,
            Direction::South => Direction::West,
            Direction::West  => Direction::North,
            Direction::North => Direction::East
        }
    }

    pub fn step(&self, pos: &(isize, isize)) -> (isize, isize) {
        match self {
            Direction::East  => (pos.0, pos.1 + 1),
            Direction::South => (pos.0 + 1, pos.1),
            Direction::West  => (pos.0, pos.1 - 1),
            Direction::North => (pos.0 - 1, pos.1)
        }
    }
}

/* ---------------------------------- Step ---------------------------------- */

pub struct Step {
    pub pos: (isize, isize),
    pub dir: Direction,
    pub sslt: usize,
    pub loss: usize
}

impl Step {
    pub fn initial() -> Vec<Step> {
        vec![
            Step { pos: (0, 1), dir: Direction::East,  sslt: 1,  loss: 0 },
            Step { pos: (1, 0), dir: Direction::South, sslt: 1,  loss: 0 }
        ]
    }

    pub fn get_forward_step(&self, loss: usize) -> Step {
        Step { pos: self.dir.step(&self.pos), dir: self.dir, sslt: self.sslt + 1, loss: loss }
    }

    pub fn get_turn_steps(&self, loss: usize) -> (Step, Step) {
        let left_dir: Direction = self.dir.left();
        let right_dir: Direction = self.dir.right();

        (
            Step { pos: left_dir.step(&self.pos),  dir: left_dir,  sslt: 1, loss: loss },
            Step { pos: right_dir.step(&self.pos), dir: right_dir, sslt: 1, loss: loss }
        )
    }
}

/* ---------------------------------- Memo ---------------------------------- */

pub struct Memo {
    pub best_east:  Vec<usize>,
    pub best_south: Vec<usize>,
    pub best_west:  Vec<usize>,
    pub best_north: Vec<usize>
}

impl Memo {
    fn new(length: usize) -> Memo {
        Memo {
            best_east:  vec![usize::MAX; length],
            best_south: vec![usize::MAX; length],
            best_west:  vec![usize::MAX; length],
            best_north: vec![usize::MAX; length]
        }
    }

    fn update(&mut self, step: &Step, loss: usize) -> bool {
        let best: &mut Vec<usize> = match step.dir {
            Direction::East  => &mut self.best_east,
            Direction::South => &mut self.best_south,
            Direction::West  => &mut self.best_west,
            Direction::North => &mut self.best_north
        };

        let index: usize = step.sslt - 1;
        let better: bool = loss < best[index];

        if better {
            best[index] = loss;
        }

        better
    }
}

/* ---------------------------------- Tile ---------------------------------- */

pub struct Tile {
    pub value: usize,
    pub memo: Memo
}

impl Tile {
    fn parse(c: char, memo_length: usize) -> Tile {
        let value: usize = c.to_string().parse().unwrap();
        Tile { value: value, memo: Memo::new(memo_length) }
    }
}

/* ---------------------------------- Grid ---------------------------------- */

pub struct Grid {
    pub tiles: Vec<Tile>,
    pub rows: isize,
    pub cols: isize
}

impl Grid {
    pub fn parse(lines: &[String], memo_length: usize) -> Grid {
        let tiles: Vec<Tile> = lines.concat().chars()
            .map(|c| Tile::parse(c, memo_length)).collect();

        Grid {
            tiles: tiles,
            rows: lines.len() as isize,
            cols: lines[0].len() as isize
        }
    }

    pub fn value(&self, pos: &(isize, isize)) -> usize {
        self.tiles[self.to_index(pos)].value
    }

    pub fn contains(&self, pos: &(isize, isize)) -> bool {
        pos.0 >= 0 && pos.0 < self.rows && pos.1 >= 0 && pos.1 < self.cols
    }

    pub fn update(&mut self, step: &Step, loss: usize) -> bool {
        let index: usize = self.to_index(&step.pos);
        let tile: &mut Tile = &mut self.tiles[index];
        tile.memo.update(step, loss)
    }

    pub fn get_best_memo(&self) -> &Memo {
        let br_pos: (isize, isize) = (self.rows - 1, self.cols - 1);
        let index: usize = self.to_index(&br_pos);
        &self.tiles[index].memo
    }

    fn to_index(&self, pos: &(isize, isize)) -> usize {
        (pos.0 * self.cols + pos.1) as usize
    }
}
