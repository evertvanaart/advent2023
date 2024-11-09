use std::collections::HashSet;

use crate::solutions::Solution;
use crate::solutions::day18::common::*;

// A naive solution, but it works. I've verified retroactively that the solution
// of the B part also works for the A part, but given the limited ranges of the
// A part, this naive approach is fast enough. We dig out the loop according to
// the instructions, keeping the coordinates of all excavated tiles in a set.
// In addition, we track the tiles to the left and to the right of the path in
// a separate set. Once we've completed the loop, we determine which of these
// two "side" sets is on the inside, which is easily done by checking which one
// contains only coordinates inside the loop bounds (which we also tracked as we
// were digging the loop). We then grow the inside area from these initial inside
// tiles, i.e. we keep iteratively adding their neighbors to the excavated area
// until we encounter no more new tiles.

/* ------------------------------- Instruction ------------------------------ */

struct Instruction {
    dir: Direction,
    count: usize
}

impl Instruction {
    fn parse(line: &str) -> Instruction {
        let fields: Vec<&str> = line.split(' ').collect();
        let dir: Direction = Direction::parse_a(fields[0]);
        let count: usize = fields[1].parse().unwrap();
        Instruction { dir, count }
    }

    fn apply(&self, digger: &mut Digger, dug_area: &mut DugArea) {
        for _ in 0 .. self.count {
            digger.step(&self.dir);
            dug_area.extend(digger.pos);
        }
    }
}

/* --------------------------------- Digger --------------------------------- */

struct Digger {
    pos: (isize, isize),
    side_l: HashSet<(isize, isize)>,
    side_r: HashSet<(isize, isize)>
}

impl Digger {
    fn new() -> Digger {
        Digger {
            pos: (0, 0),
            side_l: HashSet::new(),
            side_r: HashSet::new()
        }
    }

    fn step(&mut self, dir: &Direction) {
        match dir {
            Direction::North => {
                self.side_l.insert((self.pos.0 - 1, self.pos.1 - 1));
                self.side_r.insert((self.pos.0 - 1, self.pos.1 + 1));
                self.pos.0 -= 1;
            },
            Direction::East  => {
                self.side_l.insert((self.pos.0 - 1, self.pos.1 + 1));
                self.side_r.insert((self.pos.0 + 1, self.pos.1 + 1));
                self.pos.1 += 1;
            },
            Direction::South => {
                self.side_l.insert((self.pos.0 + 1, self.pos.1 + 1));
                self.side_r.insert((self.pos.0 + 1, self.pos.1 - 1));
                self.pos.0 += 1;
            },
            Direction::West  => {
                self.side_l.insert((self.pos.0 + 1, self.pos.1 - 1));
                self.side_r.insert((self.pos.0 - 1, self.pos.1 - 1));
                self.pos.1 -= 1;
            },
        }
    }

    fn valid_side(&self, dug_area: &DugArea) -> &HashSet<(isize, isize)> {
        if self.side_l.iter().all(|pos| dug_area.contains(pos)) {
            &self.side_l
        } else {
            &self.side_r
        }
    }
}

/* --------------------------------- DugArea -------------------------------- */

struct DugArea {
    tiles: HashSet<(isize, isize)>,
    min_row: isize,
    max_row: isize,
    min_col: isize,
    max_col: isize
}

impl DugArea {
    fn new() -> DugArea {
        DugArea {
            tiles: HashSet::from([(0, 0)]),
            min_row: 0,
            max_row: 0,
            min_col: 0,
            max_col: 0
        }
    }

    fn extend(&mut self, pos: (isize, isize)) {
        self.min_row = self.min_row.min(pos.0);
        self.max_row = self.max_row.max(pos.0);
        self.min_col = self.min_col.min(pos.1);
        self.max_col = self.max_col.max(pos.1);
        self.tiles.insert(pos);
    }

    fn grow(&mut self, inside: &HashSet<(isize, isize)>) {
        let mut queue: Vec<(isize, isize)> = Vec::new();
        let mut index: usize = 0;

        for inside_tile in inside {
            queue.push((inside_tile.0, inside_tile.1));
        }

        while index < queue.len() {
            let pos: (isize, isize) = queue[index];

            if !self.tiles.contains(&pos) {
                queue.push((pos.0 - 1, pos.1));
                queue.push((pos.0 + 1, pos.1));
                queue.push((pos.0, pos.1 - 1));
                queue.push((pos.0, pos.1 + 1));
                self.tiles.insert(pos);
            }

            index += 1;
        }
    }

    fn contains(&self, pos: &(isize, isize)) -> bool {
        pos.0 >= self.min_row && pos.0 <= self.max_row && pos.1 >= self.min_col && pos.1 <= self.max_col
    }
}

/* ---------------------------------- Main ---------------------------------- */

pub fn solve(lines: &Vec<String>) -> Solution {
    let instructions: Vec<Instruction> = lines.iter()
        .map(|l| Instruction::parse(l)).collect();

    let mut dug_area: DugArea = DugArea::new();
    let mut digger: Digger = Digger::new();
    
    for instruction in instructions {
        instruction.apply(&mut digger, &mut dug_area);
    }

    let inside: &HashSet<(isize, isize)> = digger.valid_side(&dug_area);
    
    dug_area.grow(inside);

    let result: i64 = dug_area.tiles.len() as i64;
    return Solution::Integer(result)
}
