use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn char_to_directions(c: char) -> Vec<Direction> {
    match c {
        'S' => vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ],
        '|' => vec![Direction::Up, Direction::Down],
        '-' => vec![Direction::Left, Direction::Right],
        'L' => vec![Direction::Up, Direction::Right],
        'J' => vec![Direction::Up, Direction::Left],
        '7' => vec![Direction::Down, Direction::Left],
        'F' => vec![Direction::Down, Direction::Right],
        _ => vec![],
    }
}

fn read_from_string(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|s| s.chars().collect()).collect()
}

fn get_size(s: &str) -> (usize, usize) {
    (s.lines().next().unwrap().len(), s.lines().count())
}

fn get_start_pos(tiles: &[Vec<char>]) -> (usize, usize) {
    for (y, line) in tiles.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch == 'S' {
                return (x, y);
            }
        }
    }
    panic!()
}

fn part1(s: &str) -> usize {
    let (width, height) = get_size(s);
    let map = read_from_string(s);
    let (x, y) = get_start_pos(&map);
    let mut distance_map: Vec<Vec<usize>> = vec![vec![0; width]; height];
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    let mut explored: HashSet<(usize, usize)> = HashSet::new();
    queue.push_back(((x, y), 0));
    while let Some(((x, y), dist)) = queue.pop_front() {
        if explored.contains(&(x, y)) {
            continue;
        }
        distance_map[y][x] = dist;
        explored.insert((x, y));
        for d in char_to_directions(map[y][x]) {
            match d {
                Direction::Up => {
                    if y > 0 {
                        queue.push_back(((x, y - 1), dist + 1));
                    }
                }
                Direction::Down => {
                    if y < height - 1 {
                        queue.push_back(((x, y + 1), dist + 1));
                    }
                }
                Direction::Left => {
                    if x > 0 {
                        queue.push_back(((x - 1, y), dist + 1));
                    }
                }
                Direction::Right => {
                    if x < width - 1 {
                        queue.push_back(((x + 1, y), dist + 1));
                    }
                }
            }
        }
    }
    distance_map.into_iter().flatten().max().unwrap()
}

fn part2(s: &str) -> i64 {
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

    const TEST_INPUT_1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
    const TEST_INPUT_2: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    const TEST_INPUT_3: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    const TEST_INPUT_4: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 4);
        assert_eq!(part1(TEST_INPUT_2), 4);
        assert_eq!(part1(TEST_INPUT_3), 8);
        assert_eq!(part1(TEST_INPUT_4), 8);
    }

    /*
    #[test]
    fn test_part2() {
        assert_eq!(TEST_INPUT_2, 2);
    }
    */
}
