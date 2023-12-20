use crate::utils::*;

#[derive(Debug, Clone)]
enum Module {
    Broadcaster(Vec<&'static str>),
    FlipFlop {
        name: &'static str,
        state: bool,
        outputs: Vec<&'static str>,
    },
    Conjunction {
        name: &'static str,
        inputs: HashMap<&'static str, bool>,
        outputs: Vec<&'static str>,
    },
}

impl Module {
    fn new(name: &'static str, outputs: Vec<&'static str>) -> (&'static str, Self) {
        let mut out_name = name;
        let module = if let Some(name) = name.strip_prefix('%') {
            out_name = name;
            Module::FlipFlop {
                name,
                state: false,
                outputs,
            }
        } else if let Some(name) = name.strip_prefix('&') {
            out_name = name;
            Module::Conjunction {
                name,
                inputs: HashMap::new(),
                outputs,
            }
        } else {
            Module::Broadcaster(outputs)
        };
        (out_name, module)
    }

    fn add_input(&mut self, input: &'static str) {
        match self {
            Module::Conjunction { inputs, .. } => {
                inputs.insert(input, false);
            }
            _ => {}
        }
    }

    fn handle_signal(
        &mut self,
        signal: bool,
        src: &'static str,
        out_signals: &mut VecDeque<(&'static str, &'static str, bool)>,
    ) -> bool {
        let out_name;
        let out_signal;
        let out: &Vec<&'static str>;
        match self {
            Module::Broadcaster(outputs) => {
                out_name = "broadcaster";
                out_signal = signal;
                out = outputs;
            }
            Module::FlipFlop {
                name,
                state,
                outputs,
            } => {
                if signal {
                    return false;
                }
                *state = !*state;
                out_signal = *state;
                out_name = name;
                out = outputs;
            }
            Module::Conjunction {
                name,
                inputs,
                outputs,
            } => {
                *inputs.get_mut(src).unwrap() = signal;
                out_signal = !inputs.values().all(|&v| v);
                out_name = name;
                out = outputs;
            }
        }
        out_signals.extend(out.iter().map(|&o| (out_name, o, out_signal)));
        out_signal
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let mut modules = HashMap::new();
    let mut all_outputs = HashMap::new();

    for line in input.lines() {
        let (from, to) = line.split_once(" -> ").unwrap();
        let to = to.split(", ").to_vec();

        let (from, module) = Module::new(from, to.clone());
        all_outputs.insert(from, to);
        modules.insert(from, module);
    }

    for (name, outputs) in &all_outputs {
        for output in outputs {
            if let Some(module) = modules.get_mut(output) {
                module.add_input(name);
            }
        }
    }

    let last_one = *all_outputs
        .iter()
        .find(|(_, outputs)| outputs.contains(&"rx"))
        .unwrap()
        .0;

    let last_inputs = all_outputs
        .iter()
        .filter(|(_, outputs)| outputs.contains(&last_one))
        .map(|(k, _)| *k)
        .to_set();

    let mut first_on = HashMap::new();

    let mut queue = VecDeque::new();
    'outer: for presses in 1.. {
        queue.push_back(("button", "broadcaster", false));
        while let Some((src, name, signal)) = queue.pop_front() {
            let Some(module) = modules.get_mut(name) else {
                continue;
            };
            let out_signal = module.handle_signal(signal, src, &mut queue);
            if out_signal && last_inputs.contains(name) && !first_on.contains_key(src) {
                first_on.insert(src, presses);
                if first_on.len() == last_inputs.len() {
                    break 'outer;
                }
            }
        }
    }
    let result = first_on.values().product::<usize>();
    pv!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let mut modules = HashMap::new();
    let mut all_outputs = HashMap::new();

    for line in input.lines() {
        let (from, to) = line.split_once(" -> ").unwrap();
        let to = to.split(", ").to_vec();

        let (from, module) = Module::new(from, to.clone());
        all_outputs.insert(from, to);
        modules.insert(from, module);
    }

    for (name, outputs) in all_outputs.iter() {
        for output in outputs {
            if let Some(module) = modules.get_mut(output) {
                module.add_input(name);
            }
        }
    }

    let mut queue = VecDeque::new();
    let mut low_count = 0;
    let mut high_count = 0;
    for _ in 0..1000 {
        queue.push_back(("button", "broadcaster", false));
        while let Some((src, name, signal)) = queue.pop_front() {
            if signal {
                high_count += 1;
            } else {
                low_count += 1;
            }
            let Some(module) = modules.get_mut(name) else {
                continue;
            };
            module.handle_signal(signal, src, &mut queue);
        }
    }
    pv!(low_count * high_count);
}
