use std::collections::HashSet;
use std::ops::RangeInclusive;

/* -------------------------------- Position -------------------------------- */

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

/* ---------------------------------- Brick --------------------------------- */

pub struct Brick {
    pub id: usize,
    pub positions: Vec<(usize, usize)>,
    pub range_x: RangeInclusive<usize>,
    pub range_y: RangeInclusive<usize>,
    pub range_z: RangeInclusive<usize>,
    pub height: usize,
    pub on: HashSet<usize>
}

impl Brick {
    fn parse(line: &str) -> Brick {
        let (start, end) = line.split_once('~').unwrap();
        let start_pos: Position = Position::parse(start);
        let end_pos: Position = Position::parse(end);

        let mut positions: Vec<(usize, usize)> = Vec::new();

        for x in start_pos.x ..= end_pos.x {
            for y in start_pos.y ..= end_pos.y {
                positions.push((x, y));
            }
        }

        Brick {
            id: usize::MAX,
            positions: positions,
            range_x: (start_pos.x ..= end_pos.x),
            range_y: (start_pos.y ..= end_pos.y),
            range_z: (start_pos.z ..= end_pos.z),
            height: end_pos.z - start_pos.z + 1,
            on: HashSet::new()
        }
    }
}

/* -------------------------------- HeightMap ------------------------------- */

pub struct HeightMap {
    heights: Vec<usize>,
    bricks: Vec<usize>,
    max_x: usize
}

impl HeightMap {
    pub fn create(bricks: &Vec<Brick>) -> HeightMap {
        let max_x: usize = *bricks.iter().map(|b| b.range_x.end()).max().unwrap() + 1;
        let max_y: usize = *bricks.iter().map(|b| b.range_y.end()).max().unwrap() + 1;
        
        let grid_size: usize = max_x * max_y;
        let heights: Vec<usize> = vec![0; grid_size];
        let bricks: Vec<usize> = vec![usize::MAX; grid_size];
        HeightMap { heights, bricks, max_x }
    }

    pub fn place_brick(&mut self, brick: &mut Brick) {
        let max_height: usize = brick.positions.iter().map(|pos| {
            let index: usize = self.to_index(pos);
            self.heights[index]
        }).max().unwrap();

        let mut on_brick_ids: HashSet<usize> = HashSet::new();

        for pos in &brick.positions {
            let index: usize = self.to_index(pos);
            let on_brick_id: usize = self.bricks[index];

            if on_brick_id != usize::MAX && self.heights[index] == max_height {
                on_brick_ids.insert(on_brick_id);
            }

            self.heights[index] = max_height + brick.height;
            self.bricks[index] = brick.id;
        }

        brick.on = on_brick_ids;
    }

    fn to_index(&self, pos: &(usize, usize)) -> usize {
        pos.1 * self.max_x + pos.0
    }
}

/* --------------------------------- Parsing -------------------------------- */

pub fn parse_bricks(lines: &Vec<String>) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = lines.into_iter()
        .map(|line| Brick::parse(line)).collect();

    bricks.sort_by(|a, b| 
        a.range_z.start().cmp(&b.range_z.start())
        .then(a.range_z.end().cmp(&b.range_z.end())));

    bricks.iter_mut().enumerate().for_each(|(i, b)| { b.id = i; });

    bricks
}
