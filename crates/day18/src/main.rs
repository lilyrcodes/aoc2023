use std::{fmt::Debug, fs::read_to_string, ops::Add};

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

    pub fn contains_point(&self, x: i64, y: i64) -> bool {
        self.contains_x(x) && self.contains_y(y)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum EntranceShape {
    Vert,
    Down,
    Up,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        (self.start <= other.start && other.start <= self.end)
            || (self.start <= other.end && other.end <= self.end)
            || (other.start <= self.start && self.start <= other.end)
            || (other.start <= self.end && self.end <= other.end)
    }
}

fn collapse_ranges(ranges: &mut [Range]) -> Vec<Range> {
    if ranges.is_empty() {
        return vec![];
    }
    let mut result = Vec::with_capacity(ranges.len());
    ranges.sort();
    let mut prev = ranges[0];
    for range in ranges.iter().skip(1) {
        if prev.overlaps(range) {
            prev = Range {
                start: i64::min(range.start, prev.start),
                end: i64::max(range.end, prev.end),
            };
        } else {
            result.push(prev);
            prev = *range;
        }
    }
    result.push(prev);
    result
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

fn get_ranges_for_y(lines: &[Line], y: i64) -> [Vec<Range>; 3] {
    let mut generated_ranges = [Vec::new(), Vec::new(), Vec::new()];
    for (idx, row) in [y - 1, y, y + 1].into_iter().enumerate() {
        for line in lines.iter().filter(|line| line.contains_y(row)) {
            generated_ranges[idx].push(Range::new(
                i64::min(line.start_x, line.end_x),
                i64::max(line.start_x, line.end_x),
            ));
        }
        generated_ranges[idx] = collapse_ranges(&mut generated_ranges[idx]);
    }
    generated_ranges
}

fn get_shape_from_lines(lines: &[Line], x: i64, y: i64) -> Option<EntranceShape> {
    if !lines.iter().any(|line| line.contains_point(x, y)) {
        None
    } else if lines.iter().any(|line| line.contains_point(x, y - 1)) {
        if lines.iter().any(|line| line.contains_point(x, y + 1)) {
            Some(EntranceShape::Vert)
        } else {
            Some(EntranceShape::Up)
        }
    } else if lines.iter().any(|line| line.contains_point(x, y + 1)) {
        Some(EntranceShape::Down)
    } else {
        None
    }
}

fn fill_in_ranges(lines: &[Line], ranges: &[Vec<Range>], y: i64) -> usize {
    let mut filled_in: usize = 0;
    let mut in_shape = false;
    let mut prev_range: Option<Range> = None;
    let lines = lines
        .iter()
        .filter(|line| line.contains_y(y))
        .copied()
        .collect::<Vec<Line>>();
    for range in ranges[1].iter() {
        if let Some(prev_range) = prev_range {
            if in_shape {
                filled_in += (range.start - prev_range.end - 1) as usize;
            }
        }
        let entrance_shape = get_shape_from_lines(&lines, range.start, y);
        match entrance_shape {
            Some(EntranceShape::Down) => match get_shape_from_lines(&lines, range.end, y) {
                Some(EntranceShape::Up) => {
                    in_shape = !in_shape;
                }
                Some(EntranceShape::Down) => {}
                None => {}
                Some(EntranceShape::Vert) => {
                    panic!("Should not get vert exit shape: ({}, {})", range.end, y)
                }
            },
            Some(EntranceShape::Up) => match get_shape_from_lines(&lines, range.end, y) {
                Some(EntranceShape::Down) => {
                    in_shape = !in_shape;
                }
                Some(EntranceShape::Up) => {}
                None => {}
                Some(EntranceShape::Vert) => {
                    panic!("Should not get vert exit shape: ({}, {})", range.end, y)
                }
            },
            None => {
                if let Some(EntranceShape::Vert) = get_shape_from_lines(&lines, range.end, y) {
                    in_shape = !in_shape;
                }
            }
            Some(EntranceShape::Vert) => in_shape = !in_shape,
        }
        filled_in += (range.end - range.start + 1) as usize;
        prev_range = Some(*range);
    }
    filled_in
}

fn part1(s: &str) -> usize {
    let instructions = parse_instructions(s);
    let lines = convert_to_lines(&instructions);
    let bounds = get_bounds(&lines);
    let mut total = 0;
    for y in bounds.start_y..=bounds.end_y {
        let ranges = get_ranges_for_y(&lines, y);
        total += fill_in_ranges(&lines, &ranges, y);
    }
    total
}

fn part2(s: &str) -> usize {
    let instructions = parse_color_instructions(s);
    let lines = convert_to_lines(&instructions);
    let bounds = get_bounds(&lines);
    let mut total = 0;
    for y in bounds.start_y..=bounds.end_y {
        let ranges = get_ranges_for_y(&lines, y);
        total += fill_in_ranges(&lines, &ranges, y);
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
