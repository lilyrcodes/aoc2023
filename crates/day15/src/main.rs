use std::{collections::VecDeque, fs::read_to_string};

#[derive(Clone, PartialEq, Eq, Default)]
struct Lens {
    label: String,
    value: u8,
}

enum Operation {
    Insert(Lens),
    Remove(String),
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        let (label, num) = value.split_once('=').or(value.split_once('-')).unwrap();
        if value.contains('=') {
            let value = num.parse().unwrap();
            Self::Insert(Lens {
                label: String::from(label),
                value,
            })
        } else {
            Self::Remove(String::from(label))
        }
    }
}

struct HashMap {
    boxes: Vec<VecDeque<Lens>>,
}

impl Default for HashMap {
    fn default() -> Self {
        let mut result = Self {
            boxes: Vec::with_capacity(256),
        };
        for _ in 0..256 {
            result.boxes.push(VecDeque::default());
        }
        result
    }
}

impl HashMap {
    pub fn insert(&mut self, lens: Lens) {
        let h = hash(&lens.label);
        if let Some(old_lens) = self.boxes[h].iter_mut().find(|l| l.label == lens.label) {
            old_lens.value = lens.value;
        } else {
            self.boxes[h].push_back(lens);
        }
    }

    pub fn remove(&mut self, label: String) {
        let h = hash(&label);
        self.boxes[h].retain(|l| l.label != label);
    }
}

fn hash(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .copied()
        .fold(0, |acc, b| ((acc + b as usize) * 17) % 256)
}

fn part1(s: &str) -> usize {
    s.lines().next().unwrap().split(',').map(hash).sum()
}

fn part2(s: &str) -> usize {
    let mut map = HashMap::default();
    for instruction in s.lines().next().unwrap().split(',').map(Operation::from) {
        match instruction {
            Operation::Insert(lens) => map.insert(lens),
            Operation::Remove(label) => map.remove(label),
        }
    }
    let mut total = 0;
    for (box_number, bx) in map.boxes.iter().enumerate() {
        for (slot_number, lens) in bx.iter().enumerate() {
            total += (1 + box_number) * (1 + slot_number) * lens.value as usize;
        }
    }
    total
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

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 145);
    }
}
