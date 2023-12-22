use std::fs::read_to_string;

fn parse_input(s: &str) -> Vec<Vec<u32>> {
    s.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
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

    /*
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 51);
    }
    */
}
