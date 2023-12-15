use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Rock,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        if value == '#' {
            Tile::Rock
        } else {
            Tile::Empty
        }
    }
}

type Row = Vec<Tile>;

fn make_row(s: &str) -> Row {
    s.chars().map(Tile::from).collect()
}

type Map = Vec<Row>;

fn make_maps(s: &str) -> Vec<Map> {
    let mut maps = Vec::new();
    let mut map = Map::new();
    for line in s.lines() {
        if line.is_empty() {
            maps.push(map);
            map = Map::new();
        } else {
            map.push(make_row(line));
        }
    }
    if !map.is_empty() {
        maps.push(map);
    }
    maps
}

fn is_palindrome_at(r: &Row, idx: usize) -> bool {
    let (left, right) = r.split_at(idx);
    right.iter().zip(left.iter().rev()).all(|(a, b)| a == b)
}

fn find_possible_horiz_points(r: &Row) -> Vec<usize> {
    (1..r.len())
        .filter(|idx| is_palindrome_at(r, *idx))
        .collect()
}

fn find_possible_vert_points(m: &Map, idx: usize) -> Vec<usize> {
    find_possible_horiz_points(&m.iter().map(|row| row[idx]).collect::<Row>())
}

fn calc_map_points(m: Map) -> usize {
    let horiz_points = m
        .iter()
        .map(find_possible_horiz_points)
        .fold::<Vec<usize>, _>(
            (0..m.first().unwrap().len()).collect::<Vec<usize>>(),
            |acc, val| {
                acc.into_iter()
                    .filter(|num| val.contains(num))
                    .collect::<Vec<usize>>()
            },
        );
    let vert_points = (0..m.first().unwrap().len())
        .map(|idx| find_possible_vert_points(&m, idx))
        .fold::<Vec<usize>, _>((0..m.len()).collect::<Vec<usize>>(), |acc, val| {
            acc.into_iter()
                .filter(|num| val.contains(num))
                .collect::<Vec<usize>>()
        });
    if !horiz_points.is_empty() {
        horiz_points[0]
    } else {
        vert_points[0] * 100
    }
}

fn part1(s: &str) -> usize {
    make_maps(s).into_iter().map(calc_map_points).sum()
}

/*
fn part2(s: &str) -> usize {
    todo!()
}
*/

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    /*
    let answer2 = part2(&input);
    println!("Part 2: {}", answer2);
    */
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part1() {
        assert_eq!(
            find_possible_horiz_points(&make_row("#.##..##.")),
            vec![5, 7]
        );
        assert_eq!(part1(TEST_INPUT), 405);
    }

    /*
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 525152);
    }
    */
}
