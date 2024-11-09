use crate::solutions::Solution;
use crate::solutions::day24::common::*;

// A straightforward exhaustive solution, simply check all pairs of hailstones
// for intersection inside the specified zone. The only challenging part was
// figuring out how to find intersections for these kinds of equations. Note
// that the area bounds need to be changed in the code in order to get the
// correct answer for the sample input, see constants defined below.

// sample
// const AREA_MIN: f64 =  7.0;
// const AREA_MAX: f64 = 27.0;

// main
const AREA_MIN: f64 = 200000000000000.0;
const AREA_MAX: f64 = 400000000000000.0;

/* ----------------------------- HailstonePath2D ---------------------------- */

struct HailstonePath2D {
    dx: f64,
    sx: f64,
    a: f64,
    b: f64
}

impl HailstonePath2D {
    fn compute(hailstone: Hailstone) -> HailstonePath2D {
        let a: f64 = hailstone.dy / hailstone.dx;
        let b: f64 = hailstone.sy - a * hailstone.sx;

        HailstonePath2D {
            dx: hailstone.dx,
            sx: hailstone.sx,
            a: a,
            b: b
        }
    }

    fn intersect(path_a: &HailstonePath2D, path_b: &HailstonePath2D) -> bool {
        if path_a.a == path_b.a {
            return false;
        }

        let ix: f64 = (path_b.b - path_a.b) / (path_a.a - path_b.a);

        if ix < AREA_MIN || ix > AREA_MAX {
            return false;
        }

        let ta: f64 = (ix - path_a.sx) / path_a.dx;

        if ta < 0.0 {
            return false;
        }

        let tb: f64 = (ix - path_b.sx) / path_b.dx;

        if tb < 0.0 {
            return false;
        }

        let iy: f64 = path_a.a * ix + path_a.b;

        if iy < AREA_MIN || iy > AREA_MAX {
            return false;
        }

        true
    }
}

/* ------------------------------- Main logic ------------------------------- */

fn count_intersects(paths: Vec<HailstonePath2D>) -> i64 {
    let mut count: i64 = 0;

    for index_a in 1 .. paths.len() {
        let path_a: &HailstonePath2D = &paths[index_a];

        for index_b in 0 .. index_a {
            let path_b: &HailstonePath2D = &paths[index_b];

            if HailstonePath2D::intersect(path_a, path_b) {
                count += 1;
            }
        }
    }

    count
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let hailstones: Vec<Hailstone> = lines.into_iter()
        .map(|line| Hailstone::parse(line)).collect();
    let paths: Vec<HailstonePath2D> = hailstones.into_iter()
        .map(|hs| HailstonePath2D::compute(hs)).collect();

    let result: i64 = count_intersects(paths);

    return Solution::Integer(result)
}
