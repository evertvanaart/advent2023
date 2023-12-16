/* -------------------------------- Direction ------------------------------- */

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West
}

/* ---------------------------------- Tile ---------------------------------- */

pub enum TileType {
    Empty,
    MirrorF,
    MirrorB,
    SplitterH,
    SplitterV
}

pub struct Tile {
    tile_type: TileType,
    visited: Vec<bool>,
    energized: usize
}

impl Tile {
    fn parse(c: char) -> Tile {
        let tile_type: TileType = match c {
            '.'  => TileType::Empty,
            '/'  => TileType::MirrorF,
            '\\' => TileType::MirrorB,
            '-'  => TileType::SplitterH,
            '|'  => TileType::SplitterV,
            _    => { panic!(); }
        };

        Tile { tile_type: tile_type, visited: vec![false; 4], energized: 0 }
    }

    fn reset(&mut self) {
        self.visited = vec![false; 4];
        self.energized = 0;
    }

    fn change_direction(&self, dir: Direction) -> Direction {
        match self.tile_type {
            TileType::MirrorF => {
                match dir {
                    Direction::North => Direction::East,
                    Direction::East  => Direction::North,
                    Direction::South => Direction::West,
                    Direction::West  => Direction::South
                }
            },
            TileType::MirrorB => {
                match dir {
                    Direction::North => Direction::West,
                    Direction::East  => Direction::South,
                    Direction::South => Direction::East,
                    Direction::West  => Direction::North
                }
            },
            _ => dir
        }
    }

    fn visit(&mut self, dir: &Direction) -> bool {
        let index: usize = match dir {
            Direction::North => 0,
            Direction::East  => 1,
            Direction::South => 2,
            Direction::West  => 3
        };

        let already_visited: bool = self.visited[index];

        if !already_visited {
            self.visited[index] = true;
            self.energized += 1;
            return true;
        } else {
            return false;
        }
    }
}

/* ---------------------------------- Grid ---------------------------------- */

pub struct Grid {
    tiles: Vec<Tile>,
    pub energy: usize,
    pub rows: isize,
    pub cols: isize
}

impl Grid {
    pub fn parse(lines: &[String]) -> Grid {
        let tiles: Vec<Tile> = lines.concat().chars()
            .map(|c| Tile::parse(c)).collect();

        Grid {
            tiles: tiles,
            energy: 0,
            rows: lines.len() as isize,
            cols: lines[0].len() as isize
        }
    }

    pub fn reset(&mut self) {
        self.tiles.iter_mut().for_each(|t| t.reset());
        self.energy = 0;
    }

    fn visit(&mut self, beam: &Beam) -> bool {
        let index: usize = self.to_index(&beam.pos);
        let tile: &mut Tile = &mut self.tiles[index];
        let is_new: bool = tile.visit(&beam.dir);

        if is_new && tile.energized == 1 {
            self.energy += 1;
        }

        is_new
    }

    fn contains(&self, pos: &(isize, isize)) -> bool {
        pos.0 >= 0 && pos.0 < self.rows && pos.1 >= 0 && pos.1 < self.cols
    }

    fn get(&self, pos: &(isize, isize)) -> &Tile {
        &self.tiles[self.to_index(pos)]
    }

    fn to_index(&self, pos: &(isize, isize)) -> usize {
        (pos.0 * self.cols + pos.1) as usize
    }
}

/* ---------------------------------- Beam ---------------------------------- */

pub struct Beam {
    pos: (isize, isize),
    dir: Direction
}

impl Beam {
    pub fn new(pos: (isize, isize), dir: Direction) -> Beam {
        Beam { pos, dir }
    }

    fn copy(&self, dir: Direction) -> Beam {
        Beam { pos: self.pos, dir }
    }

    fn step(&mut self, grid: &Grid) -> bool {
        let next_pos: (isize, isize) = match self.dir {
            Direction::North => (self.pos.0 - 1, self.pos.1),
            Direction::East  => (self.pos.0, self.pos.1 + 1),
            Direction::South => (self.pos.0 + 1, self.pos.1),
            Direction::West  => (self.pos.0, self.pos.1 - 1),
        };

        if !grid.contains(&next_pos) {
            return false;
        }

        let next_tile: &Tile = grid.get(&next_pos);
        let next_dir: Direction = next_tile.change_direction(self.dir);

        self.pos = next_pos;
        self.dir = next_dir;
        true
    }

    fn split(&self, grid: &Grid) -> Option<Vec<Beam>> {
        let tile: &Tile = grid.get(&self.pos);

        match tile.tile_type {
            TileType::SplitterH => {
                match self.dir {
                    Direction::East  | Direction::West  => None,
                    Direction::North | Direction::South => {
                        Some(vec!(
                            self.copy(Direction::East),
                            self.copy(Direction::West)
                        ))
                    }
                }
            },
            TileType::SplitterV => {
                match self.dir {
                    Direction::North | Direction::South => None,
                    Direction::East  | Direction::West  => {
                        Some(vec!(
                            self.copy(Direction::North),
                            self.copy(Direction::South)
                        ))
                    }
                }
            },
            _ => None
        }
    }
}

/* ------------------------------- Main logic ------------------------------- */

pub fn trace(grid: &mut Grid, beams: &mut Vec<Beam>, index: usize) -> Option<Vec<Beam>> {
    let beam: &mut Beam = &mut beams[index];

    loop {
        let should_continue: bool = beam.step(&grid);

        if !should_continue {
            return None;
        }

        let is_new: bool = grid.visit(&beam);

        if !is_new {
            return None;
        }

        if let Some(new_beams) = beam.split(grid) {
            return Some(new_beams);
        }
    }
}
