use std::{collections::{HashMap, HashSet}, u32};
use itertools::{FoldWhile::{Continue, Done}, Itertools};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn day1_1(input: &str) -> u32 {
    input.split("\n")
    .fold(0, |mut acc, line| {
            let (mut found_l, mut found_r) = (false, false);
            for (l, r) in line.chars().zip(line.chars().rev()) {
                if !found_l && l.is_numeric() {
                    acc += l.to_digit(10).unwrap() * 10;
                    found_l = true;
                }
                if !found_r && r.is_numeric() {
                    acc += r.to_digit(10).unwrap(); 
                    found_r = true;
                }
            } 
            acc
        }
    )
}

pub fn day1_2(input: &str) -> u32 {
    let nums = std::collections::HashMap::from([
        ("one", 1), 
        ("two", 2), 
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7), 
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9)
    ]);
    let mut ans = 0;

    for line in input.lines() {
        let mut lowest = (line.len(), 0);
        for (k, v) in nums.iter() {
            if lowest.0 > line.find(k).unwrap_or(line.len()) {
                lowest = (line.find(k).unwrap() , *v);
                println!("HERE");
            }
        }
        ans += lowest.1 * 10;
    }

    for line in input.lines().map(|line| line.chars().rev().collect::<String>()) {
        let mut lowest = (line.len(), 0);
        for (k, v) in nums.iter().map(|(k, v)| (k.chars().rev().collect::<String>(), v)) {
            if lowest.0 > line.find(&k).unwrap_or(line.len()) {
                lowest = (line.find(&k).unwrap(), *v);
                println!("HERE");
            }
        }
        ans += lowest.1;
    }

    ans.try_into().unwrap() 
}

pub fn day2_1(input: &str) -> u32 {
    // 12 red, 13 green, 14 blue
    input.lines().enumerate().fold(0, 
        |acc, (i, game)| {
            if game.split(": ").last().unwrap()
                .split("; ").fold(true, 
                    |valid, pull| {
                        if !valid { return false }
                        pull.split(", ").fold(true,
                            |valid, color| {
                                if !valid { return false }
                                let parts: Vec<&str> = color.splitn(2, " ").collect();
                                let num: u32 = parts[0].parse().unwrap();
                                let c = parts[1];
                                match (c, num) {
                                    ("blue", n) if n > 14 => false,
                                    ("green", n) if n > 13 => false,
                                    ("red", n) if n > 12 => false,
                                    (_, _) => true
                                }
                            }
                        )
                    }
                ) {
                acc + i as u32 + 1
            }
            else { acc } 
        }
    )
}

pub fn day2_2(input: &str) -> u32 {
    input.lines().fold(0, 
        |acc, game| {
            let (r, g, b) = game.split(": ").last().unwrap()
                .split("; ").fold((0, 0, 0), 
                    |(r, g, b), pull| {
                        let p_val = pull.split(", ").fold((0, 0, 0), 
                            |(r, g, b), color| {
                                let parts: Vec<&str> = color.splitn(2, " ").collect();
                                let num: u32 = parts[0].parse().unwrap();
                                let c = parts[1];
                                match (c, num) {
                                    ("blue", n) if n > b => (r, g, n),
                                    ("green", n) if n > g => (r, n, b),
                                    ("red", n) if n > r => (n, g, b),
                                    (_, _) => (r, g, b) 
                                }
                            }
                        );
                        (
                            std::cmp::max(p_val.0, r),
                            std::cmp::max(p_val.1, g),
                            std::cmp::max(p_val.2, b)
                        )

                    }
                ); 
            acc + (r * g * b) as u32 
        }
    )
}

fn symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

