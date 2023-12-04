use std::fs::read_to_string;

#[derive(Default, Debug)]
struct NumberCoords {
    num: u64,
    x_start: usize,
    length: usize,
    y: usize,
}

impl NumberCoords {
    fn new(num: u64, x_start: usize, length: usize, y: usize) -> Self {
        Self {
            num,
            x_start,
            length,
            y,
        }
    }

    fn from_line_and_y(line: &str, y: usize) -> Vec<Self> {
        let mut numbers: Vec<NumberCoords> = Vec::default();
        let mut digits = String::default();
        let mut cur_num_x_start: usize = 0;
        for (x, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                if digits.is_empty() {
                    cur_num_x_start = x;
                }
                digits.push(ch);
            } else if !digits.is_empty() {
                numbers.push(NumberCoords::new(
                    digits.parse().unwrap(),
                    cur_num_x_start,
                    digits.len(),
                    y,
                ));
                digits.clear();
            }
        }
        if !digits.is_empty() {
            numbers.push(NumberCoords::new(
                digits.parse().unwrap(),
                cur_num_x_start,
                digits.len(),
                y,
            ));
        }
        numbers
    }

    fn is_adjacent_to(&self, location: &Location) -> bool {
        let x_end = self.x_start + self.length;
        location.x + 1 >= self.x_start
            && location.x <= x_end
            && location.y + 1 >= self.y
            && location.y <= self.y + 1
    }
}

struct Location {
    x: usize,
    y: usize,
}

fn get_numbers(s: &str) -> Vec<NumberCoords> {
    s.lines()
        .enumerate()
        .flat_map(|(y, line)| NumberCoords::from_line_and_y(line, y).into_iter())
        .collect()
}

fn part1(s: &str) -> u64 {
    let numbers: Vec<NumberCoords> = get_numbers(s);
    let marker_locations: Vec<Location> = s
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, ch)| {
                if ch.is_ascii_digit() || ch == '.' {
                    None
                } else {
                    Some(Location { x, y })
                }
            })
        })
        .collect();
    numbers
        .into_iter()
        .filter(|coord| marker_locations.iter().any(|loc| coord.is_adjacent_to(loc)))
        .map(|coord| coord.num)
        .sum()
}

fn part2(s: &str) -> u64 {
    let numbers: Vec<NumberCoords> = get_numbers(s);
    let marker_locations: Vec<Location> = s
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, ch)| {
                if ch == '*' {
                    Some(Location { x, y })
                } else {
                    None
                }
            })
        })
        .collect();
    marker_locations
        .into_iter()
        .filter_map(|loc| {
            let adj = numbers
                .iter()
                .filter(|coord| coord.is_adjacent_to(&loc))
                .collect::<Vec<&NumberCoords>>();
            if adj.len() == 2 {
                Some(adj[0].num * adj[1].num)
            } else {
                None
            }
        })
        .sum()
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

    #[test]
    fn test_part1() {
        let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let expected = 4361;
        let actual = part1(test_input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let expected = 467835;
        let actual = part2(test_input);
        assert_eq!(actual, expected);
    }
}
