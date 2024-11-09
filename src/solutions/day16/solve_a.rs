use crate::solutions::Solution;
use crate::solutions::day16::common::*;

// We trace the initial beam until we either leave the grid, or hit a splitter
// at the right angle. When we hit a splitter, we return two new beams, and stop
// tracing the current beam. While tracing the beam, we mark the current tile
// and direction as 'visited' after each step. If we ever try to visit a tile
// and direction that we already visited, we also stop tracing the beam, in
// order to avoid infinite loops. We additionally track the total energy in
// the grid as we are tracing the beam, which we increment whenever we visited
// a tile (going in any direction) for the first time. We're done once we've
// finished tracing all beams, and no new beams have spawned from splitters.

pub fn solve(lines: &Vec<String>) -> Solution {
    let init_beam: Beam = Beam::new((0, -1), Direction::East);
    let mut beams: Vec<Beam> = vec!(init_beam);
    let mut grid: Grid = Grid::parse(lines);
    let mut index: usize = 0;

    while index < beams.len() {
        if let Some(new_beams) = trace(&mut grid, &mut beams, index) {
            new_beams.into_iter().for_each(|b| beams.push(b));
        }

        index += 1;
    }

    return Solution::Integer(grid.energy as i64)
}
