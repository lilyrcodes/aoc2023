use std::{collections::HashMap, fmt::Debug, fs::read_to_string, rc::Rc};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::Flat => '#',
            Tile::Round => 'O',
        }
    }
}

#[derive(Eq, Clone)]
struct Map {
    rows: Rc<Vec<Vec<Tile>>>,
    compressed: usize,
    compressed_cache: Vec<Rc<[Tile]>>,
    cache: HashMap<usize, (Rc<Vec<Vec<Tile>>>, usize)>,
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.compressed == other.compressed
            || self.compressed_cache[self.compressed] == other.compressed_cache[other.compressed]
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.rows.iter() {
            let s = line.iter().copied().map(char::from).collect::<String>();
            f.write_str(&s)?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let rows: Vec<Vec<Tile>> = value
            .lines()
            .map(|line| line.chars().map(Tile::from).collect::<Vec<Tile>>())
            .collect();
        let compressed_cache = vec![rows.iter().flatten().copied().collect()];
        Self {
            rows: rows.into(),
            compressed: 0,
            compressed_cache,
            cache: HashMap::default(),
        }
    }
}

impl Map {
    fn update_compression(&mut self) {
        let compressed: Rc<[Tile]> = self.rows.iter().flatten().copied().collect();
        if let Some(pos) = self.compressed_cache.iter().position(|e| e == &compressed) {
            self.compressed = pos;
        } else {
            self.compressed_cache.push(compressed);
            self.compressed = self.compressed_cache.len() - 1;
        }
    }

    fn tilt_north(&mut self) {
        let mut rows = (*self.rows).to_owned();
        for x in 0..rows[0].len() {
            for y in 0..rows.len() {
                if rows[y][x] == Tile::Round {
                    let mut new_y = y;
                    for check_y in (0..y).rev() {
                        if rows[check_y][x] == Tile::Empty {
                            new_y = check_y;
                        } else {
                            break;
                        }
                    }
                    if y != new_y {
                        rows[new_y][x] = Tile::Round;
                        rows[y][x] = Tile::Empty;
                    }
                }
            }
        }
        self.rows = Rc::from(rows);
        self.update_compression();
    }

    fn tilt_south(&mut self) {
        let mut rows = (*self.rows).to_owned();
        for x in 0..rows[0].len() {
            for y in (0..rows.len() - 1).rev() {
                if rows[y][x] == Tile::Round {
                    let mut new_y = y;
                    for check_y in y + 1..rows.len() {
                        if rows[check_y][x] == Tile::Empty {
                            new_y = check_y;
                        } else {
                            break;
                        }
                    }
                    if y != new_y {
                        rows[new_y][x] = Tile::Round;
                        rows[y][x] = Tile::Empty;
                    }
                }
            }
        }
        self.rows = Rc::from(rows);
        self.update_compression();
    }

    fn tilt_west(&mut self) {
        let mut rows = (*self.rows).to_owned();
        for y in 0..rows.len() {
            for x in 0..rows[0].len() {
                if rows[y][x] == Tile::Round {
                    let mut new_x = x;
                    for check_x in (0..x).rev() {
                        if rows[y][check_x] == Tile::Empty {
                            new_x = check_x;
                        } else {
                            break;
                        }
                    }
                    if x != new_x {
                        rows[y][new_x] = Tile::Round;
                        rows[y][x] = Tile::Empty;
                    }
                }
            }
        }
        self.rows = Rc::from(rows);
        self.update_compression();
    }

    fn tilt_east(&mut self) {
        let mut rows = (*self.rows).to_owned();
        for y in 0..rows.len() {
            for x in (0..rows[0].len() - 1).rev() {
                if rows[y][x] == Tile::Round {
                    let mut new_x = x;
                    for check_x in x + 1..rows[0].len() {
                        if rows[y][check_x] == Tile::Empty {
                            new_x = check_x;
                        } else {
                            break;
                        }
                    }
                    if x != new_x {
                        rows[y][new_x] = Tile::Round;
                        rows[y][x] = Tile::Empty;
                    }
                }
            }
        }
        self.rows = Rc::from(rows);
        self.update_compression();
    }

    fn rotate(&mut self) {
        if let Some(cached_row) = self.cache.get(&self.compressed) {
            self.rows = cached_row.0.clone();
            self.compressed = cached_row.1;
            return;
        }
        let old = self.compressed;

        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();

        self.cache.insert(old, (self.rows.clone(), self.compressed));
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
    for _ in 0..1_000_000_000 {
        map.rotate();
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

    const TILTED_EAST: &str = "....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#....";

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
    fn test_tilt_east() {
        let mut map = Map::from(TEST_INPUT);
        map.tilt_east();
        let expected = Map::from(TILTED_EAST);
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
