pub mod day01 {     pub mod solve_a;    pub mod solve_b;                    }
pub mod day02 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day03 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day04 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day05 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day06 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day07 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day08 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day09 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day10 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day11 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day12 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day13 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day14 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day15 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day16 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day17 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day18 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day19 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day20 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day21 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day22 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day23 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day24 {     pub mod solve_a;    pub mod solve_b;    mod common;     }
pub mod day25 {     pub mod solve_a;                                        }

/* -------------------------------- Solution -------------------------------- */

pub enum Solution {
    Integer(i64)
}

impl Solution {
    pub fn to_string(&self) -> String {
        match self {
            Solution::Integer(i) => i.to_string()
        }
    }
}

/* --------------------------------- Solvers -------------------------------- */

pub type Solver = fn(&Vec<String>) -> Solution;

pub fn get_solver(task: &str) -> Solver {
    match task {
        "01a" => day01::solve_a::solve,
        "01b" => day01::solve_b::solve,

        "02a" => day02::solve_a::solve,
        "02b" => day02::solve_b::solve,

        "03a" => day03::solve_a::solve,
        "03b" => day03::solve_b::solve,

        "04a" => day04::solve_a::solve,
        "04b" => day04::solve_b::solve,

        "05a" => day05::solve_a::solve,
        "05b" => day05::solve_b::solve,

        "06a" => day06::solve_a::solve,
        "06b" => day06::solve_b::solve,

        "07a" => day07::solve_a::solve,
        "07b" => day07::solve_b::solve,

        "08a" => day08::solve_a::solve,
        "08b" => day08::solve_b::solve,

        "09a" => day09::solve_a::solve,
        "09b" => day09::solve_b::solve,

        "10a" => day10::solve_a::solve,
        "10b" => day10::solve_b::solve,

        "11a" => day11::solve_a::solve,
        "11b" => day11::solve_b::solve,

        "12a" => day12::solve_a::solve,
        "12b" => day12::solve_b::solve,

        "13a" => day13::solve_a::solve,
        "13b" => day13::solve_b::solve,

        "14a" => day14::solve_a::solve,
        "14b" => day14::solve_b::solve,

        "15a" => day15::solve_a::solve,
        "15b" => day15::solve_b::solve,

        "16a" => day16::solve_a::solve,
        "16b" => day16::solve_b::solve,

        "17a" => day17::solve_a::solve,
        "17b" => day17::solve_b::solve,

        "18a" => day18::solve_a::solve,
        "18b" => day18::solve_b::solve,

        "19a" => day19::solve_a::solve,
        "19b" => day19::solve_b::solve,

        "20a" => day20::solve_a::solve,
        "20b" => day20::solve_b::solve,

        "21a" => day21::solve_a::solve,
        "21b" => day21::solve_b::solve,

        "22a" => day22::solve_a::solve,
        "22b" => day22::solve_b::solve,

        "23a" => day23::solve_a::solve,
        "23b" => day23::solve_b::solve,

        "24a" => day24::solve_a::solve,
        "24b" => day24::solve_b::solve,

        "25a" => day25::solve_a::solve,

        _ => panic!("Invalid task '{task}'")
    }
}
