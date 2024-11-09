use std::collections::HashSet;

use crate::solutions::Solution;
use crate::solutions::day22::common::*;

// Instead of modeling the whole 3D structure, we only keep track of a height
// map parallel to the ground plane. Each cell in this height map contains the
// current maximum height containing a brick, as well as the ID of this brick.
// As an additional step, we sort the list of input bricks in order from lowest
// to highest Z value. Whe then "drop" bricks onto the height map in sorted order.
// For each brick, we only need to find the current maximum height from the height
// map cells corresponding to the X and Y positions of the brick; since we sorted
// the bricks by Z value, and since the bricks all have regular shapes, we know
// that the brick will come to a rest at this maximum height plus one. We then
// update the height map accordingly, writing the new maximum height and ID of
// the brick we just dropped to the appropriate cells. Additionally, we store
// the IDs of the brick(s) that the current brick is resting on (which we
// obtain from the height map) in the brick object itself.
//
// Once we're done "dropping" all bricks onto the height map in this manner,
// we can easily determine the number of safe bricks. We iterate over all
// bricks; if the list of bricks `on` that the current brick is resting on
// contains exactly one brick, then that single brick in `on` is unsafe,
// and we remove it from the list of potentially safe bricks (initiated
// using all possible brick IDs). Once we've removed all unsafe bricks
// from this set, the size of the remaining set is our answer.

fn count_safe_bricks(bricks: Vec<Brick>) -> usize {
    let ids: Vec<usize> = (0 .. bricks.len()).collect();
    let mut safe_bricks: HashSet<usize> = HashSet::from_iter(ids);

    for brick in bricks {
        if brick.on.len() == 1 {
            let on_brick_id: &usize = brick.on.iter().next().unwrap();
            safe_bricks.remove(on_brick_id);
        }
    }

    safe_bricks.len()
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let mut bricks: Vec<Brick> = parse_bricks(lines);
    let mut height_map: HeightMap = HeightMap::create(&bricks);
    
    for brick in bricks.iter_mut() {
        height_map.place_brick(brick);
    }

    let result: usize = count_safe_bricks(bricks);
    return Solution::Integer(result as i64)
}
