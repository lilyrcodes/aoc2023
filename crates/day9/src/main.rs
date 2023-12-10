use std::fs::read_to_string;

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect()
}

fn extrapolate_stack(line: Vec<i64>) -> Vec<Vec<i64>> {
    let mut stack = vec![line];
    while !stack.last().unwrap().iter().all(|num| *num == 0) {
        let line: Vec<i64> = stack
            .last()
            .unwrap()
            .iter()
            .zip(stack.last().unwrap().iter().skip(1))
            .map(|(left, right)| right - left)
            .collect();
        stack.push(line);
    }
    stack
}

fn get_next_in_line(line: &str) -> i64 {
    let stack = extrapolate_stack(parse_line(line));

    let mut num: i64 = 0;
    for line in stack.iter().rev() {
        num += line.last().unwrap();
    }

    num
}

fn get_prev_in_line(line: &str) -> i64 {
    let stack = extrapolate_stack(parse_line(line));

    let mut num: i64 = 0;
    for line in stack.iter().rev() {
        num = line.first().unwrap() - num;
    }

    num
}

fn part1(s: &str) -> i64 {
    s.lines().map(get_next_in_line).sum()
}

fn part2(s: &str) -> i64 {
    s.lines().map(get_prev_in_line).sum()
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

    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        let actual = part1(TEST_INPUT);
        assert_eq!(actual, 114);
    }

    #[test]
    fn test_part2() {
        let actual = part2(TEST_INPUT);
        assert_eq!(actual, 2);
    }
}
