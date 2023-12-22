use std::{
    collections::{HashSet, VecDeque},
    fmt::{Debug, Write},
    fs::read_to_string,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    FMirror,
    BMirror,
    HSplitter,
    VSplitter,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Laser {
    x: usize,
    y: usize,
    direction: Direction,
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::FMirror => '/',
            Tile::BMirror => '\\',
            Tile::HSplitter => '-',
            Tile::VSplitter => '|',
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '/' => Tile::FMirror,
            '\\' => Tile::BMirror,
            '-' => Tile::HSplitter,
            '|' => Tile::VSplitter,
            _ => panic!("Unknown tile"),
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char((*self).into())
    }
}

fn parse_input(s: &str) -> Vec<Vec<Tile>> {
    s.lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect()
}

fn next_tile(width: usize, height: usize, laser: &Laser) -> Option<Laser> {
    match laser.direction {
        Direction::Up => {
            if laser.y > 0 {
                Some(Laser {
                    y: laser.y - 1,
                    ..*laser
                })
            } else {
                None
            }
        }
        Direction::Down => {
            if laser.y + 1 < height {
                Some(Laser {
                    y: laser.y + 1,
                    ..*laser
                })
            } else {
                None
            }
        }
        Direction::Left => {
            if laser.x > 0 {
                Some(Laser {
                    x: laser.x - 1,
                    ..*laser
                })
            } else {
                None
            }
        }
        Direction::Right => {
            if laser.x + 1 < width {
                Some(Laser {
                    x: laser.x + 1,
                    ..*laser
                })
            } else {
                None
            }
        }
    }
}

fn new_directions(tile: Tile, direction: Direction) -> Vec<Direction> {
    match (tile, direction) {
        (Tile::FMirror, Direction::Up) => vec![Direction::Right],
        (Tile::FMirror, Direction::Down) => vec![Direction::Left],
        (Tile::FMirror, Direction::Left) => vec![Direction::Down],
        (Tile::FMirror, Direction::Right) => vec![Direction::Up],
        (Tile::BMirror, Direction::Up) => vec![Direction::Left],
        (Tile::BMirror, Direction::Down) => vec![Direction::Right],
        (Tile::BMirror, Direction::Left) => vec![Direction::Up],
        (Tile::BMirror, Direction::Right) => vec![Direction::Down],
        (Tile::HSplitter, Direction::Up | Direction::Down) => {
            vec![Direction::Left, Direction::Right]
        }
        (Tile::VSplitter, Direction::Left | Direction::Right) => {
            vec![Direction::Up, Direction::Down]
        }
        _ => vec![direction],
    }
}

fn fire_laser(grid: &[Vec<Tile>], start_laser: Laser) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut result: Vec<Vec<bool>> = grid
        .iter()
        .map(|line| line.iter().map(|_| false).collect())
        .collect();
    let mut lasers = VecDeque::new();
    lasers.push_back(start_laser);
    let mut seen = HashSet::new();
    while let Some(laser) = lasers.pop_front() {
        result[laser.y][laser.x] = true;
        if seen.contains(&laser) {
            continue;
        }
        seen.insert(laser);
        for new_direction in new_directions(grid[laser.y][laser.x], laser.direction) {
            if let Some(laser) = next_tile(
                width,
                height,
                &Laser {
                    direction: new_direction,
                    ..laser
                },
            ) {
                lasers.push_back(laser);
            }
        }
    }
    result.into_iter().flatten().filter(|e| *e).count()
}

fn part1(s: &str) -> usize {
    let grid = parse_input(s);
    let start_laser = Laser {
        x: 0,
        y: 0,
        direction: Direction::Right,
    };
    fire_laser(&grid, start_laser)
}

fn part2(s: &str) -> usize {
    let grid = parse_input(s);
    let height = grid.len();
    let width = grid[0].len();
    let left_side = (0..height).map(|y| Laser {
        x: 0,
        y,
        direction: Direction::Right,
    });
    let right_side = (0..height).map(|y| Laser {
        x: width - 1,
        y,
        direction: Direction::Left,
    });
    let top_side = (0..width).map(|x| Laser {
        x,
        y: 0,
        direction: Direction::Down,
    });
    let bottom_side = (0..width).map(|x| Laser {
        x,
        y: height - 1,
        direction: Direction::Up,
    });
    let mut max = 0;
    for start_laser in left_side
        .chain(right_side)
        .chain(top_side)
        .chain(bottom_side)
    {
        let result = fire_laser(&grid, start_laser);
        if result > max {
            max = result;
        }
    }
    max
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

    const TEST_INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 51);
    }
}
