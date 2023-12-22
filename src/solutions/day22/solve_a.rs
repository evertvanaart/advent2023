use std::collections::HashSet;
use std::ops::RangeInclusive;

use crate::solutions::Solution;

#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
    z: usize
}

impl Position {
    fn parse(pos_str: &str) -> Position {
        let fields: Vec<usize> = pos_str.split(',')
            .map(|f| f.parse().unwrap()).collect();

        Position { x: fields[0], y: fields[1], z: fields[2] - 1 }
    }
}

struct Brick {
    id: usize,
    positions: Vec<Position>,
    range_x: RangeInclusive<usize>,
    range_y: RangeInclusive<usize>,
    range_z: RangeInclusive<usize>,
    under: HashSet<usize>,
    on: HashSet<usize>
}

impl Brick {
    fn parse(line: &str, line_no: usize) -> Brick {
        let (start, end) = line.split_once('~').unwrap();
        let start_pos: Position = Position::parse(start);
        let end_pos: Position = Position::parse(end);

        let mut positions: Vec<Position> = Vec::new();

        for x in start_pos.x ..= end_pos.x {
            for y in start_pos.y ..= end_pos.y {
                for z in start_pos.z ..= end_pos.z {
                    let pos: Position = Position { x, y, z };
                    positions.push(pos);
                }
            }
        }

        Brick {
            id: line_no,
            positions: positions,
            range_x: (start_pos.x ..= end_pos.y),
            range_y: (start_pos.y ..= end_pos.y),
            range_z: (start_pos.z ..= end_pos.z),
            under: HashSet::new(),
            on: HashSet::new()
        }
    }

    fn add_under(&mut self, under_brick_id: usize) {
        self.under.insert(under_brick_id);
    }
}

struct Grid {
    cells: Vec<usize>,
    max_x: usize,
    max_y: usize
}

impl Grid {
    fn create(bricks: &Vec<Brick>) -> Grid {
        let max_x: usize = *bricks.iter().map(|b| b.range_x.end()).max().unwrap() + 1;
        let max_y: usize = *bricks.iter().map(|b| b.range_y.end()).max().unwrap() + 1;
        let max_z: usize = *bricks.iter().map(|b| b.range_z.end()).max().unwrap() + 1;
        
        let grid_size: usize = max_x * max_y * max_z;
        let cells: Vec<usize> = vec![usize::MAX; grid_size];
        let mut grid: Grid = Grid { cells, max_x, max_y };

        for brick in bricks {
            for pos in &brick.positions {
                let index: usize = grid.to_index(pos);
                grid.cells[index] = brick.id;
            }
        }

        grid
    }

    fn move_brick(&mut self, brick: &mut Brick, z_distance: usize) {
        if z_distance == 0 {
            return;
        }

        let positions: &Vec<Position> = &brick.positions;

        for position in positions {
            let index = self.to_index(position);
            self.cells[index] = usize::MAX;
        }

        let new_range_z: RangeInclusive<usize> = 
            brick.range_z.start() - z_distance ..= brick.range_z.end() - z_distance;

        let new_positions: Vec<Position> = positions.iter().map(|pos| {
            Position { x: pos.x, y: pos.y, z: pos.z - z_distance }
        }).collect();

        for new_position in &new_positions {
            let index = self.to_index(new_position);
            self.cells[index] = brick.id;
        }

        brick.positions = new_positions;
        brick.range_z = new_range_z;
    }

    fn get_bricks(&self, positions: &Vec<Position>, own_id: usize) -> HashSet<usize> {
        HashSet::from_iter(positions.iter().filter_map(|pos| {
            let index: usize = self.to_index(pos);
            let brick_id: usize = self.cells[index];

            if brick_id == own_id || brick_id == usize::MAX {
                None
            } else {
                Some(brick_id)
            }
        }))
    }

    fn to_index(&self, pos: &Position) -> usize {
        pos.z * (self.max_x * self.max_y) + pos.y * self.max_x + pos.x
    }
}

fn drop_brick(grid: &mut Grid, bricks: &mut Vec<Brick>, index: usize) -> Option<HashSet<usize>> {
    let current_brick: &mut Brick = &mut bricks[index];

    if *current_brick.range_z.start() == 0 {
        current_brick.on = HashSet::from([usize::MAX]);
        return None;
    }

    let mut positions: Vec<Position> = current_brick.positions.clone();
    let mut bottom_z: usize = positions[0].z;
    let init_z: usize = bottom_z;

    while bottom_z > 0 {
        positions.iter_mut().for_each(|p| p.z -= 1);
        let on_bricks: HashSet<usize> = grid.get_bricks(&positions, current_brick.id);

        if on_bricks.len() > 0 {
            grid.move_brick(current_brick, init_z - bottom_z);
            current_brick.on = on_bricks.clone();
            return Some(on_bricks);
        }
       
        bottom_z -= 1;
    }

    current_brick.on = HashSet::from([usize::MAX]);
    return None
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let mut bricks: Vec<Brick> = lines.into_iter().enumerate()
        .map(|(line_no, line)| Brick::parse(line, line_no)).collect();

    bricks.sort_by(|a, b| a.range_z.start().cmp(&b.range_z.start()));

    let mut grid: Grid = Grid::create(&bricks);

    for index in 0 .. bricks.len() {
        if let Some(on_bricks) = drop_brick(&mut grid, &mut bricks, index) {
            for on_brick_id in on_bricks {
                let on_brick: &mut Brick = &mut bricks[on_brick_id];
                on_brick.add_under(index);
            }
        }
    }

    let safe_bricks: usize = bricks.iter().filter(|b| {
        b.under.is_empty() || b.under.iter().all(|under| {
            bricks[*under].on.len() > 1
        })
    }).count();

    return Solution::Integer(safe_bricks as i64)
}
