use std::env;
use std::fs::read_to_string;
use std::ops::Div;
use std::process::exit;
use std::time::Duration;
use std::time::Instant;

mod solutions;

/* -------------------------------- Constants ------------------------------- */

const PROFILE_RUNS: u32 = 20;

/* --------------------------------- Helpers -------------------------------- */

struct Arguments {
    task:  String,
    input: String,
    flag:  String
}

fn print_usage() {
    println!("Usage: cargo run  --release <task> <input> [--profile]");
	println!(" <task>     Day number (two digits) plus part ('a' or 'b')");
	println!(" <input>    Input file base name, e.g. 'input' or 'sample'");
	println!(" --profile  Run solution multiple times and compute average duration");
	println!("Example: cargo run  --release 01a sample");
}

fn parse_arguments() -> Arguments {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.len() > 4 {
        print_usage();
        exit(1);
    }
    
    let task:  String = args.get(1).unwrap().clone();
    let input: String = args.get(2).unwrap().clone();
    let flag:  String = args.get(3).unwrap_or(&String::from("")).clone();

    Arguments { task, input, flag }
}

fn parse_day(task: &str) -> &str {
    if task.len() != 3 {
        panic!("Unexpected task format '{task}'")
    }

    &task[0..2]
}

fn read_lines(task: &str, input: &str) -> Vec<String> {
    let day: &str = parse_day(task);
    let filename: String = format!("src/solutions/day{day}/input/{input}.txt");
    let read_result: Result<String, std::io::Error> = read_to_string(&filename);

    let content: String = match read_result {
        Err(_) => panic!("Failed to read file '{filename}'"),
        Ok(s) => s
    };

    return content.lines().map(String::from).collect();
}

/* ---------------------------------- Main ---------------------------------- */

fn main() {
    let arguments: Arguments = parse_arguments();
    let lines: Vec<String> = read_lines(&arguments.task, &arguments.input);
    let solver: solutions::Solver = solutions::get_solver(&arguments.task);

    if arguments.flag == "--profile" {
        println!("Profiling task '{}' using input '{}'", arguments.task, arguments.input);
        let start_time: Instant = Instant::now();

        for _ in 0..PROFILE_RUNS {
            solver(&lines);
        }

        let elapsed: Duration = start_time.elapsed();
        let average: Duration = elapsed.div(PROFILE_RUNS);
        println!("Average duration: {:?}", average);
    } else {
        println!("Running task '{}' on input '{}'", arguments.task, arguments.input);

        let start_time: Instant = Instant::now();
        let solution: solutions::Solution = solver(&lines);
        let elapsed: Duration = start_time.elapsed();

        println!("Elapsed time: {:?}", elapsed);
        println!("Solution: {}", solution.to_string());
    }
}
