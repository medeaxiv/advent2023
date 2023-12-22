use std::collections::VecDeque;

use ahash::AHashMap as HashMap;

mod parser;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> u64 {
    let modules = parser::parse(input).unwrap();
    let network = Network::from_iter(modules);
    let mut state = network.new_state();

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        low_pulses += 1;
        network.broadcast(Pulse::Low, &mut state, |_, _, pulse| match pulse {
            Pulse::Low => {
                low_pulses += 1;
            }
            Pulse::High => {
                high_pulses += 1;
            }
        });
    }

    low_pulses * high_pulses
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> u64 {
    let modules = parser::parse(input).unwrap();
    let network = Network::from_iter(modules);

    let mut rx_index = network
        .modules
        .iter()
        .position(|module| module.name == "rx")
        .expect("No rx module");

    while network.modules[rx_index].inputs.len() == 1 {
        assert!(
            matches!(
                network.modules[rx_index].module_type,
                ModuleType::None | ModuleType::Conjunction(..)
            ),
            "Path between rx and its first conjunction input must be unique and uninterrupted"
        );

        rx_index = network.modules[rx_index].inputs[0]
    }

    assert!(
        matches!(
            network.modules[rx_index].module_type,
            ModuleType::Conjunction(..)
        ),
        "Observed module must be a conjunction"
    );

    let observed_indices = network.modules[rx_index].inputs.clone();
    let mut cycle_lengths = vec![None; observed_indices.len()];

    let mut presses = 0;
    let mut state = network.new_state();

    loop {
        presses += 1;
        network.broadcast(Pulse::Low, &mut state, |source, destination, pulse| {
            if destination != rx_index || pulse != Pulse::High {
                return;
            }

            if let Some(idx) = observed_indices.iter().position(|idx| *idx == source) {
                cycle_lengths[idx] = Some(presses);
            }
        });

        if cycle_lengths.iter().all(|it| it.is_some()) || presses > 5000 {
            break;
        }
    }

    cycle_lengths
        .iter()
        .map(|it| it.expect("No cycle found"))
        .reduce(aoc_util::numerics::least_common_multiple)
        .unwrap()
}

#[derive(Debug, Clone)]
struct Network<'a> {
    modules: Vec<NetworkModule<'a>>,
    broadcaster: usize,
}

impl<'a> Network<'a> {
    pub fn new_state(&self) -> Vec<ModuleState> {
        self.modules
            .iter()
            .map(|module| module.module_type.new_state())
            .collect()
    }

    pub fn broadcast(
        &self,
        initial_pulse: Pulse,
        state: &mut [ModuleState],
        mut visitor: impl FnMut(usize, usize, Pulse),
    ) {
        assert_eq!(state.len(), self.modules.len());

        let initial_pulses = self.modules[self.broadcaster]
            .destinations
            .iter()
            .map(|&destination| (self.broadcaster, destination, initial_pulse));

        let mut queue = VecDeque::from_iter(initial_pulses);

        while let Some((source, destination, pulse)) = queue.pop_front() {
            visitor(source, destination, pulse);

            if let Some(next_pulse) = state[destination].receive(source, pulse) {
                for &next_destination in self.modules[destination].destinations.iter() {
                    queue.push_back((destination, next_destination, next_pulse));
                }
            }
        }
    }
}

impl<'a> FromIterator<Module<'a>> for Network<'a> {
    fn from_iter<T: IntoIterator<Item = Module<'a>>>(iter: T) -> Self {
        let mut builder = NetworkBuilder::default();

        for module in iter.into_iter() {
            builder.add_module(&module);
        }

        builder.build()
    }
}

#[derive(Default)]
struct NetworkBuilder<'a> {
    name_mapping: HashMap<&'a str, usize>,
    modules: Vec<NetworkModule<'a>>,
    module_inputs: Vec<Vec<usize>>,
    broadcaster: Option<usize>,
}

impl<'a> NetworkBuilder<'a> {
    pub fn add_module(&mut self, module: &Module<'a>) {
        let idx = self.register_name(module.name);
        let destinations = self.extract_destinations(idx, &module.destinations);

        let network_module = &mut self.modules[idx];
        network_module.module_type = module.module_type;
        network_module.destinations = destinations;
    }

