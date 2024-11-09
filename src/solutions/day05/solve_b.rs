use std::ops::Range;

use crate::solutions::Solution;
use crate::solutions::day05::common::*;

// We could of course run all possible seed values through the solution of the
// A part, but that would take ages. Instead, we create a function that takes the
// current seed ranges (i.e., the integer ranges of possible seed values after X
// transformations), and applies the rules of the current map to obtain the seed
// ranges after X+1 transformations. Repeating this for all maps gives us a set
// of output ranges, from which we can easily find the minimum possible output
// value. Since we only have ten initial seed ranges and seven maps, this is
// much faster than trying every single seed.
//
// The transformation step is somewhat complicated, but essentially we're using a
// queue (or "queue", since I'm trying to use zero dependencies, and the standard
// library doesn't include real queues) containing input ranges, initialized using
// the output of the previous step. For each input range, we try all range functions
// in the current map. If we find an overlap, we map this overlap to the destination
// range, and put the non-overlapping part(s) of the current range back in the input
// queue. We keep (re)checking the input ranges against the map rules until there are
// no more overlaps, at which point we add the remaining range is added to the output
// as-is. Repeat until the input queue is empty, and then repeat this transformation
// function for each of the seven maps.
//
// The risk of this approach is that the initial seed ranges might get shattered
// into a huge number of tiny ranges, but with the given seed ranges and maps this
// fortunately doesn't happen: even after the seventh map, the output range list
// only contains 119 entries, and the full solution runs in less than 50µs.

fn resolve_range_map(in_ranges: &Vec<Range<i64>>, range_map: &RangeMap) -> Vec<Range<i64>> {
    let mut current_ranges: Vec<Range<i64>> = in_ranges.clone();
    let mut out_ranges: Vec<Range<i64>> = Vec::new();
    let mut index: usize = 0;

    while index < current_ranges.len() {
        let cr: Range<i64> = current_ranges[index].clone();
        let mut matched: bool = false;

        for range_function in range_map {
            let sr: &Range<i64> = &range_function.source_range;
            let o: i64 = range_function.desintation_offset;

            if cr.end <= sr.start || cr.start >= sr.end {
                // no overlap
                continue;
            }

            if sr.contains(&cr.start) && cr.end <= sr.end {
                // cr:    |---|
                // sr: |--------|
                // map entirety of cr to destination
                out_ranges.push((cr.start + o) .. (cr.end + o));
                matched = true;
                break;
            } else if sr.contains(&cr.start) && cr.end > sr.end {
                // cr:    |--a--:-b-|
                // sr: |--------|
                // map overlap 'a' to destination
                // add remaining part 'b' to queue
                out_ranges.push((cr.start + o) .. (sr.end + o));
                current_ranges.push(sr.end .. cr.end);
                matched = true;
                break;
            } else if cr.start < sr.start && cr.end > sr.end {
                // cr: |-a-:---b---:--c--|
                // sr:     |-------|
                // map overlap 'b' to destination
                // add remaining parts 'a' and 'c' to queue
                out_ranges.push((sr.start + o) .. (sr.end + o));
                current_ranges.push(cr.start .. sr.start);
                current_ranges.push(sr.end .. cr.end);
                matched = true;
                break;
            } else if cr.start < sr.start && cr.end <= sr.end {
                // cr: |-a-:--b--|
                // sr:     |-------|
                // map overlap 'b' to destination
                // add remaining part 'a' to queue
                out_ranges.push((sr.start + o) .. (cr.end + o));
                current_ranges.push(cr.start .. sr.start);
                matched = true;
                break;
            }
        }

        if !matched {
            // if none of the source ranges in the map overlap with the current
            // range, add the current range to the output ranges unmodified
            out_ranges.push(cr);
        }

        index += 1;
    }

    out_ranges
}

fn resolve_seed_ranges(seed_ranges: &Vec<Range<i64>>, range_maps: &Vec<RangeMap>) -> i64 {
    let mut ranges: Vec<Range<i64>> = seed_ranges.clone();

    for range_map in range_maps {
        ranges = resolve_range_map(&ranges, range_map);
    }

    ranges.into_iter().map(|r| r.start).min().unwrap()
}

fn parse_seed_ranges(line: &String) -> Vec<Range<i64>> {
    let seeds: Vec<i64> = line
        .split_once(": ").unwrap().1
        .split(' ').map(|seed| seed.parse::<i64>().unwrap())
        .collect();

    seeds.chunks(2).map(|chunk| chunk[0]..(chunk[0] + chunk[1])).collect()
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let range_maps: Vec<RangeMap> = parse_maps(lines);
    let seed_ranges: Vec<Range<i64>> = parse_seed_ranges(&lines[0]);
    let result: i64 = resolve_seed_ranges(&seed_ranges, &range_maps);
    return Solution::Integer(result)
}
