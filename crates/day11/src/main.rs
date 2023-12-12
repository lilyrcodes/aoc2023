use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn distance_to(&self, other: &Self) -> usize {
        self.y.abs_diff(other.y) + self.x.abs_diff(other.x)
    }
}

fn transpose_map(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = map.first().unwrap().len();
    let height = map.len();
    let mut new_map = vec![vec!['.'; height]; width];
    for (y, line) in map.into_iter().enumerate() {
        for (x, ch) in line.into_iter().enumerate() {
            new_map[x][y] = ch;
        }
    }
    new_map
}

fn expand_map_vertical(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    map.into_iter()
        .flat_map(|line| {
            if line.iter().all(|c| *c == '.') {
                vec![line.clone(), line].into_iter()
            } else {
                vec![line].into_iter()
            }
        })
        .collect()
}

fn expand_map(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    transpose_map(expand_map_vertical(transpose_map(expand_map_vertical(map))))
}

fn get_points(map: &[Vec<char>]) -> Vec<Point> {
    map.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, ch)| {
                if *ch == '#' {
                    Some(Point { x, y })
                } else {
                    None
                }
            })
        })
        .collect()
}

fn part1(s: &str) -> usize {
    let map = s
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let map = expand_map(map);
    let points = get_points(&map);

    points
        .iter()
        .enumerate()
        .flat_map(|(skip, point1)| {
            points
                .iter()
                .skip(skip)
                .map(|point2| point1.distance_to(point2))
        })
        .sum()
}

fn part2(s: &str, expand_factor: usize) -> usize {
    let map: Vec<Vec<char>> = s
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let empty_y: Vec<usize> = map
        .iter()
        .enumerate()
        .filter_map(|(y, line)| {
            if line.iter().all(|c| *c == '.') {
                Some(y)
            } else {
                None
            }
        })
        .collect();
    let mut empty_x: Vec<usize> = Vec::default();
    for x in 0..map.first().unwrap().len() {
        let mut all_empty = true;
        for (y, _) in map.iter().enumerate() {
            if map[y][x] != '.' {
                all_empty = false;
                break;
            }
        }
        if all_empty {
            empty_x.push(x);
        }
    }
    let points = get_points(&map);
    points
        .iter()
        .enumerate()
        .flat_map(|(skip, point1)| {
            points.iter().skip(skip).map(|point2| {
                point1.distance_to(point2)
                    + empty_x
                        .iter()
                        .filter(|x_line| {
                            point1.x.min(point2.x) < **x_line && **x_line < point1.x.max(point2.x)
                        })
                        .count()
                        * (expand_factor - 1)
                    + empty_y
                        .iter()
                        .filter(|y_line| {
                            point1.y.min(point2.y) < **y_line && **y_line < point1.y.max(point2.y)
                        })
                        .count()
                        * (expand_factor - 1)
            })
        })
        .sum()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&input, 1_000_000);
    println!("Part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT, 100), 8410);
    }
}
