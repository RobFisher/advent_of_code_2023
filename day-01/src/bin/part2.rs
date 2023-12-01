use std::collections::HashMap;


fn main() {
    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}


fn get_digit_from_string_slice(s: &str) -> Option<u32> {
    let m = HashMap::from([
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
        ("0", 0),
    ]);

    for k in m.keys() {
        if s.starts_with(k) {
            return Some(m[k])
        }
    }
    None
}


fn part2(input: &str) -> String {
    let digits = [
        "1",
        "one",
        "2",
        "two",
        "3",
        "three"
    ];

    let mut result = 0;
    for line in input.lines() {
        let mut first_digit = 99;
        let mut last_digit = 0;
        for i in 0..line.len() {
            let substring: String = line.chars().skip(i).collect();
            if let Some(n) = get_digit_from_string_slice(&substring) {
                last_digit = n;
                if first_digit == 99 {
                    first_digit = n;
                }
            }
        }
        result += (first_digit * 10) + last_digit;
    }
    result.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2(
"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
        );
        assert_eq!(result, "281".to_string());
    }
}
