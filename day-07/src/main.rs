use std::collections::HashMap;
use std::usize;
use std::str::FromStr;
use std::cmp::Ordering;


fn main() {
    let input = include_str!("input.txt");
    let (part1, part2) = both_parts(input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    //println!("Part 2: {}", part2(input));
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum CamelCard {
    C2, C3, C4, C5, C6, C7, C8, C9, C10,
    J, Q, K, A, Joker
}


// note that we can compare these with each other because they
// get a discriminant value by default:
// https://doc.rust-lang.org/reference/items/enumerations.html
impl From<char> for CamelCard {
    fn from(value: char) -> Self {
        match value {
            '2' => CamelCard::C2,
            '3' => CamelCard::C3,
            '4' => CamelCard::C4,
            '5' => CamelCard::C5,
            '6' => CamelCard::C6,
            '7' => CamelCard::C7,
            '8' => CamelCard::C8,
            '9' => CamelCard::C9,
            'T' => CamelCard::C10,
            'J' => CamelCard::J,
            'Q' => CamelCard::Q,
            'K' => CamelCard::K,
            'A' => CamelCard::A,
            _ => CamelCard::Joker,
        }
    }
}


#[derive(PartialEq, PartialOrd)]
enum CamelCardsHandType {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct CamelCardsHand {
    hand: [CamelCard; 5],
}


impl CamelCardsHand {
    fn card_counts(&self) -> HashMap<CamelCard, u8> {
        let mut counts = HashMap::new();
        for c in self.hand {
            *counts.entry(c).or_insert(0) += 1;
        }
        counts
    }

    fn hand_type(&self) -> CamelCardsHandType {
        let counts = self.card_counts();
        let mut count_counts = [0u8; 5];
        for v in counts.values() {
            count_counts[*v as usize -1] += 1;
        }
        match count_counts {
            [0, 0, 0, 0, 1] => CamelCardsHandType::FiveOfAKind,
            [1, 0, 0, 1, 0] => CamelCardsHandType::FourOfAKind,
            [0, 1, 1, 0, 0] => CamelCardsHandType::FullHouse,
            [_, _, 1, 0, 0] => CamelCardsHandType::ThreeOfAKind,
            [1, 2, 0, 0, 0] => CamelCardsHandType::TwoPairs,
            [_, 1, 0, 0, 0] => CamelCardsHandType::Pair,
            _ => CamelCardsHandType::HighCard,
        }
    }
}


impl PartialOrd for CamelCardsHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}


impl Ord for CamelCardsHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_hand_type = self.hand_type();
        let other_hand_type = other.hand_type();
        if self_hand_type > other_hand_type
        {
            Ordering::Greater
        } else if self_hand_type < other_hand_type {
            Ordering::Less
        } else {
            let mut first_non_matching_index = 99;
            for i in 0..5 {
                if self.hand[i] != other.hand[i] {
                    first_non_matching_index = i;
                    break;
                }
            };
            if first_non_matching_index == 99 {
                Ordering::Equal
            } else {
                self.hand[first_non_matching_index].cmp(&other.hand[first_non_matching_index])
            }
        }
    }
}


struct CamelCardsHandError;
impl FromStr for CamelCardsHand {
    type Err = CamelCardsHandError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hand_array = [CamelCard::Joker; 5];
        let mut i = 0;
        for c in s.chars() {
            hand_array[i] = c.into();
            i += 1;
            if i > 4 || hand_array[i-1] == CamelCard::Joker {
                break;
            }
        }
        if i == 5 {
            Ok(CamelCardsHand{hand: hand_array})
        } else {
            Err(CamelCardsHandError)
        }
    }
}


// TODO impl a type to represent a hand plus a bid


#[derive(Debug)]
struct CamelCardsHandListEntry {
    hand: CamelCardsHand,
    bid: u32,
}


struct CamelCardsHandListEntryError;
impl FromStr for CamelCardsHandListEntry {
    type Err = CamelCardsHandListEntryError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = s.split(' ');
        let Some(hand_str) = fields.next() else {
            return Err(CamelCardsHandListEntryError);
        };
        let Ok(hand) = hand_str.parse() else {
            return Err(CamelCardsHandListEntryError);
        };
        let Some(bid_str) = fields.next() else {
            return Err(CamelCardsHandListEntryError);
        };
        let Ok(bid) = bid_str.parse() else {
            return Err(CamelCardsHandListEntryError);
        };

        Ok(CamelCardsHandListEntry {
            hand,
            bid,
        })
    }
}


fn both_parts(input: &str) -> (u64, u64) {
    let mut hand_list: Vec<CamelCardsHandListEntry>  = vec![];
    for line in input.lines() {
        if let Ok(hand_list_entry) = line.parse() {
            hand_list.push(hand_list_entry);
        } else {
            println!("Unable to parse {}", line);
        }
    }
    hand_list.sort_by_key(|h| h.hand);
    let mut part1_result = 0;
    for (i, h) in hand_list.into_iter().enumerate() {
        //dbg!(&h);
        part1_result += (i+1) * h.bid as usize;
    }
    let part2_result = 0;
    (part1_result as u64, part2_result)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input =
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(both_parts(input), (6440, 0));
        //assert_eq!(part2(input), 30);
    }
}
