use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
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


fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let seeds = parse_number_list_str(lines.next().unwrap()).unwrap();
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
    maps.push(current_map); // there's no line with a ':' at the end to add the final map, so we need this here
    //dbg!(&maps);
    seeds.into_iter()
         .map(|s| traverse_maps(&maps, s))
         .min().unwrap()
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
        assert_eq!(part1(input), 35);
        //assert_eq!(part2(input), 30);
    }
}
