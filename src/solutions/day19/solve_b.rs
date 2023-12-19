use std::collections::HashMap;

use crate::solutions::Solution;
use crate::solutions::day19::common::*;

// Two days in a row where the solution to the B part comes down to "let's
// use ranges instead of discrete objects". In this case, we start with a
// single composite part with ranges from 1 to 4000 for all four values,
// and we run this part through the first workflow. For each rule in the
// workflow, we find the part of the target range that matches the rule's
// predicate, and the part that doesn't; either of these parts may be empty.
// The matching range is used to create a new part at the target workflow,
// and the part moving through the current workflow is narrowed down to the
// non-matching range. After checking all rules in the workflow in this way,
// we end up with one or more new parts, which are then all either added to
// the back of the queue, or accepted (increasing the total combination count
// by the product of the length of its four remaining ranges), or rejected.
// We then take the next part from this queue, and repeat the process until
// there's no more new parts in the queue.

/* ------------------------------- Comparator ------------------------------- */

impl Comparator {
    fn get_matching(&self, min: usize, max: usize) -> Option<(usize, usize)> {
        match self {
            Comparator::GreaterThan(limit) => {
                if max > *limit {
                    Some(((*limit + 1).max(min), max))
                } else {
                    None
                }
            },

            Comparator::LessThan(limit) => {
                if min < *limit {
                    Some((min, (*limit - 1).min(max)))
                } else {
                    None
                }
            }
        }
    }

    fn get_non_matching(&self, min: usize, max: usize) -> Option<(usize, usize)> {
        match self {
            Comparator::GreaterThan(limit) => {
                if min <= *limit {
                    Some((min, (*limit).min(max)))
                } else {
                    None
                }
            },

            Comparator::LessThan(limit) => {
                if max >= *limit {
                    Some(((*limit).max(min), max))
                } else {
                    None
                }
            }
        }
    }
}

/* ---------------------------------- Rule ---------------------------------- */

impl Rule {
    fn apply(&self, part: &Part) -> (Option<Part>, Option<Part>) {
        match self {
            Rule::Always(target) => {
                (Some(part.copy(target.clone())), None)
            },

            Rule::Check(index, cmp, target) => {
                let mut new: Option<Part> = None;
                let mut rem: Option<Part> = None;
                let min: usize = part.mins[*index];
                let max: usize = part.maxs[*index];

                if let Some(matching) = cmp.get_matching(min, max) {
                    let mut new_part: Part = part.copy(target.clone());
                    new_part.mins[*index] = matching.0;
                    new_part.maxs[*index] = matching.1;
                    new = Some(new_part);
                }

                if let Some(non_matching) = cmp.get_non_matching(min, max) {
                    let mut remaining_part: Part = part.clone();
                    remaining_part.mins[*index] = non_matching.0;
                    remaining_part.maxs[*index] = non_matching.1;
                    rem = Some(remaining_part);
                }

                (new, rem)
            }
        }
    }
}

/* -------------------------------- Workflow -------------------------------- */

impl Workflow {
    fn apply(&self, part: &Part) -> Vec<Part> {
        let mut new_parts: Vec<Part> = Vec::new();
        let mut remaining: Option<Part> = Some(part.clone());

        for rule in self.rules.iter() {
            if let Some(remaining_part) = remaining {
                let (new, rem) = rule.apply(&remaining_part);

                if let Some(new_part) = new {
                    new_parts.push(new_part);
                }

                remaining = rem;
            } else {
                break;
            }
        }

        new_parts
    }
}

/* ---------------------------------- Part ---------------------------------- */

struct Part {
    at: String,
    mins: Vec<usize>,
    maxs: Vec<usize>
}

impl Part {
    fn initial() -> Part {
        Part { at: String::from("in"), mins: vec![1; 4], maxs: vec![4000; 4] }
    }

    fn copy(&self, new_at: String) -> Part {
        Part { at: new_at, mins: self.mins.clone(), maxs: self.maxs.clone() }
    }

    fn clone(&self) -> Part {
        Part { at: self.at.clone(), mins: self.mins.clone(), maxs: self.maxs.clone() }
    }

    fn score(&self) -> usize {
        (0 .. 4).map(|i| self.maxs[i] - self.mins[i] + 1).product()
    }
}

/* ---------------------------------- Main ---------------------------------- */

fn apply_all(workflow_map: HashMap<String, Workflow>) -> usize {
    let mut parts: Vec<Part> = vec!(Part::initial());
    let mut total: usize = 0;
    let mut index: usize = 0;


    while index < parts.len() {
        let part: &Part = &parts[index];
        let workflow: &Workflow = workflow_map.get(&part.at).unwrap();
        let new_parts: Vec<Part> = workflow.apply(part);

        for new_part in new_parts {
            if new_part.at == "A" {
                total += new_part.score();
            } else if new_part.at != "R" {
                parts.push(new_part);
            }
        }

        index += 1;
    }

    total
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let blocks: Vec<&[String]> = lines.split(|line| line.is_empty()).collect();
    let workflows: Vec<Workflow> = blocks[0].iter().map(|line| Workflow::parse(line)).collect();

    let mut workflow_map: HashMap<String, Workflow> = HashMap::new();
    workflows.into_iter().for_each(|w| { workflow_map.insert(w.label.clone(), w); });
    let result: usize = apply_all(workflow_map);
    return Solution::Integer(result as i64)
}
