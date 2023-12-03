use std::{collections::HashMap, fs::read_to_string};

fn extract_calibration_value_part2(s: &str) -> i64 {
    let lookup = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let (_, first_key) = lookup
        .keys()
        .filter_map(|c| s.find(c).map(|pos| (pos, *c)))
        .min()
        .unwrap();
    let (_, last_key) = lookup
        .keys()
        .filter_map(|c| s.rfind(c).map(|pos| (pos, *c)))
        .max()
        .unwrap();
    lookup.get(first_key).unwrap() * 10 + lookup.get(last_key).unwrap()
}

fn extract_calibration_value_part1(s: &str) -> i64 {
    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    (*digits.first().unwrap() as i64) * 10 + (*digits.last().unwrap() as i64)
}

fn sum_calibration_values_part1(input: &str) -> i64 {
    input.lines().map(extract_calibration_value_part1).sum()
}

fn sum_calibration_values_part2(input: &str) -> i64 {
    input.lines().map(extract_calibration_value_part2).sum()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let total = sum_calibration_values_part1(&input);
    println!("Part 1: {}", total);
    let total = sum_calibration_values_part2(&input);
    println!("Part 2: {}", total);
}

#[cfg(test)]
mod tests {
    use crate::{sum_calibration_values_part1, sum_calibration_values_part2};

    #[test]
    fn basic_test_part1() {
        let basic_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let sum = sum_calibration_values_part1(basic_input);
        assert_eq!(sum, 142);
    }

    #[test]
    fn basic_test_part2() {
        let basic_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let sum = sum_calibration_values_part2(basic_input);
        assert_eq!(sum, 281);
    }
}
