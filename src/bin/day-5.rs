use std::{time::Instant, os::unix::process};
use aoc2023::day5_1;
use itertools::Itertools;
 
fn main() {
    let now = Instant::now();
    println!("{}", day5_1(include_str!("../../input/day5.input")));
    println!("Time Taken: {}us", now.elapsed().as_micros());
//    let now = Instant::now();
//    println!("{}", day5_2(include_str!("../../input/day5.input")));
//    println!("Time Taken: {}s", now.elapsed().as_secs_f32());
    let now = Instant::now();
    println!("{}", day5_2_fast(include_str!("../../input/day5.input")));
    println!("Time Taken: {}s", now.elapsed().as_secs_f32());
}

fn match_range(map: &(u64, u64, u64), range: (u64, u64)) -> (Vec<(u64, u64)>, Option<(u64, u64)>) {
    let &(dest, map_source, map_range) = map;
    let (seed_start, seed_range) = range;
    let seed_end = seed_start+seed_range;
    let map_end = map_source+map_range;

    match (map_source, map_end, seed_start, seed_end) {
        (map_source, map_end, seed_start, seed_end)
            if map_source <= seed_end && seed_end <= map_end => {
                (
                    vec![],
                    Some((seed_start+dest-map_source, seed_range))
                )
            },

        (map_source, map_end, seed_start, seed_end)
            if seed_start <= map_source && map_source < seed_end 
            && map_end >= seed_end => {
                (
                    vec![(seed_start, seed_start-map_source)],
                    Some((dest, map_source-seed_end))
                )
            },

        (map_source, map_end, seed_start, seed_end)
            if seed_start <= map_source && map_end <= seed_end => {
                (
                    vec![(seed_start, seed_start-map_source), (seed_end, map_end-seed_end)],
                    Some((dest, map_range))
                )
            },

        (map_source, map_end, seed_start, seed_end)
            if seed_start <= map_end && map_end <= seed_end 
            && map_source < seed_start => {
                (
                    vec![(map_end, seed_end-map_end)],
                    Some((seed_start+dest-map_source, map_end-seed_start))
                )
            },

        _ => (vec![(seed_start, seed_range)], None) 
    }
}

fn process_seed(all_maps: &Vec<Vec<(u64, u64, u64)>>, seed: &(u64, u64), layer: usize) -> u64 {
    let mut seeds = vec![*seed];
    let mut result = Vec::new();
    println!("seed: {seeds:?}");
    println!("layer: {layer}");
    println!("all_maps: {:?}", all_maps.len());

    'seed: while let Some(seed_range) = seeds.pop() {
        for map in &all_maps[layer] {
            if let (non_matched, Some(matched)) = match_range(map, seed_range) {
                dbg!(&non_matched);
                result.push(matched);
                seeds.extend(non_matched);
                continue 'seed;
            }
        }
        result.push(seed_range);
    }
    println!("{:?}", result);
    println!("");

    if layer >= all_maps.len()-1 {
        result.iter().fold(u64::MAX, |acc, &(s, _)| s.min(acc))
    }
    else {
        let mut min = u64::MAX;
        for result_seed in result.iter() {
            min = min.min(process_seed(all_maps, result_seed, layer+1));  
        }
        min
    }
}

fn day5_2_fast(input: &str) -> u64 {
    let Some((seeds, lines)) = input.split_once("\n\n") else { return 0 };
    let seeds: Vec<&str> = seeds
        .strip_prefix("seeds: ").unwrap()
        .split_whitespace().collect();

    let seeds: Vec<(u64, u64)> = seeds.chunks(2)
        .map(|n| (n[0].parse().unwrap(), n[1].parse().unwrap()))
        .collect();

    let mut all_maps: Vec<Vec<(u64, u64, u64)>> = vec![];
    for mapping in lines.split("\n\n") {
        let Some((_, mapping)) = mapping.trim().split_once("\n") else { continue; };
        let mut temp = vec![];
        for map in mapping.lines() {
            temp.push(
                    map.split_whitespace().map(|m| m.parse().unwrap()).collect_tuple().unwrap()
                );
        }
        all_maps.push(temp);
    }
    
    process_seed(&all_maps, &seeds[0], 0)
//    seeds.iter().map(|seed| process_seed(&all_maps, seed, 0)).min().unwrap() 
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn part1() {
        let result = day5_1("seeds: 79 14 55 13

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
56 93 4");
        assert_eq!(result, 35);
    }

    #[test]
    fn part2() {
        let result = day5_2_fast("seeds: 79 14 55 13

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
56 93 4");
        assert_eq!(result, 46);
    }
}
