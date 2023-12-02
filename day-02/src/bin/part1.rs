use std::str::FromStr;


fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}


fn part1(input: &str) -> String {
    let bag = CubeSet{red: 12, green: 13, blue: 14};
    let mut total = 0u32;
    for line in input.lines() {
        if let Ok(game) = Game::from_str(line) {
            total += game.game_result(&bag);
        }
    }
    total.to_string()
}


#[derive(Debug)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}


impl CubeSet {
    fn possible_subset(&self, other: &CubeSet) -> bool {
        other.red >= self.red &&
        other.green >= self.green &&
        other.blue >= self.blue
    }
}


#[derive(Debug, PartialEq, Eq)]
struct CubeSetError;


impl FromStr for CubeSet {
    type Err = CubeSetError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut last_seen_int = 0;
        let parts = s.split(' ');
        for part in parts {
            if let Ok(i) = part.parse::<u32>() {
                last_seen_int = i;
            } else {
                if part.starts_with("red") {
                    red = last_seen_int;
                } else if part.starts_with("green") {
                    green = last_seen_int;
                } else if part.starts_with("blue") {
                    blue = last_seen_int;
                }
            }
        }
        Ok(CubeSet { red: red, green: green, blue: blue })
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    selections: Vec<CubeSet>,    
}


#[derive(Debug, PartialEq, Eq)]
struct GameError;


impl FromStr for Game {
    type Err = GameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut id = 0;
        let parts = s.split(':');
        let mut selections: Vec<CubeSet> = vec![];
        for part in parts {
            if part.starts_with("Game ") {
                let game_parts: Vec<&str> = part.split(' ').collect();
                id = game_parts.last().unwrap().parse::<u32>().unwrap();
            } else {
                let selection_strings = part.split(';');
                for selection_string in selection_strings {
                    if let Ok(selection_result) = CubeSet::from_str(selection_string) {
                        selections.push(selection_result);
                    }
                }
            }
        }
        Ok(Game{id: id, selections: selections})
    }
}


impl Game {
    fn game_result(&self, bag: &CubeSet) -> u32 {
        let mut result = self.id;
        for selection in self.selections.iter() {
            if !selection.possible_subset(&bag) {
                result = 0;
                break;
            }
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        );
        assert_eq!(result, "8".to_string());
    }
}
