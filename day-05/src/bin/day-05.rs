use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    let (part1, part2) = both_parts(input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    //println!("Part 2: {}", part2(input));
}


#[derive(Debug)]
struct GardeningMapEntry {
    dest_range_start: u64,
    src_range_start: u64,
    range_len: u64,
}


type GardeningMap = Vec<GardeningMapEntry>;


#[derive(Debug, PartialEq, Eq)]
struct GardeningMapEntryError;


fn parse_number_list_str(s: &str) -> Option<Vec<u64>> {
    let number_list_str_split = s.split(' ');
    let result: Vec<u64> = number_list_str_split.filter_map(|n| n.parse::<u64>().ok()).collect();
    if result.len() > 0 {
        Some(result)
    } else {
        None
    }
}


impl FromStr for GardeningMapEntry {
    type Err = GardeningMapEntryError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = parse_number_list_str(s).ok_or(GardeningMapEntryError)?;
        Ok(GardeningMapEntry {
            dest_range_start: *numbers.get(0).ok_or(GardeningMapEntryError)?,
            src_range_start: *numbers.get(1).ok_or(GardeningMapEntryError)?,
            range_len: *numbers.get(2).ok_or(GardeningMapEntryError)?,
        })
    }
}


fn get_mapped_value(map: &GardeningMap, value: u64) -> u64 {
    for entry in map {
        if value >= entry.src_range_start && value < entry.src_range_start + entry.range_len {
            let diff = value - entry.src_range_start;
            return entry.dest_range_start + diff
        }
    }
    value
}


fn traverse_maps(maps: &Vec<GardeningMap>, seed: u64) -> u64 {
    let mut mapped_value = seed;
    //println!("seed {}", seed);
    for map in maps {
        mapped_value = get_mapped_value(&map, mapped_value);
        //println!("{}", mapped_value);
    }
    //println!("location {}", mapped_value);
    mapped_value
}


fn parse_maps(lines: std::str::Lines<'_>) -> Vec<Vec<GardeningMapEntry>> {
    let mut maps: Vec<GardeningMap> = vec![];
    let mut current_map: GardeningMap = vec![];
    for line in lines {
        if line.contains(':') {
            maps.push(current_map);
            current_map = vec![];
        } else {
            if let Ok(map_entry) = GardeningMapEntry::from_str(line) {
                current_map.push(map_entry);
            }
        }
    }
    maps.push(current_map);
    // there's no line with a ':' at the end to add the final map, so we need this here
    //dbg!(&maps);
    maps
}


fn expand_seeds(unexpanded_seeds: &Vec<u64>) -> Vec<u64> {
    let mut seeds = vec![];
    for i in 0..unexpanded_seeds.len()/2 {
        let start_seed = unexpanded_seeds[i*2];
        let range_length = unexpanded_seeds[(i*2)+1];
        let end_seed = start_seed + range_length;
        let mut seeds_to_add: Vec<u64> = (start_seed..end_seed).collect();
        seeds.append(&mut seeds_to_add);
    }
    seeds
}


fn both_parts(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
    let seeds = parse_number_list_str(lines.next().unwrap()).unwrap();
    let expanded_seeds = expand_seeds(&seeds);
    let seeds_to_search = expanded_seeds.len();
    println!("Seeds to search: {}", seeds_to_search);
    let maps = parse_maps(lines);

    let part1_result = 
    (&seeds).into_iter()
            .map(|s| traverse_maps(&maps, *s))
            .min().unwrap();

    let update_interval = (seeds_to_search/1000) + 1;
    let mut next_progress_update = update_interval;
    let mut seeds_searched = 0;
    let mut part2_result = u64::MAX;
    for seed in expanded_seeds {
        seeds_searched += 1;
        if seeds_searched > next_progress_update {
            next_progress_update += update_interval;
            println!("{}%", (seeds_searched as f64 /seeds_to_search as f64)*100.0);
        }
        let location = traverse_maps(&maps, seed);
        if location < part2_result {
            part2_result = location;
        }
    }
    (part1_result, part2_result)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input =
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(both_parts(input), (35, 46));
        //assert_eq!(part2(input), 30);
    }
}
