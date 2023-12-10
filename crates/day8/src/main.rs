use petgraph::prelude::*;
use std::{collections::HashMap, fs::read_to_string};

struct Pair<'a> {
    left: &'a str,
    right: &'a str,
}

struct Input<'a> {
    instructions: &'a str,
    map: HashMap<&'a str, Pair<'a>>,
}

impl<'a> From<&'a str> for Input<'a> {
    fn from(value: &'a str) -> Self {
        let mut iter = value.lines();
        let instructions = iter.next().unwrap();
        let mut map = HashMap::new();
        iter.next();
        for line in iter {
            let (from, to) = line.split_once(" = (").unwrap();
            let (left, right) = to.split_once(", ").unwrap();
            let right = right.strip_suffix(')').unwrap();
            map.insert(from, Pair { left, right });
        }
        Self { instructions, map }
    }
}

impl<'a> Input<'a> {
    pub fn follow_directions(&self) -> usize {
        let mut steps: usize = 0;
        let mut current = "AAA";
        let mut step_iter = self.instructions.chars().cycle();
        while current != "ZZZ" {
            let cur_char = step_iter.next().unwrap();
            let cur_pair = self.map.get(current).unwrap();
            current = if cur_char == 'L' {
                cur_pair.left
            } else {
                cur_pair.right
            };
            steps += 1;
        }
        steps
    }

    pub fn follow_simul_directions(&self) -> usize {
        let mut steps: usize = 0;
        let mut all_current: Vec<&str> = self
            .map
            .keys()
            .filter(|k| k.ends_with('A'))
            .copied()
            .collect();
        let mut step_iter = self.instructions.chars().cycle();
        while !all_current.iter().all(|k| k.ends_with('Z')) {
            let cur_char = step_iter.next().unwrap();
            let cur_pairs = all_current.iter().map(|k| self.map.get(*k).unwrap());
            all_current = if cur_char == 'L' {
                cur_pairs.map(|p| p.left).collect()
            } else {
                cur_pairs.map(|p| p.right).collect()
            };
            steps += 1;
        }
        steps
    }
}

fn part1(s: &str) -> usize {
    Input::from(s).follow_directions()
}

fn part2(s: &str) -> usize {
    Input::from(s).follow_simul_directions()
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

    const TEST_INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part1() {
        let actual = part1(TEST_INPUT_1);
        assert_eq!(actual, 2);
        let actual = part1(TEST_INPUT_2);
        assert_eq!(actual, 6);
    }

    #[test]
    fn test_part2() {
        let actual = part2(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(actual, 6);
    }
}
