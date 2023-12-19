use std::collections::HashMap;

use crate::solutions::Solution;
use crate::solutions::day19::common::*;

// Fairly straightforward, if maybe a bit too verbose. Not much to say about this
// one, for each part we simply check each rule in each workflow until we find one
// of the two stopping conditions. After a couple of days where I really struggled
// with bugs, it's nice to get the right answer on the very first attempt.

/* ------------------------------- Comparator ------------------------------- */

impl Comparator {
    fn check(&self, value: usize) -> bool {
        match self {
            Comparator::GreaterThan(limit) => value > *limit,
            Comparator::LessThan(limit)    => value < *limit
        }
    }
}

/* ---------------------------------- Rule ---------------------------------- */

impl Rule {
    fn check(&self, part: &Part) -> Option<String> {
        match self {
            Rule::Always(target) => Some(target.clone()),
            Rule::Check(index, cmp, target) => {
                let part_value: usize = part.values[*index];

                if cmp.check(part_value) {
                    Some(target.clone())
                } else {
                    None
                }
            }
        }
    }
}

/* ---------------------------------- Part ---------------------------------- */

struct Part {
    values: Vec<usize>,
    total: usize
}

impl Part {
    fn parse(line: &str) -> Part {
        let fields: Vec<&str> = line[1 .. line.len() - 1].split(',').collect();
        let pairs: Vec<(&str, &str)> = fields.into_iter()
            .map(|f| f.split_once('=').unwrap()).collect();

        let mut values: Vec<usize> = vec![0; 4];
        let mut total: usize = 0;

        for pair in pairs {
            let index: usize = xmas_to_index(pair.0);
            let value: usize = pair.1.parse().unwrap();
            values[index] = value;
            total += value;
        }

        Part { values, total }
    }

    fn apply(&self, workflow_label: String, workflow_map: &HashMap<String, Workflow>) -> String {
        let workflow: &Workflow = workflow_map.get(&workflow_label).unwrap();

        for rule in &workflow.rules {
            if let Some(target) = rule.check(self) {
                return target;
            }
        }

        panic!()
    }
}

/* ---------------------------------- Main ---------------------------------- */

fn apply_all(parts: Vec<Part>, workflow_map: HashMap<String, Workflow>) -> usize {
    let mut total: usize = 0;

    for part in parts {
        let mut current_workflow: String = String::from("in");

        loop {
            current_workflow = part.apply(current_workflow, &workflow_map);

            if current_workflow == "A" {
                total += part.total;
                break;
            } else if current_workflow == "R" {
                break;
            }
        }
    }

    total
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let blocks: Vec<&[String]> = lines.split(|line| line.is_empty()).collect();
    let workflows: Vec<Workflow> = blocks[0].iter().map(|line| Workflow::parse(line)).collect();
    let parts: Vec<Part> = blocks[1].iter().map(|line| Part::parse(line)).collect();

    let mut workflow_map: HashMap<String, Workflow> = HashMap::new();
    workflows.into_iter().for_each(|w| { workflow_map.insert(w.label.clone(), w); });

    let result: usize = apply_all(parts, workflow_map);
    return Solution::Integer(result as i64)
}
