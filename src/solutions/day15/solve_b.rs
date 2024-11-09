use std::collections::HashMap;

use crate::solutions::Solution;
use crate::solutions::day15::common::*;

// When modeling the box, we use two data structures, a map from label to index,
// and a vector containing the actual lens objects at the indicates index. The
// map allows us to find the position of a lens in a box in O(1). Adding a new
// lens is done by appending to the vector and inserting the new last index into
// the map, and swapping out a lens is done by simply replacing the lens object
// at the target position. The only slightly challenging operation is removing
// a lens; if we'd actually remove the vector item, we'd have to shift all lenses
// to the right of this lens one spot to the left, and update the indices in the
// map accordingly, giving us O(N) complexity. Instead, we simply set the vector
// entry to `None`, and when computing the power of the lenses in the box, we
// skip over these empty entries. This could potentially lead to some very
// sparse vectors, but in practice the largest `lenses` vector is still
// only 16 entries long at the end of the input, empty spots and all. 

/* ---------------------------------- Lens ---------------------------------- */

struct Lens {
    label: String,
    length: usize
}

impl Lens {
    fn parse(field: &str) -> Lens {
        let (label, length_str) = field.split_once('=').unwrap();
        Lens { label: String::from(label), length: length_str.parse().unwrap() }
    }

    fn compute_power(&self, base: usize, pos: usize) -> usize {
        base * pos * self.length
    }
}

/* ----------------------------------- Box ---------------------------------- */

struct Box {
    lens_indices: HashMap<String, usize>,
    lenses: Vec<Option<Lens>>
}

impl Box {
    fn new() -> Box {
        Box { lens_indices: HashMap::new(), lenses: Vec::new() }
    }

    fn remove_lens(&mut self, label: String) {
        if let Some(lens_index) = self.get_lens_index(&label) {
            self.lens_indices.remove(&label);
            self.lenses[lens_index] = None;
        }
    }

    fn add_lens(&mut self, lens: Lens) {
        if let Some(lens_index) = self.get_lens_index(&lens.label) {
            self.lens_indices.insert(lens.label.clone(), lens_index);
            self.lenses[lens_index] = Some(lens);
        } else {
            self.lens_indices.insert(lens.label.clone(), self.lenses.len());
            self.lenses.push(Some(lens));
        }
    }

    fn get_lens_index(&self, label: &String) -> Option<usize> {
        match self.lens_indices.get(label) {
            Some(i) => Some(*i),
            None => None
        }
    }

    fn compute_power(&self, box_index: usize) -> usize {
        let base: usize = box_index + 1;
        let mut lens_pos: usize = 1;
        let mut sum: usize = 0;
        
        for lens in self.lenses.iter() {
            match lens {
                None => {},
                Some(l) => {
                    sum += l.compute_power(base, lens_pos);
                    lens_pos += 1;
                }
            }
        }

        sum
    }
}

/* ------------------------------- Main logic ------------------------------- */

fn get_label(instruction: &str) -> &str {
    let end_index = instruction.chars().enumerate()
        .find(|(_, c)| !c.is_alphabetic())
        .map(|(i, _)| i)
        .unwrap_or(instruction.len());

    &instruction[..end_index]
}

fn apply_step(instruction: &str, boxes: &mut Vec<Box>) {
    let label: &str = get_label(instruction);
    let box_index: usize = compute_hash(label);
    let target_box: &mut Box = &mut boxes[box_index];

    if instruction.ends_with('-') {
        target_box.remove_lens(String::from(label));
    } else {
        target_box.add_lens(Lens::parse(instruction));
    }
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let mut boxes: Vec<Box> = Vec::new();
    (0 .. 256).for_each(|_| boxes.push(Box::new()));

    lines[0].split(',').for_each(|field| apply_step(field, &mut boxes));

    let result: usize = boxes.iter().enumerate()
        .map(|(i, b)| b.compute_power(i)).sum();
    
    return Solution::Integer(result as i64)
}
