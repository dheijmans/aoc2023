use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: HashMap<String, Module> = parse_modules(input)?;
    insert_conjuction_inputs(&mut map);
    let (mut lows, mut highs): (u32, u32) = (0, 0);
    for _ in 0..1000 {
        let mut queue: VecDeque<(String, String, bool)> =
            VecDeque::from([("button".to_string(), "broadcaster".to_string(), false)]);
        while let Some((from, dest, pulse)) = queue.pop_front() {
            match pulse {
                false => lows += 1,
                true => highs += 1,
            }
            if let Some(Module {
                id,
                module_type,
                outputs,
            }) = map.get_mut(&dest)
            {
                match module_type {
                    ModuleType::Broadcaster => outputs
                        .iter()
                        .for_each(|output| queue.push_back((id.clone(), output.clone(), pulse))),
                    ModuleType::FlipFlop { state } => {
                        if !pulse {
                            *state = !*state;
                            outputs.iter().for_each(|output| {
                                queue.push_back((id.clone(), output.clone(), *state))
                            })
                        }
                    }
                    ModuleType::Conjunction { states } => {
                        *states.get_mut(&from)? = pulse;
                        let all_high: bool = states.iter().all(|(_, &state)| state);
                        outputs.iter().for_each(|output| {
                            queue.push_back((id.clone(), output.clone(), !all_high))
                        })
                    }
                };
            }
        }
    }
    Some(lows * highs)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_modules(input: &str) -> Option<HashMap<String, Module>> {
    let lines = input.lines();
    let mut map: HashMap<String, Module> = HashMap::new();
    for line in lines {
        let (module, outputs) = line.split_once(" -> ")?;
        let outputs: Vec<String> = outputs.split(", ").map(|x| x.to_string()).collect();
        let module = match module.chars().next()? {
            '%' => Module::new(
                module[1..].to_string(),
                ModuleType::FlipFlop { state: false },
                outputs,
            ),
            '&' => Module::new(
                module[1..].to_string(),
                ModuleType::Conjunction {
                    states: HashMap::new(),
                },
                outputs,
            ),
            'b' => Module::new(module.to_string(), ModuleType::Broadcaster, outputs),
            _ => panic!("Unrecognized module!"),
        };
        map.insert(module.id.clone(), module);
    }
    Some(map)
}

fn insert_conjuction_inputs(map: &mut HashMap<String, Module>) {
    let mut conjuctions: Vec<(String, String)> = vec![];
    for (id, module) in &*map {
        for output in &module.outputs {
            conjuctions.push((id.clone(), output.clone()));
        }
    }
    for (input, output) in &conjuctions {
        map.get_mut(output).and_then(|output| {
            output.get_conjunction_states().and_then(|inputs| {
                inputs.insert(input.clone(), false);
                Some(())
            })
        });
    }
}

struct Module {
    id: String,
    module_type: ModuleType,
    outputs: Vec<String>,
}

impl Module {
    fn new(id: String, module_type: ModuleType, outputs: Vec<String>) -> Self {
        Self {
            id,
            module_type,
            outputs,
        }
    }

    fn get_conjunction_states(&mut self) -> Option<&mut HashMap<String, bool>> {
        match &mut self.module_type {
            ModuleType::Conjunction { states } => Some(states),
            _ => None,
        }
    }
}

enum ModuleType {
    Broadcaster,
    FlipFlop { state: bool },
    Conjunction { states: HashMap<String, bool> },
}
