use std::{
    cmp::{max, min},
    collections::HashMap,
    fs::read_to_string,
};

struct Pair<'a> {
    left: &'a str,
    right: &'a str,
}

struct Input<'a> {
    instructions: Vec<bool>,
    map: HashMap<&'a str, Pair<'a>>,
}

impl<'a> From<&'a str> for Input<'a> {
    fn from(value: &'a str) -> Self {
        let mut iter = value.lines();
        let instructions = iter.next().unwrap().chars().map(|c| c == 'L').collect();
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
        let mut step_iter = self.instructions.clone().into_iter().cycle();
        while current != "ZZZ" {
            let cur_char = step_iter.next().unwrap();
            let cur_pair = self.map.get(current).unwrap();
            current = if cur_char {
                cur_pair.left
            } else {
                cur_pair.right
            };
            steps += 1;
        }
        steps
    }

    pub fn get_cycle_length(&self, start: &str) -> usize {
        let mut steps = 0;
        let mut current = start;
        let mut step_iter = self.instructions.clone().into_iter().cycle();
        while !current.ends_with('Z') {
            let cur_char = step_iter.next().unwrap();
            let cur_pair = self.map.get(current).unwrap();
            current = if cur_char {
                cur_pair.left
            } else {
                cur_pair.right
            };
            steps += 1;
        }
        steps
    }
}

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm_all(input: &[usize]) -> usize {
    if input.len() == 1 {
        return input[0];
    }
    let a = input[0];
    let b = lcm_all(&input[1..]);
    a * b / gcd(a, b)
}

fn part1(s: &str) -> usize {
    Input::from(s).follow_directions()
}

fn part2(s: &str) -> usize {
    let input = Input::from(s);
    let lengths: Vec<usize> = input
        .map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| input.get_cycle_length(k))
        .collect();
    lcm_all(&lengths)
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

AAA = (AAB, XXX)
AAB = (XXX, AAZ)
AAZ = (AAB, XXX)
BBA = (BBB, XXX)
BBB = (BBC, BBC)
BBC = (BBZ, BBZ)
BBZ = (BBB, BBB)
XXX = (XXX, XXX)",
        );
        assert_eq!(actual, 6);
    }
}
