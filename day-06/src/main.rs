use core::time;

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


/* algebraic solution
distance = speed * time
time = total_time - button_time
speed = button_time

distance = button_time * (total_time - button_time)

distance = (button_time * total_time) - (button_time * button_time)

distance + button_time^2 = button_time * total_time

button_time^2 - (total_time * button_time) + distance = 0

ax^2 + bx + c = 0

a = 1
x = button_time
b = -total_time
c = distance

x = ( -b +/- sqrt(b^2 - 4ac) ) / 2a

*/
fn quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
    println!("a={} b={} c={}", a, b, c);
    let i = ((b * b) - (4.0 * a * c)).sqrt();
    let upper = ((0.0 - b) + i) / (2.0 * a);
    let lower = ((0.0 - b) - i) / (2.0 * a);
    println!("{} to {}", lower, upper);
    (upper, lower)
}


fn winning_button_times(total_time: u64, distance: u64) -> (u64, u64) {
    let (upper_f, lower_f) = quadratic_formula(1.0, 0.0 - total_time as f64, distance as f64);
    // if the max button press time is 5.8ms then the last option we have is 5ms, so round that down
    // if the min button press tim is 2.6ms then the first option we have is 3ms, so round that up
    // BUT that causes a bug where if the answer is a whole number, we're off by one
    // so instead, on a total hunch (it's late) we add one and floor the min, subtract one and ceil the max.
    (upper_f.ceil() as u64 - 1, lower_f.floor() as u64 + 1)
}


fn get_num_race_options(time: u64, record_distance: u64) -> u64 {
    let (max, min) = winning_button_times(time, record_distance);
    let options = max - min + 1;
    println!("t: {} d: {} min: {} max: {} options: {}", time, record_distance, min, max, options);
    options
}


fn part2_parse_number_from_str(s: &str) -> u64 {
    s.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u64>().unwrap()
}


fn both_parts(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
    let times_str = lines.next().unwrap();
    let times = parse_number_list_str(times_str).unwrap();
    let distances_str = lines.next().unwrap();
    let distances = parse_number_list_str(distances_str).unwrap();
    let mut part1_result = 1;
    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        part1_result *= get_num_race_options(time, distance);
    }

    let part2_time = part2_parse_number_from_str(times_str);
    let part2_distance = part2_parse_number_from_str(distances_str);
    let part2_result = get_num_race_options(part2_time, part2_distance);

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
        assert_eq!(both_parts(input), (288, 71503));
        //assert_eq!(part2(input), 30);
    }
}
