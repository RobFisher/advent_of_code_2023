use std::str::FromStr;


fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}


struct Map2D {
    map_2d: Vec<Vec<char>>,
}


#[derive(Debug, PartialEq, Eq)]
struct Map2DError;


impl FromStr for Map2D {
    type Err = Map2DError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map_2d: Vec<Vec<char>> = Vec::new();
        for line in s.lines() {
            let mut line_vec: Vec<char> = Vec::new();
            for c in line.chars() {
                line_vec.push(c);
            }
            map_2d.push(line_vec);
        }
        Ok(Map2D { map_2d: map_2d })
    }
}


impl Map2D {
    fn get_coordinate(&self, x: usize, y: usize) -> char {
        let mut result = '.';
        if let Some(row) = self.map_2d.get(y) {
            if let Some(column) = row.get(x) {
                result = *column;
            }
        }
        result
    }

    fn is_symbol(&self, x: usize, y: usize) -> bool {
        let c = self.get_coordinate(x, y);
        c != '.' && !c.is_numeric()
    }

    fn symbol_adjacent_to(&self, x: usize, y: usize) -> bool {
        (x > 0) && (y > 0) && self.is_symbol(x-1, y-1) ||
        (x > 0) && self.is_symbol(x-1, y) ||
        (x > 0) && self.is_symbol(x-1, y+1) ||
        (y > 0) && self.is_symbol(x, y-1) ||
        self.is_symbol(x, y+1) ||
        (y > 0) && self.is_symbol(x+1, y-1) ||
        self.is_symbol(x+1, y) ||
        self.is_symbol(x+1, y+1)
    }

    fn get_part_number_total_in_row(&self, y: usize) -> u32 {
        let mut result = 0u32;
        if let Some(row) = self.map_2d.get(y) {
            let mut current_number = 0;
            let mut adjacent_symbol = false;
            for (x, c) in row.iter().enumerate() {
                if c.is_numeric() {
                    current_number *= 10;
                    current_number += c.to_digit(10).unwrap();
                    if !adjacent_symbol {
                        adjacent_symbol = self.symbol_adjacent_to(x, y);
                    }
                } else {
                    // it's only a part number if there is an adjacent symbol
                    if adjacent_symbol {
                        result += current_number;
                    }
                    current_number = 0;
                    adjacent_symbol = false;
                }
            }
            // if we've reached the end of the line we might have a part number
            if adjacent_symbol {
                result += current_number;
            }
        }
        result
    }

    fn get_part_number_total(&self) -> u32 {
        (0..self.map_2d.len()).map(|y| self.get_part_number_total_in_row(y))
                              .sum()
    }
}


fn part1(input: &str) -> u32 {
    let map = Map2D::from_str(input).expect("Unable to parse map.");
    map.get_part_number_total()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        );
        assert_eq!(result, 4361);
    }
}

