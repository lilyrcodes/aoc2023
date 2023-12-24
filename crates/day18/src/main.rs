use std::{fmt::Debug, fs::read_to_string};

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

fn get_max_width(instructions: &[Instruction]) -> usize {
    instructions
        .iter()
        .filter_map(|i| {
            if i.direction == Direction::Right || i.direction == Direction::Left {
                Some(i.steps)
            } else {
                None
            }
        })
        .sum::<usize>()
        + 3
}

fn get_max_height(instructions: &[Instruction]) -> usize {
    instructions
        .iter()
        .filter_map(|i| {
            if i.direction == Direction::Down || i.direction == Direction::Up {
                Some(i.steps)
            } else {
                None
            }
        })
        .sum::<usize>()
        + 3
}

fn create_grid(instructions: &[Instruction]) -> Vec<Vec<bool>> {
    let width = get_max_width(instructions);
    let height = get_max_height(instructions);
    (0..height).map(|_| vec![false; width]).collect()
}

fn apply_instruction(
    start_x: usize,
    start_y: usize,
    grid: &mut [Vec<bool>],
    instruction: &Instruction,
) -> (usize, usize) {
    let (end_x, end_y) = match instruction.direction {
        Direction::Up => (start_x, start_y - instruction.steps),
        Direction::Down => (start_x, start_y + instruction.steps),
        Direction::Left => (start_x - instruction.steps, start_y),
        Direction::Right => (start_x + instruction.steps, start_y),
    };
    let small_y = usize::min(start_y, end_y);
    let big_y = usize::max(start_y, end_y);
    let small_x = usize::min(start_x, end_x);
    let big_x = usize::max(start_x, end_x);
    for line in grid.iter_mut().take(big_y + 1).skip(small_y) {
        for item in line.iter_mut().take(big_x + 1).skip(small_x) {
            *item = true;
        }
    }
    (end_x, end_y)
}

fn follow_instructions(grid: &mut [Vec<bool>], instructions: &[Instruction]) {
    let mut x = grid[0].len() / 2;
    let mut y = grid.len() / 2;
    for instruction in instructions {
        (x, y) = apply_instruction(x, y, grid, instruction);
    }
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
    let mut grid = create_grid(&instructions);
    follow_instructions(&mut grid, &instructions);
    fill_in(&grid)
}

fn part2(s: &str) -> usize {
    let instructions = parse_color_instructions(s);
    let mut grid = create_grid(&instructions);
    follow_instructions(&mut grid, &instructions);
    fill_in(&grid)
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