fn check_row(prev: &str, curr: &str, next: &str) -> u32 {
    let mut sum = 0;
    for (_key, group) in curr.chars().enumerate().group_by(|(_, c)| c.is_numeric()).into_iter().filter(|(key, _)| *key) {
        let num: Vec<(usize, char)> = group.collect();
        let start = if num[0].0 != 0 {num[0].0 - 1} else {0};
        let end = if num.last().unwrap().0 + 1 < curr.len() {num.last().unwrap().0 + 1} else {curr.len() - 1};

        if symbol(curr.chars().nth(start).unwrap()) || symbol(curr.chars().nth(end).unwrap()) {
            sum += num.iter().map(|(_, l)| l).collect::<String>().parse::<u32>().unwrap();
            continue;
        }

        let mut iter = prev.chars().zip(next.chars());
        let mut i = 0;

        let range = start..=end;
        for elem in range {
            let (p, n): (char, char) = iter.nth(elem - i).unwrap();
            i = elem + 1;
            if symbol(p) || symbol(n) {
                sum += num.iter().map(|(_, l)| l).collect::<String>().parse::<u32>().unwrap();
                break;
            }
        }
    }
    sum 
}

pub fn day3_1(input: &str) -> u32 {
    let blank: &str = &".".repeat(input.lines().next().unwrap().len());
    let mut engine = vec![blank];
    engine.extend(input.lines());
    engine.push(blank);

    engine.windows(3)
        .map(|win| check_row(win[0], win[1], win[2]))
        .sum()

}

#[derive(Debug)]
struct Gear {
    value: u32,
    parts: u32
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Position(usize, usize);

#[derive(Default)]
struct Walk {
    start: usize,
    end: usize,
    row: usize,
    count: usize
}
impl Iterator for Walk {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let count = self.count;
        self.count += 1;
        match count {
            0 => Some(Position(self.row, self.start)),
            1 => Some(Position(self.row, self.end)),
            even if even % 2 == 0 && even <= (self.end - self.start) * 2 + 3 => Some(Position(self.row-1, self.start+even/2-1)),
            odd if odd % 2 == 1 && odd <= (self.end - self.start) * 2 + 3 => Some(Position(self.row+1, self.start+odd/2-1)),
            _ => None
        }
    }
}

pub fn day3_2(input: &str) -> u32 { 
    let blank: &str = &".".repeat(input.lines().next().unwrap().len());
    let mut engine = vec![blank];
    engine.extend(input.lines());
    engine.push(blank);
    
    let mut gears: HashMap<Position, Gear> = HashMap::new();
    for (mut row, win) in engine.windows(3).enumerate() {
        row += 1;
        let curr = win[1];
        let groups = curr.chars()
            .enumerate()
            .group_by(|(_, c)| c.is_numeric());
            
        for (_, group) in groups.into_iter().filter(|(key, _ )| *key) {
            let part: Vec<(usize, char)> = group.collect();
            let num = part.iter()
                .map(|(_, l)| l)
                .collect::<String>()
                .parse::<u32>()
                .unwrap();

            let start = if part[0].0 != 0 {part[0].0 - 1} else { 0 };
            let end = if part.last().unwrap().0 + 1 < curr.len() {part.last().unwrap().0 + 1} else {curr.len() - 1};
            
            let positions = Walk { start, end, row, count: 0 };
            for pos in positions {
                let c = win[pos.0 + 1 - row].chars().nth(pos.1);
                if c == Some('*') {
                    gears.entry(pos)
                        .and_modify(
                            |gear| {
                                gear.value *= num; 
                                gear.parts += 1
                            }
                        )
                        .or_insert(Gear { value: num, parts: 1 }); 
                } 
            }
        }
    }
    
    gears.values().fold(0, |acc, gear| if gear.parts == 2 { acc + gear.value } else { acc })
}

pub fn day4_1(input: &str) -> u32 {
    input.lines()
        .fold(0, 
            |acc, game| {
                let Some((_, game)) = game.split_once(": ") else { return acc };
                let Some((wins, nums)) = game.split_once(" | ") else { return acc };
                let winning_nums: HashSet<&str> = HashSet::from_iter(wins.split_whitespace());

                acc + u32::pow(2,
                    nums.split_whitespace()
                        .filter(|num| winning_nums.contains(num))
                        .count().try_into().unwrap()
                )/2       
            }
        ) 
}

