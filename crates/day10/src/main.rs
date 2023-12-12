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

fn add_to_explore_queue(
    queue: &mut VecDeque<((usize, usize), usize, Direction)>,
    valid_directions: &[Direction],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    dist: usize,
) {
    for d in valid_directions {
        match d {
            Direction::Up => {
                if y > 0 {
                    queue.push_back(((x, y - 1), dist + 1, Direction::Down));
                }
            }
            Direction::Down => {
                if y < height - 1 {
                    queue.push_back(((x, y + 1), dist + 1, Direction::Up));
                }
            }
            Direction::Left => {
                if x > 0 {
                    queue.push_back(((x - 1, y), dist + 1, Direction::Right));
                }
            }
            Direction::Right => {
                if x < width - 1 {
                    queue.push_back(((x + 1, y), dist + 1, Direction::Left));
                }
            }
        }
    }
}

fn part1(s: &str) -> usize {
    let (width, height) = get_size(s);
    let map = read_from_string(s);
    let mut distance_map: Vec<Vec<usize>> = vec![vec![0; width]; height];
    let mut queue: VecDeque<((usize, usize), usize, Direction)> = VecDeque::new();
    let mut explored: HashSet<(usize, usize)> = HashSet::new();
    queue.push_back((get_start_pos(&map), 0, Direction::Up));
    while let Some(((x, y), dist, incoming_dir)) = queue.pop_front() {
        if explored.contains(&(x, y)) {
            continue;
        }
        let valid_directions = char_to_directions(map[y][x]);
        if !valid_directions.contains(&incoming_dir) {
            continue;
        }
        distance_map[y][x] = dist;
        explored.insert((x, y));
        add_to_explore_queue(&mut queue, &valid_directions, x, y, width, height, dist);
    }
    distance_map.into_iter().flatten().max().unwrap()
}

fn get_start_character(map: &[Vec<char>], x: usize, y: usize) -> char {
    let has_left = x > 0 && "-FL".contains(map[y][x - 1]);
    let has_up = y > 0 && "|F7".contains(map[y - 1][x]);
    let has_down = y < map.len() - 1 && "|JL".contains(map[y + 1][x]);
    if has_up {
        if has_down {
            '|'
        } else if has_left {
            'J'
        } else {
            'L'
        }
    } else if has_down {
        if has_left {
            '7'
        } else {
            'F'
        }
    } else {
        '-'
    }
}

fn part2(s: &str) -> usize {
    let (width, height) = get_size(s);
    let map = read_from_string(s);
    let mut pipe_map: Vec<Vec<char>> = vec![vec!['.'; width]; height];
    let mut queue: VecDeque<((usize, usize), usize, Direction)> = VecDeque::new();
    let mut explored: HashSet<(usize, usize)> = HashSet::new();
    let (start_x, start_y) = get_start_pos(&map);
    queue.push_back(((start_x, start_y), 0, Direction::Up));
    while let Some(((x, y), dist, incoming_dir)) = queue.pop_front() {
        if explored.contains(&(x, y)) {
            continue;
        }
        let valid_directions = char_to_directions(map[y][x]);
        if !valid_directions.contains(&incoming_dir) {
            continue;
        }
        pipe_map[y][x] = map[y][x];
        explored.insert((x, y));
        add_to_explore_queue(&mut queue, &valid_directions, x, y, width, height, dist);
    }
    pipe_map[start_y][start_x] = get_start_character(&pipe_map, start_x, start_y);
    for line in pipe_map.iter() {
        println!("{}", line.iter().collect::<String>());
    }
    let mut tile_count = 0;
    for (y, line) in pipe_map.into_iter().enumerate() {
        let mut in_boundary = false;
        let mut stack: Vec<char> = Vec::default();
        for (x, ch) in line.into_iter().enumerate() {
            match ch {
                '|' => in_boundary = !in_boundary,
                'F' | 'L' => stack.push(ch),
                'J' => {
                    if stack.pop().unwrap() != 'L' {
                        in_boundary = !in_boundary;
                    }
                }
                '7' => {
                    if stack.pop().unwrap() != 'F' {
                        in_boundary = !in_boundary;
                    }
                }
                _ => {}
            }
            if in_boundary && ch == '.' {
                tile_count += 1;
                println!("({}, {})", x, y);
            }
        }
    }
    tile_count
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
    const TEST_INPUT_5: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    const TEST_INPUT_6: &str = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";
    const TEST_INPUT_7: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 4);
        assert_eq!(part1(TEST_INPUT_2), 4);
        assert_eq!(part1(TEST_INPUT_3), 8);
        assert_eq!(part1(TEST_INPUT_4), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 1);
        assert_eq!(part2(TEST_INPUT_2), 1);
        assert_eq!(part2(TEST_INPUT_5), 4);
        assert_eq!(part2(TEST_INPUT_6), 4);
        assert_eq!(part2(TEST_INPUT_7), 10);
    }
}
