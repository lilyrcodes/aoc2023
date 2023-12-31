use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    rc::Rc,
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pulse<'a> {
    state: PulseState,
    source: &'a str,
    destination: &'a str,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum PartKind<'a> {
    Button,
    Broadcaster,
    FlipFlop {
        on: bool,
    },
    Conjunction {
        input_state: Vec<(&'a str, PulseState)>,
    },
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Part<'a> {
    kind: PartKind<'a>,
    id: &'a str,
    destinations: Rc<[&'a str]>,
}

impl<'a, 'b> From<&'b str> for Part<'a>
where
    'b: 'a,
{
    fn from(value: &'b str) -> Part<'a> {
        let (kind_and_name, destinations) = value.split_once(" -> ").unwrap();
        let (kind, id) = match kind_and_name {
            BROADCASTER => (PartKind::Broadcaster, BROADCASTER),
            _ => match kind_and_name.split_at(1) {
                ("%", name) => (PartKind::FlipFlop { on: false }, name),
                ("&", name) => (
                    PartKind::Conjunction {
                        input_state: Vec::default(),
                    },
                    name,
                ),
                _ => panic!("Unknown part type!"),
            },
        };
        let destinations = destinations.split(", ").collect();
        Self {
            kind,
            id,
            destinations,
        }
    }
}

impl<'a> Part<'a> {
    fn process_pulse(&mut self, pulse: Pulse<'a>) -> Vec<Pulse<'a>> {
        match &mut self.kind {
            PartKind::Broadcaster => self
                .destinations
                .iter()
                .map(|d| Pulse {
                    source: self.id,
                    destination: d,
                    state: pulse.state,
                })
                .collect(),
            PartKind::FlipFlop { on } => match pulse.state {
                PulseState::High => vec![],
                PulseState::Low => {
                    *on = !*on;
                    let state = if *on {
                        PulseState::High
                    } else {
                        PulseState::Low
                    };
                    self.destinations
                        .iter()
                        .map(|d| Pulse {
                            source: self.id,
                            destination: d,
                            state,
                        })
                        .collect()
                }
            },
            PartKind::Conjunction { input_state } => {
                input_state
                    .iter_mut()
                    .find(|(name, _)| *name == pulse.source)
                    .unwrap()
                    .1 = pulse.state;
                let state = if input_state
                    .iter()
                    .all(|(_, state)| *state == PulseState::High)
                {
                    PulseState::Low
                } else {
                    PulseState::High
                };
                self.destinations
                    .iter()
                    .map(|d| Pulse {
                        source: self.id,
                        destination: d,
                        state,
                    })
                    .collect()
            }
            PartKind::Button => panic!("Button can't receive pulses!"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct State<'a> {
    parts: HashMap<&'a str, Part<'a>>,
    pulses: VecDeque<Pulse<'a>>,
}

const BROADCASTER: &str = "broadcaster";
const BUTTON: &str = "button";

impl<'a, 'b> From<&'b str> for State<'a>
where
    'b: 'a,
{
    fn from(value: &'b str) -> Self {
        let mut parts: HashMap<&'a str, Part<'a>> =
            value.lines().map(Part::from).map(|p| (p.id, p)).collect();
        parts.insert(
            BUTTON,
            Part {
                kind: PartKind::Button,
                id: BUTTON,
                destinations: vec![BROADCASTER].into(),
            },
        );
        for part_id in parts.clone().into_keys() {
            for part in parts.clone().into_values() {
                if part.destinations.contains(&part_id) {
                    if let PartKind::Conjunction { input_state } =
                        &mut parts.get_mut(&part_id).unwrap().kind
                    {
                        input_state.push((part.id, PulseState::Low));
                    }
                }
            }
        }

        Self {
            parts,
            pulses: VecDeque::default(),
        }
    }
}

impl<'a> State<'a> {
    fn process_pulses(&mut self) -> (usize, usize) {
        let mut low = 0;
        let mut high = 0;
        while let Some(pulse) = self.pulses.pop_front() {
            match pulse.state {
                PulseState::Low => low += 1,
                PulseState::High => high += 1,
            };
            if let Some(destination_part) = self.parts.get_mut(&pulse.destination) {
                self.pulses.extend(destination_part.process_pulse(pulse));
            }
        }
        (low, high)
    }

    fn process_pulses_part2(&mut self) -> bool {
        let mut rx_low_pulses: usize = 0;
        while let Some(pulse) = self.pulses.pop_front() {
            if pulse.state == PulseState::Low && pulse.destination == "rx" {
                rx_low_pulses += 1;
            }
            if let Some(destination_part) = self.parts.get_mut(&pulse.destination) {
                self.pulses.extend(destination_part.process_pulse(pulse));
            }
        }
        rx_low_pulses != 0
    }

    fn push_button(&mut self) {
        self.pulses.push_back(Pulse {
            state: PulseState::Low,
            source: BUTTON,
            destination: BROADCASTER,
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
    let mut state = State::from(s);
    let mut count: usize = 1;
    state.push_button();
    while !state.process_pulses_part2() {
        count += 1;
        if count % 1_000_000 == 0 {
            dbg!(count);
        }
        state.push_button();
    }
    count
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