    pub fn build(mut self) -> Network<'a> {
        for (idx, inputs) in self.module_inputs.into_iter().enumerate() {
            if let ModuleType::Conjunction(input_count) = &mut self.modules[idx].module_type {
                *input_count = inputs.len();
            }

            self.modules[idx].inputs = inputs;
        }

        Network {
            modules: self.modules,
            broadcaster: self.broadcaster.expect("No broadcaster module"),
        }
    }

    fn register_name(&mut self, name: &'a str) -> usize {
        let idx = if let Some(idx) = self.name_mapping.get(name) {
            *idx
        } else {
            let idx = self.modules.len();
            self.modules.push(NetworkModule::empty(name));
            self.module_inputs.push(Vec::new());
            self.name_mapping.insert(name, idx);
            idx
        };

        if name == "broadcaster" {
            self.broadcaster = Some(idx);
        }

        idx
    }

    fn extract_destinations(
        &mut self,
        source_index: usize,
        destinations: &[&'a str],
    ) -> Vec<usize> {
        let mut network_destinations = Vec::with_capacity(destinations.len());
        for &name in destinations.iter() {
            let idx = self.register_name(name);
            network_destinations.push(idx);
            self.register_input(source_index, idx);
        }

        network_destinations
    }

    fn register_input(&mut self, source_index: usize, destination_index: usize) {
        self.module_inputs[destination_index].push(source_index);
    }
}

#[derive(Debug, Clone)]
struct NetworkModule<'a> {
    name: &'a str,
    module_type: ModuleType,
    inputs: Vec<usize>,
    destinations: Vec<usize>,
}

impl<'a> NetworkModule<'a> {
    pub fn new(
        name: &'a str,
        module_type: ModuleType,
        inputs: Vec<usize>,
        destinations: Vec<usize>,
    ) -> Self {
        Self {
            name,
            module_type,
            inputs,
            destinations,
        }
    }

    pub fn empty(name: &'a str) -> Self {
        Self::new(name, ModuleType::None, Vec::new(), Vec::new())
    }
}

#[derive(Debug, Clone)]
struct Module<'a> {
    name: &'a str,
    module_type: ModuleType,
    destinations: Vec<&'a str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ModuleType {
    None,
    FlipFlop,
    Conjunction(usize),
}

impl ModuleType {
    pub fn new_state(&self) -> ModuleState {
        match self {
            Self::None => ModuleState::None,
            Self::FlipFlop => ModuleState::FlipFlop { active: false },
            Self::Conjunction(input_count) => ModuleState::Conjunction {
                memory: Vec::new(),
                input_count: *input_count,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ModuleState {
    None,
    FlipFlop {
        active: bool,
    },
    Conjunction {
        memory: Vec<(usize, Pulse)>,
        input_count: usize,
    },
}

impl ModuleState {
    pub fn receive(&mut self, source: usize, pulse: Pulse) -> Option<Pulse> {
        match self {
            Self::None => None,
            Self::FlipFlop { active } => {
                if pulse == Pulse::High {
                    None
                } else if *active {
                    *active = false;
                    Some(Pulse::Low)
                } else {
                    *active = true;
                    Some(Pulse::High)
                }
            }
            Self::Conjunction {
                memory,
                input_count,
            } => {
                let mut found = false;
                let mut high = if pulse == Pulse::High { 1 } else { 0 };

                for (input, stored) in memory.iter_mut() {
                    let input = *input;

                    if input == source {
                        found = true;
                        *stored = pulse;
                    } else if *stored == Pulse::High {
                        high += 1;
                    }
                }

                if !found {
                    memory.push((source, pulse));
                }

                if high == *input_count {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

#[cfg(test)]
mod tests {
    use aoc_util::test::setup_tracing;
    use rstest::rstest;

    use super::*;

    const TEST_INPUT1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

    const TEST_INPUT2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

    #[rstest]
    #[case(TEST_INPUT1, 32000000)]
    #[case(TEST_INPUT2, 11687500)]
    fn test_part1(#[case] input: &str, #[case] expected: u64) {
        setup_tracing();
        let solution = solve_part1(input);
        assert_eq!(solution, expected);
    }
}
