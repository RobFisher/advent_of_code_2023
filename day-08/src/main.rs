use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let (part1, part2) = both_parts(input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}


fn parse_line(line: &str) -> Option<(&str, &str, &str)> {
    let mut node_directions_split = line.split('=');
    let node = node_directions_split.next()?;
    let trimmed_node = node.trim();
    let directions = node_directions_split.next()?;
    let mut directions_split = directions.split(',');
    let left = directions_split.next()?;
    let right = directions_split.next()?;
    // in our input there is always one byte per character
    let left_trimmed = &left[2..5];
    let right_trimmed = &right[1..4];
    Some((trimmed_node, left_trimmed, right_trimmed))
}


fn both_parts(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    let mut nodes = HashMap::new();
    for line in lines {
        if let Some((node, left, right)) = parse_line(line) {
            nodes.insert(node, (left, right));
        }
    }
    let mut instruction_index = 0;
    let mut next_node = "AAA";
    let mut num_steps = 0;
    while next_node != "ZZZ" {
        //println!("{}", next_node);
        let (next_node_left, next_node_right) = nodes.get(next_node).unwrap();
        num_steps += 1;
        let instruction = if let Some(some_instruction) = instructions.chars().nth(instruction_index) {
            some_instruction
        } else {
            instruction_index = 0;
            instructions.chars().nth(0).unwrap()
        };
        instruction_index += 1;
        next_node = if instruction == 'L' {
            next_node_left
        } else {
            next_node_right
        };
    }
    (num_steps, 0)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input =
"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(both_parts(input), (2, 0));

        let input =
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(both_parts(input), (6, 0));
    }
}
