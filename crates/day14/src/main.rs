use std::fs::read_to_string;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Flat,
    Round,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Flat,
            'O' => Self::Round,
            _ => panic!("Unknown tile type"),
        }
    }
}

struct Map {
    rows: Vec<Vec<Tile>>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let rows = value
            .lines()
            .map(|line| line.chars().map(Tile::from).collect::<Vec<Tile>>())
            .collect();
        Self { rows }
    }
}

fn part1(s: &str) -> usize {
    todo!()
}

fn part2(s: &str) -> usize {
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

    const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    const ROLLED_NORTH: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 400);
    }
}
