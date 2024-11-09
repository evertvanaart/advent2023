use std::collections::HashMap;

use crate::solutions::Solution;
use crate::solutions::day20::common::*;

// I feel like I cheated on this one. The general gist is obvious; the button
// press count we're looking for is going to be way too high for us to simulate
// it all (trust me, I've tried), so instead we're going to once again look for
// cycles. In particular, we want to find cycles in specific subsets of the net-
// work of modules. We find appropraite subsets by working backwards from the
// target "rx" module until we've found a group of more than one module leading
// to our "rx" output. Multiplying the cycle size - i.e., the number of button
// presses after which a module starts to repeat itself - for each of these
// subsets gives us the least common multiple, which will be our target count.
//
// There are several issues with this approach however:
// - It assumes that a cycle contains exactly one high pulse at the very end
//   of the cycle, while in reality a cycle might have multiple high pulses.
// - It assumes that we're looking for a high pulse in order to determine the
//   cycle boundary. This is based on me visually confirming that there's a
//   Conjunction module on the path to "rx", and that the output of this module
//   is inverted through a second Conjunction module. If the network topology
//   had been even slight different (e.g. if "rx" had been preceded by a
//   flip-flop instead of a conjunction), none of this would have worked.
// - It assumes that the cycles of these subsets are small enough that we
//   can find them in a reasonable amount of time; for all we know, the
//   length of these subset cycles could still have been in the trillions.
// - It assumes that the cycle lengths are all coprimes, meaning that their
//   least common multiple can be found by simple multiplication.
// - It assumes that the cycle starts at zero button presses.
//
// All of these assumptions hold for the given input, and so it spits out
// the correct answer in a fairly short amount of time. It still doesn't
// sit right with me however; as a personal rule, I want the solutions to
// these problems to be as generic as possible, and avoid relying on infor-
// mation or assumptions not explicitly stated in the description. This is
// clearly not the case here, which is why I consider it cheating.
//
// On the other hand, rewriting this to produce the correct solution for
// any hypothetical network in a reasonable amount of time would have been
// a royal pain in the ass. Most of the assumptions outlined above can be
// verified, but the third one especially - the one about the subset cycles
// being reasonably short - is much harder. If this assumption doesn't hold,
// we'd still be looking at a runtime in the order of hours. It should be
// possible to work around that issue (something something network graph
// partitioning), but since we've already got the right answer by making
// a bunch of shaky but true assumptions, why should we?

struct ModuleCycle {
    id: usize,
    count: i64
}

fn find_target_modules(modules: &HashMap<usize, Box<dyn Module>>, rx_id: usize) -> Vec<usize> {
    let mut ids: Vec<usize> = modules.iter()
        .filter(|(_, module)| module.get_outputs().contains(&rx_id))
        .map(|(id, _)| *id).collect();

    while ids.len() == 1 {
        let module: &Box<dyn Module> = modules.get(&ids[0]).unwrap();
        ids = module.get_inputs().clone();
    }

    ids
}

fn push_button(modules: &mut HashMap<usize, Box<dyn Module>>,
        target_cycles: &mut Vec<ModuleCycle>,
        broadcaster_id: usize, count: i64) {
    let mut pulses: Vec<Pulse> = vec!(Pulse::initial(broadcaster_id));
    let mut index: usize = 0;

    while index < pulses.len() {
        let pulse: &Pulse = &pulses[index];

        if let Some(module) = modules.get_mut(&pulse.to) {
            let new_pulses: Vec<Pulse> = module.process(pulse);
            new_pulses.into_iter().for_each(|p| pulses.push(p));
        }

        index += 1;
    }

    target_cycles.iter_mut().filter(|cycle| cycle.count == -1).for_each(|cycle| {
        if pulses.iter().any(|pulse| pulse.from == cycle.id && pulse.value == PulseValue::High) {
            cycle.count = count;
        }
    });
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let id_map: HashMap<String, usize> = assign_ids(lines);
    let broadcaster_id: usize = *id_map.get("broadcaster").unwrap();
    let rx_id: usize = *id_map.get("rx").unwrap();

    let mut modules: HashMap<usize, Box<dyn Module>> = parse(lines, id_map);
    register_inputs(&mut modules);

    let target_modules: Vec<usize> = find_target_modules(&modules, rx_id);
    let mut target_cycles: Vec<ModuleCycle> = target_modules.into_iter()
        .map(|id| ModuleCycle { id: id, count: -1 }).collect();

    let mut button_presses: i64 = 0;

    while target_cycles.iter().any(|cycle| cycle.count == -1) {
        button_presses += 1;
        
        push_button(&mut modules, &mut target_cycles,
            broadcaster_id, button_presses);
    }

    let result: i64 = target_cycles.into_iter().map(|cycle| cycle.count).product();

    return Solution::Integer(result)
}
