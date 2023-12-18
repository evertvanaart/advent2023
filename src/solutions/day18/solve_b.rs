use std::ops::Range;

use crate::solutions::Solution;
use crate::solutions::day18::common::*;

// The new scale makes the solution of the A part all but infeasible. Instead,
// we look for all segments of the wall surrounding the area, i.e. the border tiles
// dug out before we excavate the center. Each wall segment is either horizontal or
// vertical. Horizontal segments are always inclusive on both ends, while vertical
// segments are inclusive at the top, but exclusive at the bottom; this is needed
// in order to be able to determine whether we're inside or outside, see below.
// 
// Once we've got all wall segments (i.e. we completed the loop by following all
// instructions), we separate the horizontal and the vertical segments, and sort
// both lists; horizontal segments by row and then by start column, vertical
// segments only by column. We then iterate through the horizontal segments,
// grouping any horizontal segments that are on the same row (`h_slice` in
// `calculate_area()`). For each row, we then calculate the area (number of
// excavated tiles) on both this row (A) and the next row (B), and increase
// the total area by A _plus_ B times the distance to the next row containing
// one or more horizontal segments. (There's probably a way to do this while
// calculating only A _or_ B, not both, but I couldn't come up with one).
//
// To calculate the area of a row, we first find all vertical segments that
// intersect with this row, resulting in a list of column indices. We can now
// use the fact that only the tops of vertical segments are inclusive to easily
// determine the column ranges for which the row is "inside" the area; this
// "inside" state flips after every intersecting column, and since we've got
// an enclosed area, the number of intersections is always even. Each pair of
// intersecting columns gives us one range, and to the resulting list we add
// ranges for all horizontal segments in this row, if any. Finding the total
// length of these ranges, minus overlaps, gives us the area per row.

/* ------------------------------- Instruction ------------------------------ */

struct Instruction {
    dir: Direction,
    count: usize
}

impl Instruction {
    fn parse(line: &str) -> Instruction {
        let fields: Vec<&str> = line.split(' ').collect();
        let color_field: &str = fields[2];
        let l: usize = color_field.len();

        let dir_char: &str = &color_field[l - 2 .. l - 1];
        let dir: Direction = Direction::parse_b(dir_char);

        let count_str: &str = &color_field[2 .. l - 2];
        let count: usize = usize::from_str_radix(count_str, 16).unwrap();
        Instruction { dir, count }
    }

    fn apply(&self, digger: &mut Digger) -> WallSegment {
        let start_pos: (isize, isize) = digger.pos;

        for _ in 0 .. self.count {
            digger.step(&self.dir);
        }

        digger.to_segment(start_pos, &self.dir)
    }
}

/* --------------------------------- Digger --------------------------------- */

struct Digger {
    pos: (isize, isize)
}

impl Digger {
    fn new() -> Digger {
        Digger { pos: (0, 0) }
    }

    fn step(&mut self, dir: &Direction) {
        match dir {
            Direction::North => { self.pos.0 -= 1; },
            Direction::East  => { self.pos.1 += 1; },
            Direction::South => { self.pos.0 += 1; },
            Direction::West  => { self.pos.1 -= 1; },
        }
    }

    fn to_segment(&self, start: (isize, isize), dir: &Direction) -> WallSegment {
        match dir {
            Direction::West  => WallSegment::create_horizontal(self.pos.1, start.1, self.pos.0),
            Direction::East  => WallSegment::create_horizontal(start.1, self.pos.1, self.pos.0),
            Direction::North => WallSegment::create_vertical(self.pos.0, start.0, self.pos.1),
            Direction::South => WallSegment::create_vertical(start.0, self.pos.0, self.pos.1),
        }
    }
}

/* ------------------------------- WallSegment ------------------------------ */

struct WallSegment {
    rows: Range<isize>,
    cols: Range<isize>,
    horizontal: bool
}

impl WallSegment {
    fn create_horizontal(col_start: isize, col_end: isize, row: isize) -> WallSegment {
        WallSegment { rows: (row .. row + 1), cols: (col_start .. col_end), horizontal: true }
    }

    fn create_vertical(row_start: isize, row_end: isize, col: isize) -> WallSegment {
        WallSegment { rows: (row_start .. row_end), cols: (col .. col + 1), horizontal: false }
    }
}

/* ---------------------------------- Main ---------------------------------- */

fn sort_segments(all_segments: Vec<WallSegment>) -> (Vec<WallSegment>, Vec<WallSegment>) {
    let mut h_segments: Vec<WallSegment> = Vec::new();
    let mut v_segments: Vec<WallSegment> = Vec::new();

    for segment in all_segments {
        if segment.horizontal {
            h_segments.push(segment);
        } else {
            v_segments.push(segment);
        }
    }

    h_segments.sort_by(|a, b| {
        a.rows.start.cmp(&b.rows.start).then(a.cols.start.cmp(&b.cols.start))
    });

    v_segments.sort_by(|a, b| {
        a.cols.start.cmp(&b.cols.start)
    });

    (h_segments, v_segments)
}

fn calculate_row_area(row: isize, h_slice: &[WallSegment], v_segments: &Vec<WallSegment>) -> usize {
    let v_cols: Vec<isize> = v_segments.iter()
        .filter(|seg| seg.rows.contains(&row))
        .map(|seg| seg.cols.start).collect();

    let mut col_ranges: Vec<(isize, isize)> = (0 .. (v_cols.len() / 2)).map(|index| {
        let col_start: isize = v_cols[2 * index    ];
        let col_end:   isize = v_cols[2 * index + 1];
        (col_start, col_end)
    }).collect();

    h_slice.iter().for_each(|seg| {
        col_ranges.push((seg.cols.start, seg.cols.end))
    });

    col_ranges.sort_by(|a, b| {
        a.0.cmp(&b.0).then(a.1.cmp(&b.1))
    });

    let mut index: usize = 0;
    let mut area: isize = 0;

    while index < col_ranges.len() {
        let current_range: &(isize, isize) = &col_ranges[index];
        let mut current_end: isize = current_range.1;
        index += 1;

        while index < col_ranges.len() && col_ranges[index].0 <= current_end {
            current_end = current_end.max(col_ranges[index].1);
            index += 1;
        }

        area += current_end - current_range.0 + 1;
    }
    
    area as usize
}

fn calculate_area(h_segments: Vec<WallSegment>, v_segments: Vec<WallSegment>) -> usize {
    let mut index: usize = 0;
    let mut area: usize = 0;

    while index < h_segments.len() {
        let row: isize = h_segments[index].rows.start;
        let start_index: usize = index;
        index += 1;

        while index < h_segments.len() && h_segments[index].rows.start == row {
            index += 1;
        }

        let h_slice: &[WallSegment] = &h_segments[start_index .. index];
        area += calculate_row_area(row, h_slice, &v_segments);

        if index < h_segments.len() {
            let row_diff: isize = h_segments[index].rows.start - row - 1;
            let row_area: usize = calculate_row_area(row + 1, &Vec::new(), &v_segments);
            area += row_area * row_diff as usize;
        }
    }

    area
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let instructions: Vec<Instruction> = lines.iter()
        .map(|l| Instruction::parse(l)).collect();

    let mut digger: Digger = Digger::new();

    let segments: Vec<WallSegment> = instructions.iter()
        .map(|i| i.apply(&mut digger)).collect();

    let (h_segments, v_segments) = sort_segments(segments);

    let result: usize = calculate_area(h_segments, v_segments);

    return Solution::Integer(result as i64)
}
