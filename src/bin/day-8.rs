use std::{time::Instant, collections::HashMap};

use num::integer::lcm;

fn main() {
    let now = Instant::now();
    println!("{}", day8_1(include_str!("../../input/day8.input")));
    println!("Time Taken: {}ms", now.elapsed().as_millis());
    println!("{}", day8_2(include_str!("../../input/day8.input")));
    println!("Time Taken: {}ms", now.elapsed().as_millis());
}

fn make_map(input: &str) -> HashMap<&str, (&str, &str)> {
    HashMap::from_iter(
        input.lines()
            .filter_map(|line| {
                let (key, val) = line.split_once(" = ")?;
                let val = val.strip_prefix("(")?.strip_suffix(")")?.split_once(", ")?;
                Some((key, val))
            })
    )
}

fn day8_1(input: &str) -> u32 {
    let Some((instructions, map)) = input.split_once("\n\n") else { return 0 };

    let mut instructions = instructions.chars().cycle();
    let map = make_map(map); 

    let mut count = 0;
    let mut current_value = "AAA";

    loop {
        if current_value == "ZZZ" {
            break count;
        }
        count += 1;
        let instruction = instructions.next();
        match instruction {
            Some('L') => current_value = map.get(&current_value).unwrap().0, 
            Some('R') => current_value = map.get(&current_value).unwrap().1,
            _ => break 0
        }
    }
}

// No assumtions are made for this implementation although it will proably never find the answer
// because I tried running it for 2 hours whilst at gym and it still had not found answer
#[allow(dead_code)]
fn day8_2_brute_force(input: &str) -> u64 {
    let Some((instructions, map)) = input.split_once("\n\n") else { return 0 };

    let mut instructions = instructions.chars().cycle();
    let map = make_map(map); 

    let mut count = 0;
    let mut current_val: Vec<&str> = map.keys()
        .filter_map(|&key| if key.ends_with("A") { Some(key) } else { None })
        .collect();

    loop {
        if current_val.iter().all(|val| val.ends_with("Z")) {
            break count;
        }
        count += 1;
        let instruction = instructions.next();
        match instruction {
            Some('L') => current_val = current_val.iter().map(|val| map.get(val).unwrap().0).collect(),
            Some('R') => current_val = current_val.iter().map(|val| map.get(val).unwrap().1).collect(),
            _ => break 0
        }
    }
}

// Assumtions Made:
//  Each staring position loops back to itself
//  The end of each loop is a location that ends in "Z"
//  The first occurance of a location that ends in "Z" signifiies the end of a loop
//
// For Day 8 part 2 2023 this would get you correct answer
fn day8_2(input: &str) -> u64 {
    let Some((instructions, map)) = input.split_once("\n\n") else { return 0 };

    let map = make_map(map); 

    let mut starting_vals: Vec<&str> = map.keys()
        .filter_map(|&key| if key.ends_with("A") { Some(key) } else { None })
        .collect();

    let mut first_occurance = Vec::new();

    for val in starting_vals.iter_mut() {
        let mut instructions_iter = instructions.chars().cycle();
        let mut count = 0;

        first_occurance.push( loop {
            if val.ends_with("Z") { break count;}

            count += 1;
            let instruction = instructions_iter.next();
            let dest = map.get(val).unwrap();
            
            match instruction {
                Some('L') => *val = dest.0,
                Some('R') => *val = dest.1,
                _ => ()
            }
        })
    }
    first_occurance.iter().fold(1, |acc, &val| { lcm(val, acc) })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1() { 
        let result = day8_1("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)");

        assert_eq!(result, 2);
    }

    #[test]
    fn part1_2() { 
        let result = day8_1("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)");

        assert_eq!(result, 6);
    }

    #[test]
    fn part2() { 
        let result = day8_2("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)");

        assert_eq!(result, 6);
    }
}
