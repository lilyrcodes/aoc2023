use std::fs::read_to_string;

#[derive(Default, Debug)]
struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<&str> for Pull {
    fn from(value: &str) -> Self {
        let mut pull = Self::default();
        for sub in value.split(", ") {
            let (num, color) = sub.split_once(' ').unwrap();
            let num: u32 = num.parse().unwrap();
            match color {
                "red" => pull.red += num,
                "blue" => pull.blue += num,
                "green" => pull.green += num,
                &_ => panic!("uh oh"),
            }
        }
        pull
    }
}

impl Pull {
    pub fn is_possible_with(&self, red: u32, green: u32, blue: u32) -> bool {
        red >= self.red && green >= self.green && blue >= self.blue
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            red: u32::max(self.red, other.red),
            green: u32::max(self.green, other.green),
            blue: u32::max(self.blue, other.blue),
        }
    }

    pub fn power(&self) -> u64 {
        self.red as u64 * self.green as u64 * self.blue as u64
    }
}

#[derive(Default, Debug)]
struct Game {
    id: u32,
    pulls: Vec<Pull>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let mut game = Self::default();
        let (game_str, pulls_str) = value.split_once(": ").unwrap();
        game.id = game_str.split_once(' ').unwrap().1.parse().unwrap();
        for pull_str in pulls_str.split("; ") {
            game.pulls.push(Pull::from(pull_str));
        }
        game
    }
}

impl Game {
    pub fn is_possible_with(&self, red: u32, green: u32, blue: u32) -> bool {
        self.pulls
            .iter()
            .all(|pull| pull.is_possible_with(red, green, blue))
    }

    pub fn min_pull(&self) -> Pull {
        self.pulls
            .iter()
            .fold(Pull::default(), |acc: Pull, e| acc.max(e))
    }
}

fn part1(input: &str) -> u64 {
    let games: Vec<Game> = input.lines().map(Game::from).collect();
    games
        .into_iter()
        .filter(|game| game.is_possible_with(12, 13, 14))
        .map(|game| game.id as u64)
        .sum()
}

fn part2(input: &str) -> u64 {
    let games: Vec<Game> = input.lines().map(Game::from).collect();
    games
        .iter()
        .map(Game::min_pull)
        .map(|pull| pull.power())
        .sum()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let basic_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = part1(basic_input);
        assert_eq!(output, 8);
    }

    #[test]
    fn test_part2() {
        let basic_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = part2(basic_input);
        assert_eq!(output, 2286);
    }
}
