use std::collections::HashMap;

/* ------------------------------- PulseValue ------------------------------- */

#[derive(Clone, Copy, PartialEq)]
pub enum PulseValue {
    Low,
    High
}

/* ---------------------------------- State --------------------------------- */

pub enum State {
    On,
    Off
}

impl State {
    pub fn flip(&self) -> State {
        match self {
            State::On  => State::Off,
            State::Off => State::On
        }
    }
}

/* ---------------------------------- Pulse --------------------------------- */

pub struct Pulse {
    pub value: PulseValue,
    pub from:  usize,
    pub to:    usize,
}

impl Pulse {
    pub fn initial(initial_to: usize) -> Pulse {
        Pulse {
            value: PulseValue::Low,
            from:  usize::MAX,
            to:    initial_to
        }
    }
}

/* --------------------------------- Module --------------------------------- */

pub trait Module {
    fn process(&mut self, pulse: &Pulse) -> Vec<Pulse>;
    fn register_input(&mut self, input_id: usize);
    fn get_outputs(&self) -> &Vec<usize>;
    fn get_inputs(&self) -> &Vec<usize>;
}

/* ------------------------------- Broadcaster ------------------------------ */

pub struct Broadcaster {
    id:      usize,
    outputs: Vec<usize>,
    inputs:  Vec<usize>
}

impl Module for Broadcaster {
    fn process(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        let output_value: PulseValue = pulse.value;

        self.outputs.iter().map(|out_id| {
            Pulse { 
                value: output_value,
                from:  self.id,
                to:    *out_id
            }
        }).collect()
    }

    fn register_input(&mut self, input_id: usize) { self.inputs.push(input_id); }
    fn get_outputs(&self) -> &Vec<usize> { &self.outputs }
    fn get_inputs(&self) -> &Vec<usize> { &self.inputs }
}

impl Broadcaster {
    pub fn new(id: usize, output_ids: Vec<usize>) -> Box<dyn Module> {
        Box::new(Broadcaster { id: id, outputs: output_ids, inputs: Vec::new() })
    }
}

/* -------------------------------- FlipFlop -------------------------------- */

pub struct FlipFlop {
    id:      usize,
    state:   State,
    outputs: Vec<usize>,
    inputs:  Vec<usize>
}

impl Module for FlipFlop {
    fn process(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        match pulse.value {
            PulseValue::High => Vec::new(),
            PulseValue::Low  => {
                self.state = self.state.flip();
                
                let output_value: PulseValue = match self.state {
                    State::On => PulseValue::High,
                    State::Off => PulseValue::Low
                };

                self.outputs.iter().map(|output_id| {
                    Pulse {
                        value: output_value,
                        from:  self.id,
                        to:    *output_id
                    }
                }).collect()
            }
        }
    }

    fn register_input(&mut self, input_id: usize) { self.inputs.push(input_id); }
    fn get_outputs(&self) -> &Vec<usize> { &self.outputs }
    fn get_inputs(&self) -> &Vec<usize> { &self.inputs }
}

impl FlipFlop {
    pub fn new(id: usize, output_ids: Vec<usize>) -> Box<dyn Module> {
        Box::new(FlipFlop {
            id:      id,
            state:   State::Off,
            outputs: output_ids.clone(),
            inputs:  Vec::new()
        })
    }
}

/* ------------------------------- Conjunction ------------------------------ */

pub struct Conjunction {
    id:      usize,
    memory:  HashMap<usize, PulseValue>,
    outputs: Vec<usize>,
    inputs:  Vec<usize>
}

impl Module for Conjunction {
    fn process(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        self.memory.insert(pulse.from, pulse.value);
        let all_high: bool = self.memory.values().all(|v| *v == PulseValue::High);
        let output_value: PulseValue = if all_high { PulseValue::Low } else { PulseValue::High };

        self.outputs.iter().map(|output_id| {
            Pulse {
                value: output_value,
                from:  self.id,
                to:    *output_id
            }
        }).collect()
    }

    fn register_input(&mut self, input_id: usize) {
        self.memory.insert(input_id, PulseValue::Low);
        self.inputs.push(input_id);
    }

    fn get_outputs(&self) -> &Vec<usize> { &self.outputs }
    fn get_inputs(&self) -> &Vec<usize> { &self.inputs }
}

impl Conjunction {
    pub fn new(id: usize, output_ids: Vec<usize>) -> Box<dyn Module> {
        Box::new(Conjunction {
            id:      id,
            memory:  HashMap::new(),
            outputs: output_ids.clone(),
            inputs:  Vec::new()
        })
    }
}

/* ------------------------------- Main logic ------------------------------- */

pub fn assign_ids(lines: &Vec<String>) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut counter: usize = 0;

    for line in lines {
        let fields: Vec<&str> = line.split(|c: char| !c.is_alphabetic())
            .filter(|field| !field.is_empty()).collect();

        for field in fields {
            let key: String = String::from(field);

            if !map.contains_key(&key) {
                map.insert(key, counter);
                counter += 1;
            }
        }
    }

    map
}

pub fn parse(lines: &Vec<String>, id_map: HashMap<String, usize>) -> HashMap<usize, Box<dyn Module>> {
    let mut modules: HashMap<usize, Box<dyn Module>> = HashMap::new();

    for line in lines {
        let (input, output) = line.split_once(" -> ").unwrap();

        let output_ids: Vec<usize> = output.split(", ").map(|f| *id_map.get(f).unwrap()).collect();

        if input.starts_with('%') {
            let input_id: usize = *id_map.get(&input[1 ..]).unwrap();
            let module: Box<dyn Module> = FlipFlop::new(input_id, output_ids);
            modules.insert(input_id, module);
        } else if input.starts_with('&') {
            let input_id: usize = *id_map.get(&input[1 ..]).unwrap();
            let module: Box<dyn Module> = Conjunction::new(input_id, output_ids);
            modules.insert(input_id, module);
        } else if input == "broadcaster" {
            let input_id: usize = *id_map.get(input).unwrap();
            let module: Box<dyn Module> = Broadcaster::new(input_id, output_ids);
            modules.insert(input_id, module);
        } else {
            panic!();
        }
    }

    modules
}

pub fn register_inputs(modules: &mut HashMap<usize, Box<dyn Module>>) {
    let outputs: Vec<(usize, Vec<usize>)> = modules.iter().map(|(k, v)| {
        (*k, v.get_outputs().clone())
    }).collect();

    for entry in outputs {
        for target_id in entry.1 {
            if let Some(target_module) = modules.get_mut(&target_id) {
                target_module.register_input(entry.0);
            }
        }
    }
}