pub fn day4_2(input: &str) -> u32 {
    let hits: Vec<usize> = input.lines()
        .map(|game| {
            let Some((_, game)) = game.split_once(": ") else { return 0 };
            let Some((wins, nums)) = game.split_once(" | ") else { return 0 };
            let winning_nums: HashSet<&str> = HashSet::from_iter(wins.split_whitespace());

            nums.split_whitespace()
                .filter(|num| winning_nums.contains(num))
                .count().try_into().unwrap()
        }
        )
        .collect(); 

    let mut cards = vec![1; hits.len()];

    for (i, hit) in hits.into_iter().enumerate() {
        for m in i+1..=i+hit {
            cards[m] += cards[i];
        } 
    }
    cards.into_iter().sum()
}

pub fn day5_1(input: &str) -> u64 {
    let Some((seeds, lines)) = input.split_once("\n\n") else { return 0 };
    let seeds: Vec<u64> = seeds
        .strip_prefix("seeds: ").unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut transform = seeds.clone();
    let mut curr_maps: Vec<(u64, u64, u64)> = vec![];
    for mapping in lines.split("\n\n") {
        let Some((_, mapping)) = mapping.trim().split_once("\n") else { continue; };
        for map in mapping.lines() {
            curr_maps
                .push(
                    map.split_whitespace().map(|m| m.parse().unwrap()).collect_tuple().unwrap()
                );
        }
        for t in &mut transform {
            *t = curr_maps.iter()
                .fold_while((*t).clone(), 
                    |acc, m| 
                    if m.1 <= acc && acc < (m.1 + m.2) 
                        { Done(acc + m.0 - m.1) } 
                    else { Continue(acc) }
                ).into_inner();
        }
        curr_maps.clear();
    }
    transform.iter().enumerate().fold((0, transform[0]), 
        |(best_i, best_d), (i, distance)| if distance < &best_d { (i, *distance) } else { (best_i, best_d) }
    ).1
}

struct GenerateSeeds {
    seeds: Vec<(u64, u64)>
}

impl Iterator for GenerateSeeds {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(mut item) = self.seeds.last_mut() else { return None };

        if item.1 == 0 {
            self.seeds.pop();
            item = self.seeds.last_mut()?;
        }

        item.1 -= 1;
        Some(item.0 + item.1)
    }
}

pub fn day5_2(input: &str) -> u64 {
    let Some((seeds, lines)) = input.split_once("\n\n") else { return 0 };
    let seeds: Vec<&str> = seeds
        .strip_prefix("seeds: ").unwrap()
        .split_whitespace().collect();

    let seeds: Vec<(u64, u64)> = seeds.chunks(2)
        .map(|n| (n[0].parse().unwrap(), n[1].parse().unwrap()))
        .collect();
    let seeds = GenerateSeeds { seeds };

    let mut curr_maps: Vec<Vec<(u64, u64, u64)>> = vec![];
    for mapping in lines.split("\n\n") {
        let Some((_, mapping)) = mapping.trim().split_once("\n") else { continue; };
        let mut temp = vec![];
        for map in mapping.lines() {
            temp.push(
                    map.split_whitespace().map(|m| m.parse().unwrap()).collect_tuple().unwrap()
                );
        }
        curr_maps.push(temp);
    }

    let mut best = u64::MAX;

    for mut seed in seeds {
        'outer_loop: for maps in &curr_maps {
            for (target, source, range) in maps {
                if source <= &seed && seed < source + range {
                    seed = seed + target - source;
                    continue 'outer_loop;
                }
            }
        }
        if best > seed { best = seed; }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = day1_2("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
        assert_eq!(result, 281);
    }
    #[test]
    fn it_works_2() {
        let result = day3_1("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(result, 4361);
    }
    #[test]
    fn it_works_3() {
        let result = day3_2("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(result, 467835);
    }
}
