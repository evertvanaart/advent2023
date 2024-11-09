use crate::solutions::Solution;
use crate::solutions::day05::common::*;

// First parse each map to a list of range functions, each consisting of
// the source range and an offset from the source range to the destination
// range (i.e. source + offset = destination for any value in the source range).
// We ignore the map names while parsing, and simply assume that the maps are
// listed in the correct order, i.e. first A-to-B, then B-to-C, then C-to-D,
// and so on. Once we've parsed all maps, all we need to do is run each seed
// through all maps; for each map, we iterate through the range functions,
// and apply the first one for which the source range contains the current
// value; if no ranges match, the current value is not modified. After
// doing this for all seed values, we only need to take the minimum.

fn parse_seeds(line: &String) -> Vec<i64> {
    let seeds = line.split_once(": ").unwrap().1;
    seeds.split(' ').map(|seed| seed.parse::<i64>().unwrap()).collect()
}

fn resolve_seed(seed: i64, range_maps: &Vec<RangeMap>) -> i64 {
    let mut value: i64 = seed;

    for map in range_maps {
        for range_function in map {
            if range_function.source_range.contains(&value) {
                value += range_function.desintation_offset;
                break;
            }
        }
    }

    value
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let range_maps: Vec<RangeMap> = parse_maps(lines);
    let seeds: Vec<i64> = parse_seeds(&lines[0]);

    let result: i64 = seeds.into_iter()
        .map(|seed| resolve_seed(seed, &range_maps))
        .min().unwrap();

    return Solution::Integer(result)
}
