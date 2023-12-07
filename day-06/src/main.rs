fn main() {
    let input = include_str!("input.txt");
    let (part1, part2) = both_parts(input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    //println!("Part 2: {}", part2(input));
}


fn parse_number_list_str(s: &str) -> Option<Vec<u64>> {
    let number_list_str_split = s.split(' ');
    let result: Vec<u64> = number_list_str_split.filter_map(|n| n.parse::<u64>().ok()).collect();
    if result.len() > 0 {
        Some(result)
    } else {
        None
    }
}


fn get_distance(button_press_time: u64, total_time: u64) -> u64 {
    let speed = button_press_time;
    let remaining_time = total_time - button_press_time;
    speed * remaining_time
}


fn get_num_race_options(time: u64, record_distance: u64) -> u64 {
    let mut options = 0;
    for button_press_time in 0..time {
        let distance = get_distance(button_press_time, time);
        if distance > record_distance {
            options += 1;
        }
    }
    options
}


fn both_parts(input: &str) -> (u64, u64) {
    let part2_result = 0;

    let mut lines = input.lines();
    let times = parse_number_list_str(lines.next().unwrap()).unwrap();
    let distances = parse_number_list_str(lines.next().unwrap()).unwrap();
    let mut part1_result = 1;
    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        part1_result *= get_num_race_options(time, distance);
    }

    (part1_result, part2_result)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input =
"Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(both_parts(input), (288, 0));
        //assert_eq!(part2(input), 30);
    }
}
