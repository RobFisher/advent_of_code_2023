use std::str::FromStr;


fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
}


#[derive(Debug)]
struct GameCard {
    _game_id: u32,
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}


#[derive(Debug, PartialEq, Eq)]
struct GameCardError;


fn parse_number_list_str(s: &str) -> Option<Vec<u32>> {
    let number_list_str_split = s.split(' ');
    let result: Vec<u32> = number_list_str_split.filter_map(|n| n.parse::<u32>().ok()).collect();
    if result.len() > 0 {
        Some(result)
    } else {
        None
    }
}


impl FromStr for GameCard {
    type Err = GameCardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Err(GameCardError);
        let mut sections = s.split(':');
        // these nested if-lets turned out uglier than I'd imagined. There are various
        // ways around this; see: https://stackoverflow.com/questions/71267256/how-to-avoid-nested-chains-of-if-let
        if let Some(header_section) = sections.next() {
            let header_parts = header_section.split(' ');
            if let Some(game_id_str) = header_parts.last() {
                if let Ok(game_id) = game_id_str.parse::<u32>() {
                    if let Some(number_lists_section) = sections.next() {
                        let mut number_lists = number_lists_section.split('|');
                        if let Some(winning_numbers_str) = number_lists.next() {
                            if let Some(winning_numbers) = parse_number_list_str(winning_numbers_str) {
                                if let Some(your_numbers_str) = number_lists.next() {
                                    if let Some(your_numbers) = parse_number_list_str(your_numbers_str) {
                                        result = Ok(GameCard {
                                            _game_id: game_id,
                                            winning_numbers: winning_numbers,
                                            your_numbers: your_numbers
                                        })
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        result
    }
}


impl GameCard {
    fn points(&self) -> u32 {
        let mut points = 0u32;
        let mut next_score = 1u32;
        for my_number in &self.your_numbers {
            for winning_number in &self.winning_numbers {
                if my_number == winning_number {
                    points = next_score;
                    next_score *= 2;
                }
            }
        }
        points
    }
}


fn part1(input: &str) -> u32 {
    let mut points = 0u32;
    for line in input.lines() {
        if let Ok(game) = GameCard::from_str(line) {
            points += game.points();
        }
    }
    points
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input =
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(input), 13);
    }
}
