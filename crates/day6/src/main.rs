use std::fs::read_to_string;

fn distance_traveled(charge_time: u64, travel_time: u64) -> u64 {
    charge_time * travel_time
}

fn min_charge_time(total_time: u64, record_distance: u64) -> u64 {
    for charge_time in 1..total_time {
        if distance_traveled(charge_time, total_time - charge_time) > record_distance {
            return charge_time;
        }
    }
    panic!("Can't beat distance!")
}

fn max_charge_time(total_time: u64, record_distance: u64) -> u64 {
    for charge_time in (1..total_time).rev() {
        if distance_traveled(charge_time, total_time - charge_time) > record_distance {
            return charge_time;
        }
    }
    panic!("Can't beat distance!")
}

fn part1(s: &str) -> u64 {
    let mut lines = s.lines();
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(&str::parse::<u64>)
        .map(Result::unwrap)
        .collect();
    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(&str::parse::<u64>)
        .map(Result::unwrap)
        .collect();
    let mut margin: u64 = 1;
    for (total_time, record_distance) in times.into_iter().zip(distances.into_iter()) {
        margin *= max_charge_time(total_time, record_distance)
            - min_charge_time(total_time, record_distance)
            + 1;
    }
    margin
}

fn part2(s: &str) -> u64 {
    let mut lines = s.lines();
    let total_time: u64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .flat_map(&str::chars)
        .map(|ch| ch.to_digit(10).unwrap() as u64)
        .fold(0, |acc, item| acc * 10 + item);
    let record_distance: u64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .flat_map(&str::chars)
        .map(|ch| ch.to_digit(10).unwrap() as u64)
        .fold(0, |acc, item| acc * 10 + item);
    max_charge_time(total_time, record_distance) - min_charge_time(total_time, record_distance) + 1
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

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        let actual = part1(TEST_INPUT);
        assert_eq!(actual, 288);
    }

    #[test]
    fn test_part2() {
        let actual = part2(TEST_INPUT);
        assert_eq!(actual, 71503);
    }
}
