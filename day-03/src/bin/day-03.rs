use std::str::FromStr;


fn main() {
    let input = include_str!("../input.txt");
    let map = Map2D::from_str(input).expect("Unable to parse map.");
    let part1_result = map.get_part_number_total();
    let part2_result = map.get_gear_ratio_total();
    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
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

    fn is_numeric(&self, x: usize, y: usize) -> bool {
        self.get_coordinate(x, y).is_numeric()
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

    fn get_number_at_location(&self, x: usize, y: usize) -> u32 {
        // search left to find the start of the number
        let mut x_cursor = x;
        let mut result = 0u32;
        while x_cursor > 0 && self.is_numeric(x_cursor-1, y) {
            x_cursor -= 1;
        }
        // scan the number
        while let Some(n) = self.get_coordinate(x_cursor, y).to_digit(10) {
            result *= 10;
            result += n;
            x_cursor += 1;
        }
        result
    }

    fn gear_ratio_at_location(&self, x: usize, y: usize) -> u32 {
        // the two numbers are either above, below, to the left or to the right
        let mut adjacent_numbers: Vec<u32> = Vec::new();

        if y > 0 {
            // look above
            if self.is_numeric(x, y-1) {
                adjacent_numbers.push(self.get_number_at_location(x, y-1));
            } else {
                // if there is no number directly above, there could be one both
                // up-left and up-right
                if x > 0 && self.is_numeric(x-1, y-1) {
                    adjacent_numbers.push(self.get_number_at_location(x-1, y-1));
                }
                if self.is_numeric(x+1, y-1) {
                    adjacent_numbers.push(self.get_number_at_location(x+1, y-1));
                }
            }
        }
        // look below
        if self.is_numeric(x, y+1) {
            adjacent_numbers.push(self.get_number_at_location(x, y+1));
        } else {
            // if there is no number directly below, there could be one both
            // down-left and down-right
            if x > 0 && self.is_numeric(x-1, y+1) {
                adjacent_numbers.push(self.get_number_at_location(x-1, y+1));
            }
            if self.is_numeric(x+1, y+1) {
                adjacent_numbers.push(self.get_number_at_location(x+1, y+1));
            }
        }

        // look left
        if x > 0 && self.is_numeric(x-1, y) {
            adjacent_numbers.push(self.get_number_at_location(x-1, y));
        }
        // look right
        if self.is_numeric(x+1, y) {
            adjacent_numbers.push(self.get_number_at_location(x+1, y));
        }
        // only valid if there are exactly two numbers
        dbg!(&adjacent_numbers);
        if adjacent_numbers.len() == 2 {
            adjacent_numbers.into_iter().product()
        } else {
            0
        }
    }

    fn get_gear_ratio_total_in_row(&self, y: usize) -> u32 {
        let mut result = 0u32;
        if let Some(row) = self.map_2d.get(y) {
            for (x, c) in row.iter().enumerate() {
                if *c == '*' {
                    result += self.gear_ratio_at_location(x, y);
                }
            }
        }
        result
    }

    fn get_gear_ratio_total(&self) -> u32 {
        (0..self.map_2d.len()).map(|y| self.get_gear_ratio_total_in_row(y))
                              .sum()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input =
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let map = Map2D::from_str(input).expect("invalid input");
        assert_eq!(map.get_part_number_total(), 4361);
        assert_eq!(map.get_gear_ratio_total(), 467835);

        let input2 =
"12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";
        let map2 = Map2D::from_str(input2).expect("invalid input");
        assert_eq!(map2.get_gear_ratio_total(), 6756);
    }
}

