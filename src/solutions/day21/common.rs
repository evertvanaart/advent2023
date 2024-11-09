use std::collections::HashSet;

/* ---------------------------------- Tile ---------------------------------- */

pub enum Tile {
    Empty,
    Start,
    Rock
}

/* ---------------------------------- Grid ---------------------------------- */

pub struct Grid {
    pub tiles: Vec<Tile>,
    pub rows: isize,
    pub cols: isize
}

impl Grid {
    pub fn parse(lines: &Vec<String>) -> Grid {
        let tiles: Vec<Tile> = lines.concat().chars().map(|c| {
            match c {
                '.' => Tile::Empty,
                'S' => Tile::Start,
                '#' => Tile::Rock,
                _ => panic!()
            }
        }).collect();

        Grid {
            tiles: tiles,
            rows:  lines.len() as isize,
            cols:  lines[0].len() as isize
        }
    }

    pub fn get(&self, pos: &(isize, isize)) -> &Tile {
        let index: usize = self.to_index(pos);
        &self.tiles[index]
    }

    pub fn contains(&self, pos: &(isize, isize)) -> bool {
        pos.0 >= 0 && pos.0 < self.rows && pos.1 >= 0 && pos.1 < self.cols
    }

    pub fn is_empty(&self, pos: &(isize, isize)) -> bool {
        match self.get(pos) {
            Tile::Empty => true,
            Tile::Start => true,
            Tile::Rock  => false
        }
    }

    fn to_index(&self, pos: &(isize, isize)) -> usize {
        (pos.0 * self.cols + pos.1) as usize
    }

    fn to_coordinates(&self, index: isize) -> (isize, isize) {
        (index / self.cols, index % self.cols)
    }

    pub fn find_start(&self) -> (isize, isize) {
        let index: usize = self.tiles.iter().enumerate().find(|(_, tile)| {
            match tile {
                Tile::Start => true,
                _ => false
            }
        }).map(|(i, _)| i).unwrap();
    
        self.to_coordinates(index as isize)
    }
}

/* ---------------------------------- Main ---------------------------------- */

pub fn add_next(grid: &Grid, next: &mut HashSet<(isize, isize)>, pos: (isize, isize)) {
    if grid.contains(&pos) && grid.is_empty(&pos) {
        next.insert(pos);
    }
}

pub fn step(grid: &Grid, current: HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
    let mut next: HashSet<(isize, isize)> = HashSet::new();

    for pos in current {
        add_next(grid, &mut next, (pos.0 + 1, pos.1));
        add_next(grid, &mut next, (pos.0 - 1, pos.1));
        add_next(grid, &mut next, (pos.0, pos.1 + 1));
        add_next(grid, &mut next, (pos.0, pos.1 - 1));
    }

    next
}
