use std::collections::HashSet;

use crate::solutions::Solution;
use crate::solutions::day22::common::*;

// We start by repeating the A part; after this, each brick contains a list of
// the IDs that it is resting on. We use this information to create two "maps"
// (since the brick IDs are also 0-based indices, we use vectors instead of
// real maps for better performance). One map contains the IDs of the bricks
// that the target brick is resting on (`on_map`), and the other is essentially
// the inverse, containing a list of bricks that are resting on the target one
// (`under_map`). With these two maps, we can quite easily determine the number
// of bricks that will drop if brick X gets removed: we check each of the bricks
// resting on brick X (via `under_map`), and if all bricks that brick Y is resting
// on (via `on_map`) have dropped, we add Y to the set of dropped bricks, and con-
// tinue checking the bricks that were resting on Y. We continue this process until
// no new bricks drop. There is a probably a more efficient way to do this invol-
// ving memoization to avoid repeated calculations, but dumbly repeating this whole
// process for every single brick is still fairly fast.

fn create_on_map(bricks: &Vec<Brick>) -> Vec<HashSet<usize>> {
    bricks.iter().map(|b| b.on.clone()).collect()
}

fn create_under_map(bricks: &Vec<Brick>, on_map: &Vec<HashSet<usize>>) -> Vec<HashSet<usize>> {
    bricks.iter().map(|brick| {
        HashSet::from_iter(on_map.iter().enumerate()
            .filter(|(_, on)| on.contains(&brick.id))
            .map(|(id, _)| id))
    }).collect()
}

fn drop_all(initial_brick_index: usize, on_map: &Vec<HashSet<usize>>, under_map: &Vec<HashSet<usize>>) -> usize {
    let mut dropped: HashSet<usize> = HashSet::from([initial_brick_index]);
    let mut to_process: Vec<usize> = Vec::new();
    let mut index: usize = 0;

    under_map[initial_brick_index].iter().for_each(|i| to_process.push(*i));

    while index < to_process.len() {
        let checked_brick_index = to_process[index];
        if on_map[checked_brick_index].iter().all(|i| dropped.contains(i)) {
            under_map[checked_brick_index].iter().for_each(|i| to_process.push(*i));
            dropped.insert(checked_brick_index);
        }

        index += 1;
    }

    dropped.len()
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let mut bricks: Vec<Brick> = parse_bricks(lines);
    let mut height_map: HeightMap = HeightMap::create(&bricks);
    
    for brick in bricks.iter_mut() {
        height_map.place_brick(brick);
    }

    let on_map: Vec<HashSet<usize>> = create_on_map(&bricks);
    let under_map: Vec<HashSet<usize>> = create_under_map(&bricks, &on_map);

    let result: usize = bricks.into_iter().map(|brick| {
        drop_all(brick.id, &on_map, &under_map) - 1
    }).sum();

    return Solution::Integer(result as i64)
}
