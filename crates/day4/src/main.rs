use std::{fs::read_to_string, collections::HashSet};

#[derive(Debug, Default, Clone)]
struct Card {
    score: u64,
    matches: usize,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (_, rest) = value.split_once(": ").unwrap();
        let (winners, numbers) = rest.split_once(" | ").unwrap();
        let winners: HashSet<u32> = winners.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        let numbers: HashSet<u32> = numbers.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        let matches = winners.intersection(&numbers).count();
        let score = if matches == 0 {
            0
        } else {
            1 << (matches - 1)
        };
        Card { score, matches }
    }
}

fn part1(s: &str) -> u64 {
    s.lines().map(Card::from).map(|c| c.score).sum()
}

fn part2(s: &str) -> u64 {
    let cards: Vec<Card> = s.lines().map(Card::from).collect();
    let mut card_counts: Vec<usize> = cards.iter().map(|_| 1).collect();
    for (cur_card_idx, card) in cards.into_iter().enumerate() {
        let cur_count = card_counts[cur_card_idx];
        for prize_count in card_counts.iter_mut().skip(cur_card_idx + 1).take(card.matches) {
            *prize_count += cur_count;
        }
    }
    card_counts.into_iter().sum::<usize>() as u64
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

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        let actual = part1(TEST_INPUT);
        assert_eq!(actual, 13);
    }

    #[test]
    fn test_part2() {
        let actual = part2(TEST_INPUT);
        assert_eq!(actual, 30);
    }
}
