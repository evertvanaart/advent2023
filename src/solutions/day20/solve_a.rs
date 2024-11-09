use std::collections::HashMap;

use crate::solutions::Solution;
use crate::solutions::day20::common::*;

// As usual, for the A part I went with a straightforward implementation of
// the described rules. We press the button 1000 times, and each button press
// generates an initial pulse going towards the broadcaster module. We then
// determine the output pulses resulting from this input pulse (if any) and
// add them to the back of our queue. Using a queue (or queue-like vector)
// means we automatically process the pulses in the correct order, i.e. we
// will process all pulses of the current generation before we process any
// pulses of the next generation. The processing itself is straightforward,
// and implemented in each of the three `Module` implementations. When con-
// structing the network of modules, we convert the textual labels to indices
// for slightly better performance, and we do an additional pass over the
// parsed modules to register the inputs of each module. We keep track of
// the number of high and low pulses generated during each press, and at
// the end we multiply the sums of these two sets of values.

fn push_button(modules: &mut HashMap<usize, Box<dyn Module>>, broadcaster_id: usize) -> (i64, i64) {
    let mut pulses: Vec<Pulse> = vec!(Pulse::initial(broadcaster_id));
    let mut high_pulses: usize = 0;
    let mut low_pulses: usize = 1;
    let mut index: usize = 0;

    while index < pulses.len() {
        let pulse: &Pulse = &pulses[index];

        if let Some(module) = modules.get_mut(&pulse.to) {
            let new_pulses: Vec<Pulse> = module.process(pulse);
            
            for new_pulse in new_pulses {
                match new_pulse.value {
                    PulseValue::High => { high_pulses += 1; }
                    PulseValue::Low  => { low_pulses  += 1; }
                }

                pulses.push(new_pulse);
            }
        }

        index += 1;
    }

    (low_pulses as i64, high_pulses as i64)
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let id_map: HashMap<String, usize> = assign_ids(lines);
    let broadcaster_id: usize = *id_map.get("broadcaster").unwrap();
    let mut modules: HashMap<usize, Box<dyn Module>> = parse(lines, id_map);
    register_inputs(&mut modules);

    let (low_pulses, high_pulses) = (0 .. 1000)
        .map(|_| push_button(&mut modules, broadcaster_id) )
        .fold((0, 0), |acc, v| (acc.0 + v.0, acc.1 + v.1));

    return Solution::Integer(low_pulses * high_pulses)
}
