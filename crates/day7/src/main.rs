use std::{collections::HashMap, fs::read_to_string};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            _ => Self::Two,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl From<&[Card; 5]> for HandType {
    fn from(value: &[Card; 5]) -> Self {
        let mut counter: HashMap<&Card, u8> = HashMap::default();
        for card in value {
            counter.insert(card, counter.get(card).copied().unwrap_or_default() + 1);
        }
        match counter.values().max().unwrap() {
            5 => return Self::FiveOfKind,
            4 => return Self::FourOfKind,
            1 => return Self::HighCard,
            _ => {}
        };
        if counter.values().any(|x| *x == 3) {
            if counter.values().any(|x| *x == 2) {
                return Self::FullHouse;
            }
            return Self::ThreeOfKind;
        }
        if counter.values().filter(|x| **x == 2).count() == 2 {
            return Self::TwoPair;
        }
        Self::OnePair
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bid: u64,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut iter = value.split_whitespace();
        let hand: Vec<Card> = iter.next().unwrap().chars().map(Card::from).collect();
        let bid = iter.next().unwrap().parse::<u64>().unwrap();
        let cards = [hand[0], hand[1], hand[2], hand[3], hand[4]];
        let hand_type = HandType::from(&cards);
        Self {
            hand_type,
            cards,
            bid,
        }
    }
}

fn parse_input(s: &str) -> Vec<Hand> {
    s.lines().map(Hand::from).collect()
}

fn part1(s: &str) -> u64 {
    let mut data = parse_input(s);
    data.sort();
    data.into_iter()
        .enumerate()
        .map(|(i, data)| (i as u64 + 1) * data.bid)
        .sum()
}

fn part2(s: &str) -> u64 {
    let data = parse_input(s);
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

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        let actual = part1(TEST_INPUT);
        assert_eq!(actual, 6440);
    }

    #[test]
    fn test_part2() {
        let actual = part2(TEST_INPUT);
        assert_eq!(actual, 46);
    }
}
