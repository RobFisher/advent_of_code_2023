use std::collections::HashMap;
use std::usize;
use std::str::FromStr;
use std::cmp::Ordering;
use itertools::Itertools;


fn main() {
    let input = include_str!("input.txt");
    let (part1, part2) = both_parts(input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    //println!("Part 2: {}", part2(input));
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum CamelCard {
    Joker, C2, C3, C4, C5, C6, C7, C8, C9, C10,
    J, Q, K, A, None
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
            _ => CamelCard::None,
        }
    }
}


#[derive(PartialEq, PartialOrd, Debug)]
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
        let mut counts = self.card_counts();
        let mut count_counts = [0u8; 5];
        let num_jokers = *counts.get(&CamelCard::Joker).unwrap_or(&0u8);
        let best_card_with_most = if num_jokers > 0  && num_jokers < 5 {
            let sorted_keys = counts.keys().sorted();
            // find the highest non-joker card (last in the list of sorted keys) with the most copies
            *sorted_keys.filter(|k| **k != CamelCard::Joker)
                .max_by_key(|k|
                    counts.get(k).unwrap_or(&0u8)
                )
                .unwrap_or(&CamelCard::None)
        } else {
            // either we don't have any jokers or there are no jokers
            // so we don't need to find a card for jokers to become
            CamelCard::None
        };
        // turn the jokers into the best card we have the most of
        counts.remove(&CamelCard::Joker);
        *counts.entry(best_card_with_most).or_insert(0) += num_jokers;
        for v in counts.values() {
            if v > &0 {
                count_counts[*v as usize -1] += 1;
            }
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
        let mut hand_array = [CamelCard::None; 5];
        let mut i = 0;
        for c in s.chars() {
            hand_array[i] = c.into();
            i += 1;
            if i > 4 || hand_array[i-1] == CamelCard::None {
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


// parse as normal and then replace the Jacks with Jokers
impl CamelCardsHandListEntry {
    fn from_str_with_jokers(s: &str) -> Result<Self, CamelCardsHandListEntryError> {
        let mut hand_list_entry: CamelCardsHandListEntry = s.parse()?;
        for i in 0..5 {
            if hand_list_entry.hand.hand[i] == CamelCard::J {
                hand_list_entry.hand.hand[i] = CamelCard::Joker;
            }
        }
        Ok(hand_list_entry)
    }
}


fn parse_input(input: &str, jokers: bool) -> Vec<CamelCardsHandListEntry> {
    let mut hand_list: Vec<CamelCardsHandListEntry>  = vec![];
    for line in input.lines() {
        if let Ok(hand_list_entry) = if jokers {
            CamelCardsHandListEntry::from_str_with_jokers(line)
        } else {
            line.parse()
        }
        {
            hand_list.push(hand_list_entry);
        } else {
            println!("Unable to parse {}", line);
        }
    }
    hand_list
}


fn calculate_result(mut hand_list: Vec<CamelCardsHandListEntry>) -> u64 {
    let mut result = 0;
    hand_list.sort_by_key(|h| h.hand);
    for (i, h) in hand_list.into_iter().enumerate() {
        //dbg!(&h);
        result += (i+1) * h.bid as usize;
    }
    result as u64
}


fn both_parts(input: &str) -> (u64, u64) {
    let hand_list = parse_input(input, false);
    let part1_result = calculate_result(hand_list);
    println!("*****");
    let hand_list = parse_input(input, true);
    let part2_result = calculate_result(hand_list);
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
        assert_eq!(both_parts(input), (6440, 5905));
        //assert_eq!(part2(input), 30);
    }
}
