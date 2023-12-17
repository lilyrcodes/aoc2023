use std::fs::read_to_string;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
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

impl Map {
    fn tilt_north(&mut self) {
        for x in 0..self.rows[0].len() {
            for y in 0..self.rows.len() {
                if self.rows[y][x] == Tile::Round {
                    let mut new_y = y;
                    for check_y in (0..y).rev() {
                        if self.rows[check_y][x] == Tile::Empty {
                            new_y = check_y;
                        } else {
                            break;
                        }
                    }
                    if y != new_y {
                        self.rows[new_y][x] = Tile::Round;
                        self.rows[y][x] = Tile::Empty;
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for x in 0..self.rows[0].len() {
            for y in (0..self.rows.len() - 1).rev() {
                if self.rows[y][x] == Tile::Round {
                    let mut new_y = y;
                    for check_y in y..self.rows.len() {
                        if self.rows[check_y][x] == Tile::Empty {
                            new_y = check_y;
                        } else {
                            break;
                        }
                    }
                    if y != new_y {
                        self.rows[new_y][x] = Tile::Round;
                        self.rows[y][x] = Tile::Empty;
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for y in 0..self.rows.len() {
            for x in 0..self.rows[0].len() {
                if self.rows[y][x] == Tile::Round {
                    let mut new_x = x;
                    for check_x in (0..x).rev() {
                        if self.rows[y][check_x] == Tile::Empty {
                            new_x = check_x;
                        } else {
                            break;
                        }
                    }
                    if x != new_x {
                        self.rows[y][new_x] = Tile::Round;
                        self.rows[y][x] = Tile::Empty;
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.rows.len() {
            for x in (0..self.rows[0].len()).rev() {
                if self.rows[y][x] != Tile::Round && x < self.rows[0].len() - 1 {
                    let mut new_x = x;
                    for check_x in x..self.rows[0].len() {
                        if self.rows[y][check_x] == Tile::Empty {
                            new_x = check_x;
                        } else {
                            break;
                        }
                    }
                    if x != new_x {
                        self.rows[y][new_x] = Tile::Round;
                        self.rows[y][x] = Tile::Empty;
                    }
                }
            }
        }
    }

    fn rotate(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn compute_load(&self) -> usize {
        self.rows
            .iter()
            .rev()
            .enumerate()
            .map(|(y, line)| (y + 1) * line.iter().filter(|t| **t == Tile::Round).count())
            .sum()
    }
}

fn part1(s: &str) -> usize {
    let mut map = Map::from(s);
    map.tilt_north();
    map.compute_load()
}

fn part2(s: &str) -> usize {
    let mut map = Map::from(s);
    for _ in 0..1_000_000 {
        let before = map.clone();
        for _ in 0..1_000 {
            map.rotate();
        }
        if map == before {
            break;
        }
    }
    map.compute_load()
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

    const TILTED_NORTH: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    const TILTED_WEST: &str = "O....#....
OOO.#....#
.....##...
OO.#OO....
OO......#.
O.#O...#.#
O....#OO..
O.........
#....###..
#OO..#....";

    const TILTED_SOUTH: &str = ".....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
OO....OO..
#OO..###..
#OO.O#...O";

    const ROTATED_ONCE: &str = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 136);
    }

    #[test]
    fn test_tilt_north() {
        let mut map = Map::from(TEST_INPUT);
        map.tilt_north();
        let expected = Map::from(TILTED_NORTH);
        assert_eq!(expected, map);
    }

    #[test]
    fn test_tilt_west() {
        let mut map = Map::from(TEST_INPUT);
        map.tilt_west();
        let expected = Map::from(TILTED_WEST);
        assert_eq!(expected, map);
    }

    #[test]
    fn test_tilt_south() {
        let mut map = Map::from(TEST_INPUT);
        map.tilt_south();
        let expected = Map::from(TILTED_SOUTH);
        assert_eq!(expected, map);
    }

    #[test]
    fn test_part2() {
        let mut map = Map::from(TEST_INPUT);
        map.rotate();
        let expected = Map::from(ROTATED_ONCE);
        assert_eq!(expected, map);
        assert_eq!(part2(TEST_INPUT), 64);
    }
}
