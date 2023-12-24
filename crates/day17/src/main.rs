use priority_queue::PriorityQueue;
use std::{cmp::Reverse, collections::HashSet, fs::read_to_string, hash::Hash};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Clone, Copy)]
struct Move {
    distance: u32,
    x: usize,
    y: usize,
    steps: u8,
    direction: Direction,
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.steps == other.steps
            && self.direction == other.direction
    }
}

impl Eq for Move {}

impl Hash for Move {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.x);
        state.write_usize(self.y);
        state.write_u8(self.steps);
        self.direction.hash(state);
    }
}

impl Move {
    pub fn start() -> Self {
        Self {
            distance: 0,
            x: 0,
            y: 0,
            steps: 0,
            direction: Direction::Right,
        }
    }

    pub fn can_move(&self, direction: Direction, is_part_2: bool) -> bool {
        if self.direction.opposite() == direction {
            return false;
        }
        if self.direction == direction {
            if is_part_2 {
                return self.steps < 10;
            } else {
                return self.steps < 3;
            }
        }
        if is_part_2 {
            self.steps >= 4
        } else {
            true
        }
    }

    pub fn in_bounds(&self, width: usize, height: usize, direction: Direction) -> bool {
        match direction {
            Direction::Left => self.x > 0,
            Direction::Right => self.x < width - 1,
            Direction::Up => self.y > 0,
            Direction::Down => self.y < height - 1,
        }
    }

    pub fn apply_move(&self, grid: &[Vec<u32>], direction: Direction) -> Self {
        let x = match direction {
            Direction::Left => self.x - 1,
            Direction::Right => self.x + 1,
            _ => self.x,
        };
        let y = match direction {
            Direction::Up => self.y - 1,
            Direction::Down => self.y + 1,
            _ => self.y,
        };
        let steps = if self.direction == direction {
            self.steps + 1
        } else {
            1
        };
        let distance = self.distance + grid[y][x];
        Self {
            distance,
            x,
            y,
            steps,
            direction,
        }
    }
}

fn parse_input(s: &str) -> Vec<Vec<u32>> {
    s.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn initialize_queue() -> PriorityQueue<Move, Reverse<u32>> {
    let mut queue: PriorityQueue<Move, Reverse<u32>> = PriorityQueue::new();
    queue.push(Move::start(), Reverse(0));
    queue
}

fn initialize_prevs(grid: &[Vec<u32>]) -> Vec<Vec<Option<(usize, usize)>>> {
    grid.iter().map(|line| vec![None; line.len()]).collect()
}

fn get_neighbors(grid: &[Vec<u32>], cur_move: &Move, is_part_2: bool) -> Vec<Move> {
    let height = grid.len();
    let width = grid[0].len();
    let mut result = Vec::new();
    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        if cur_move.can_move(direction, is_part_2) && cur_move.in_bounds(width, height, direction) {
            result.push(cur_move.apply_move(grid, direction));
        }
    }
    result
}

fn find_path(grid: &[Vec<u32>], is_part_2: bool) -> u32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut queue = initialize_queue();
    let mut prevs = initialize_prevs(grid);
    let mut seen: HashSet<Move> = HashSet::new();
    while let Some((cur_move, _)) = queue.pop() {
        for next_move in get_neighbors(grid, &cur_move, is_part_2) {
            if next_move.x == width - 1 && next_move.y == height - 1 {
                return next_move.distance;
            }
            if let Some(old_move) = seen.get(&next_move) {
                if next_move.distance < old_move.distance {
                    prevs[next_move.y][next_move.x] = Some((cur_move.y, cur_move.x));
                    queue.remove(&next_move);
                    queue.push(next_move, Reverse(next_move.distance));
                }
            } else {
                prevs[next_move.y][next_move.x] = Some((cur_move.y, cur_move.x));
                queue.remove(&next_move);
                queue.push(next_move, Reverse(next_move.distance));
            }
            seen.insert(next_move);
        }
    }
    panic!("No path found!");
}

fn part1(s: &str) -> u32 {
    find_path(&parse_input(s), false)
}

fn part2(s: &str) -> u32 {
    find_path(&parse_input(s), true)
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

    const TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 94);
    }
}
