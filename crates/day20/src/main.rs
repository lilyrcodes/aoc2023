use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum PulseState {
    Low,
    High,
}

impl Default for PulseState {
    fn default() -> Self {
        Self::Low
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Pulse {
    state: PulseState,
    source: String,
    destination: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum PartKind {
    Button,
    Broadcaster,
    FlipFlop {
        on: bool,
    },
    Conjunction {
        input_state: HashMap<String, PulseState>,
    },
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Part {
    kind: PartKind,
    id: String,
    destinations: Vec<String>,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let (kind_and_name, destinations) = value.split_once(" -> ").unwrap();
        let (kind, id) = match kind_and_name {
            "broadcaster" => (PartKind::Broadcaster, String::from("broadcaster")),
            _ => match kind_and_name.split_at(1) {
                ("%", name) => (PartKind::FlipFlop { on: false }, String::from(name)),
                ("&", name) => (
                    PartKind::Conjunction {
                        input_state: HashMap::default(),
                    },
                    String::from(name),
                ),
                _ => panic!("Unknown part type!"),
            },
        };
        let destinations = destinations.split(", ").map(String::from).collect();
        Self {
            kind,
            id,
            destinations,
        }
    }
}

impl Part {
    fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        match &self.kind {
            PartKind::Broadcaster => self
                .destinations
                .iter()
                .map(|d| Pulse {
                    source: self.id.clone(),
                    destination: d.clone(),
                    state: pulse.state,
                })
                .collect(),
            PartKind::FlipFlop { on } => todo!(),
            PartKind::Conjunction { input_state } => todo!(),
            PartKind::Button => panic!("Button can't receive pulses!"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct State {
    parts: HashMap<String, Part>,
    pulses: VecDeque<Pulse>,
}

impl From<&str> for State {
    fn from(value: &str) -> Self {
        let mut parts: HashMap<String, Part> = value
            .lines()
            .map(Part::from)
            .map(|p| (p.id.clone(), p))
            .collect();
        parts.insert(
            String::from("button"),
            Part {
                kind: PartKind::Button,
                id: String::from("button"),
                destinations: vec![String::from("broadcaster")],
            },
        );
        Self {
            parts,
            pulses: VecDeque::default(),
        }
    }
}

impl State {
    fn process_pulses(&mut self) -> (usize, usize) {
        let mut low = 0;
        let mut high = 0;
        while let Some(pulse) = self.pulses.pop_front() {
            match pulse.state {
                PulseState::Low => low += 1,
                PulseState::High => high += 1,
            };
            let destination_part = self.parts.get_mut(&pulse.destination).unwrap();
            self.pulses.extend(destination_part.process_pulse(pulse));
        }
        (low, high)
    }

    fn push_button(&mut self) {
        self.pulses.push_back(Pulse {
            state: PulseState::Low,
            source: String::from("button"),
            destination: String::from("broadcaster"),
        });
    }
}

fn part1(s: &str) -> usize {
    let mut state = State::from(s);
    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        state.push_button();
        let (lows, highs) = state.process_pulses();
        low += lows;
        high += highs;
    }
    low * high
}

fn part2(s: &str) -> usize {
    todo!()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&input);
    println!("Part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_SIMPLE_INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    const TEST_INPUT: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_SIMPLE_INPUT), 8_000 * 4_000);
        assert_eq!(part1(TEST_INPUT), 4250 * 2750);
    }

    /*
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 167409079868000);
    }
    */
}
