use crate::solutions::Solution;

// I did not finish this one. Mathematics isn't really my strong suit, and so
// despite a lot of scribbled equations and poorly drawn graphs, I wasn't able
// to come up with a way to find the rock's starting point and velocity. The
// commented-out attempt below was based on the idea to first find an X velo-
// city dx and a Y velocity dy, subtract dx and dy from the velocity of each
// hailstone, and (ignoring the Z dimension) check if all modified hailstones
// pass through the same integer point. This approach – which I admittedly got
// from Reddit – could then theoretically be used to find the Z velocity, and
// from there the starting point.

// This approach works for the sample input, but for the actual input it either
// doesn't work, or the required dx and dy are so high that the current exhaus-
// tive approach of finding dx and dy isn't realistic. Having already spent far
// too much time on this, and with no motivation for finding and implementing a
// new approach, I felt fine leaving it here.

// const DELTA: f64 = 0.001;

// impl Hailstone {
//     fn intersect_xy(hailstone1: &Hailstone, hailstone2: &Hailstone, rock_velocity: &(isize, isize)) -> Option<(f64, f64)> {
//         let dx1: f64 = hailstone1.dx - (rock_velocity.0 as f64);
//         let dx2: f64 = hailstone2.dx - (rock_velocity.0 as f64);

//         if f64::abs(dx1) < DELTA && f64::abs(dx2) >= DELTA {
//             let ix: f64 = hailstone1.sx;
//             let dy2: f64 = hailstone2.dy - (rock_velocity.1 as f64);

//             let a2: f64 = dy2 / dx2;
//             let b2: f64 = hailstone2.sy - a2 * hailstone2.sx;

//             let iy: f64 = a2 * ix + b2;
//             return Some((ix, iy))
//         } else if f64::abs(dx2) < DELTA  && f64::abs(dx1) >= DELTA {
//             let ix: f64 = hailstone2.sx;
//             let dy1: f64 = hailstone1.dy - (rock_velocity.1 as f64);

//             let a1: f64 = dy1 / dx1;
//             let b1: f64 = hailstone1.sy - a1 * hailstone1.sx;

//             let iy: f64 = a1 * ix + b1;
//             return Some((ix, iy))
//         }

//         let dy1: f64 = hailstone1.dy - (rock_velocity.1 as f64);
//         let dy2: f64 = hailstone2.dy - (rock_velocity.1 as f64);

//         let a1: f64 = dy1 / dx1;
//         let a2: f64 = dy2 / dx2;

//         if f64::abs(a1 - a2) < DELTA {
//             // lines are parallel
//             return None;
//         }

//         let b1: f64 = hailstone1.sy - a1 * hailstone1.sx;
//         let b2: f64 = hailstone2.sy - a2 * hailstone2.sx;

//         let ix: f64 = (b2 - b1) / (a1 - a2);
//         let iy: f64 = a1 * ix + b1;
//         Some((ix, iy))
//     }
// }

// fn is_integer(v: f64) -> bool { f64::abs(f64::round(v) - v) < DELTA }
// fn is_integer_pair(p: (f64, f64)) -> bool { is_integer(p.0) && is_integer(p.1) }
// fn to_integer_pair(p: (f64, f64)) -> (isize, isize) { (f64::round(p.0) as isize, f64::round(p.1) as isize) }

// fn test_xy_velocity(hailstones: &Vec<Hailstone>, rock_velocity: (isize, isize)) -> bool {
//     let mut found_intersection: bool = false;
//     let mut intersection: (isize, isize) = (0, 0);

//     for i in 0 .. hailstones.len() - 1 {
//         for j in i + 1 .. hailstones.len() {
//             if let Some(new_intersection) = Hailstone::intersect_xy(&hailstones[i], &hailstones[j], &rock_velocity) {
//                 if !is_integer_pair(new_intersection) {
//                     // println!("{:?}: not an integer pair: {:?}", rock_velocity, new_intersection);
//                     return false;
//                 }

//                 let ip: (isize, isize) = to_integer_pair(new_intersection);

//                 if !found_intersection {
//                     found_intersection = true;
//                     intersection = ip;
//                 } else if intersection != ip {
//                     // println!("{:?}: different intersection: {:?} != {:?}", rock_velocity, intersection, ip);
//                     return false;
//                 }
//             }
//         } 
//     }

//     println!("{:?}: valid intersection: {:?}", rock_velocity, intersection);
//     return true;
// }

// fn test_xy_velocities(hailstones: &Vec<Hailstone>, ix: isize, iy: isize) -> Option<(isize, isize)> {
//     if test_xy_velocity(hailstones, (ix, iy)) {
//         return Some((ix, iy));
//     } else if ix > 0 && test_xy_velocity(hailstones, (-ix, iy)) {
//         return Some((-ix, iy));
//     } else if iy > 0 && test_xy_velocity(hailstones, (ix, -iy)) {
//         return Some((ix, -iy));
//     } else if ix > 0 && iy > 0 && test_xy_velocity(hailstones, (-ix, -iy)) {
//         return Some((-ix, -iy));
//     }

//     None
// }
// fn find_xy_velocity(hailstones: &Vec<Hailstone>) {
//     for ix in 0 .. 500000 {//isize::MAX {
//         for iy in 0 ..= ix {
//             test_xy_velocities(hailstones, ix, iy);
//         }
//     }
// }

pub fn solve(_: &Vec<String>) -> Solution {
    // let hailstones: Vec<Hailstone> = lines.into_iter()
    //     .map(|line| Hailstone::parse(line)).collect();

    // find_xy_velocity(&hailstones);

    return Solution::Integer(-1)
}
