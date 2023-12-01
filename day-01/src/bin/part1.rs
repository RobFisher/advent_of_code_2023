fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}


fn part1(input: &str) -> String {
    let mut result = 0;
    for line in input.lines() {
        let mut first_character = 'x';
        let mut last_character = 'x';
        for character in line.chars() {
            if character >= '0' && character <= '9' {
                if first_character == 'x' {
                    first_character = character;
                }
                last_character = character;
            }
        }
        let number_from_line = format!("{}{}", first_character, last_character);
        result += number_from_line.parse::<i32>().unwrap();
    }
    result.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
        );
        assert_eq!(result, "142".to_string());
    }
}
