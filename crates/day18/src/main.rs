use std::{
    fmt::Debug,
    fs::read_to_string,
    ops::{Add, AddAssign},
};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Direction::Up => "U",
            Direction::Down => "D",
            Direction::Left => "L",
            Direction::Right => "R",
        })
    }
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unknown direction!"),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("Unknown direction!"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Instruction {
    direction: Direction,
    steps: usize,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let segments = value.split_whitespace().collect::<Vec<&str>>();
        Self {
            direction: Direction::from(segments[0]),
            steps: segments[1].parse().unwrap(),
        }
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?} {:?}", self.direction, self.steps))
    }
}

impl Instruction {
    fn from_color(value: &str) -> Self {
        let color = value.split_whitespace().nth(2).unwrap().split_at(2).1;
        let (steps, direction) = color.split_at(5);
        let steps = usize::from_str_radix(steps, 16).unwrap();
        let direction = Direction::from(direction.chars().next().unwrap());
        Self { direction, steps }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
struct Line {
    start_x: i64,
    start_y: i64,
    end_x: i64,
    end_y: i64,
}

impl Add<&Instruction> for &Line {
    type Output = Line;
    fn add(self, rhs: &Instruction) -> Self::Output {
        let start_x = self.end_x;
        let start_y = self.end_y;
        let (end_x, end_y) = match rhs.direction {
            Direction::Up => (start_x, start_y - rhs.steps as i64),
            Direction::Down => (start_x, start_y + rhs.steps as i64),
            Direction::Left => (start_x - rhs.steps as i64, start_y),
            Direction::Right => (start_x + rhs.steps as i64, start_y),
        };
        Self::Output {
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }
}

impl Line {
    pub fn contains_y(&self, y: i64) -> bool {
        (self.start_y <= y && y <= self.end_y) || (self.end_y <= y && y <= self.start_y)
    }

    pub fn contains_x(&self, x: i64) -> bool {
        (self.start_x <= x && x <= self.end_x) || (self.end_x <= x && x <= self.start_x)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum EntranceShape {
    Vert,
    Down,
    Up,
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.lines().map(Instruction::from).collect()
}

fn parse_color_instructions(s: &str) -> Vec<Instruction> {
    s.lines().map(Instruction::from_color).collect()
}

fn convert_to_lines(instructions: &[Instruction]) -> Vec<Line> {
    let mut prev_line = Line::default();
    let mut lines = Vec::with_capacity(instructions.len());
    for instruction in instructions {
        let line = &prev_line + instruction;
        lines.push(line);
        prev_line = line;
    }
    lines
}

fn get_bounds(lines: &[Line]) -> Line {
    let start_x = lines
        .iter()
        .map(|line| i64::min(line.start_x, line.end_x))
        .min()
        .unwrap();
    let start_y = lines
        .iter()
        .map(|line| i64::min(line.start_y, line.end_y))
        .min()
        .unwrap();
    let end_x = lines
        .iter()
        .map(|line| i64::max(line.start_x, line.end_x))
        .max()
        .unwrap();
    let end_y = lines
        .iter()
        .map(|line| i64::max(line.start_y, line.end_y))
        .max()
        .unwrap();
    Line {
        start_x,
        start_y,
        end_x,
        end_y,
    }
}

fn get_rows_for_y(lines: &[Line], y: i64, bounds: &Line) -> [Vec<bool>; 3] {
    let width = (bounds.start_x.abs_diff(bounds.end_x) + 3) as usize;
    let mut generated_lines = [vec![false; width], vec![false; width], vec![false; width]];
    for (idx, row) in [y - 1, y, y + 1].into_iter().enumerate() {
        for line in lines.iter().filter(|line| line.contains_y(row)) {
            for x in i64::min(line.start_x, line.end_x)..=i64::max(line.start_x, line.end_x) {
                generated_lines[idx][(x + bounds.start_x + 1) as usize] = true;
            }
        }
    }
    generated_lines
}

fn get_shape(grid: &[Vec<bool>], x: usize, y: usize) -> Option<EntranceShape> {
    if !grid[y][x] {
        None
    } else if grid[y - 1][x] {
        if grid[y + 1][x] {
            Some(EntranceShape::Vert)
        } else {
            Some(EntranceShape::Up)
        }
    } else if grid[y + 1][x] {
        Some(EntranceShape::Down)
    } else {
        None
    }
}

fn fill_in(grid: &[Vec<bool>]) -> usize {
    let mut filled_in: usize = 0;
    let height = grid.len();
    let width = grid[0].len();
    for y in 1..height - 1 {
        let mut entrance_shape: Option<EntranceShape> = None;
        let mut in_shape = false;
        for x in 1..width - 1 {
            match entrance_shape {
                Some(EntranceShape::Down) => match get_shape(grid, x, y) {
                    Some(EntranceShape::Up) => {
                        in_shape = !in_shape;
                        entrance_shape = None;
                    }
                    Some(EntranceShape::Down) => entrance_shape = None,
                    None => {}
                    Some(EntranceShape::Vert) => {
                        panic!("Should not get vert exit shape: ({}, {})", x, y)
                    }
                },
                Some(EntranceShape::Up) => match get_shape(grid, x, y) {
                    Some(EntranceShape::Down) => {
                        in_shape = !in_shape;
                        entrance_shape = None;
                    }
                    Some(EntranceShape::Up) => entrance_shape = None,
                    None => {}
                    Some(EntranceShape::Vert) => {
                        panic!("Should not get vert exit shape: ({}, {})", x, y)
                    }
                },
                None => match get_shape(grid, x, y) {
                    Some(EntranceShape::Vert) => in_shape = !in_shape,
                    shape => entrance_shape = shape,
                },
                Some(EntranceShape::Vert) => {
                    panic!("Should not get vert entrance shape: ({}, {})", x, y)
                }
            }

            if grid[y][x] || in_shape {
                filled_in += 1;
            }
        }
    }
    filled_in
}

fn part1(s: &str) -> usize {
    let instructions = parse_instructions(s);
    let lines = convert_to_lines(&instructions);
    let bounds = get_bounds(&lines);
    let mut total = 0;
    for y in bounds.start_y..=bounds.end_y {
        let rows = get_rows_for_y(&lines, y, &bounds);
        total += fill_in(&rows);
    }
    total
}

fn part2(s: &str) -> usize {
    let instructions = parse_color_instructions(s);
    let lines = convert_to_lines(&instructions);
    let bounds = get_bounds(&lines);
    let mut total = 0;
    let num_lines = bounds.end_y.abs_diff(bounds.start_y) + 1;
    for (num, y) in (bounds.start_y..=bounds.end_y).enumerate() {
        if num % 1000 == 999 {
            println!("{} / {}", num + 1, num_lines);
        }
        let rows = get_rows_for_y(&lines, y, &bounds);
        total += fill_in(&rows);
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

    const TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 62);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 952408144115);
    }
}
