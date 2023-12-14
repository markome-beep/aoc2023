use std::{time::Instant, usize};

use itertools::Itertools;

fn main() {
    let now = Instant::now();
    println!("{}", day13_1(include_str!("../../input/day13.input")));
    println!("Time Taken: {}ms", now.elapsed().as_millis());
    println!("{}", day13_2(include_str!("../../input/day13.input")));
    println!("Time Taken: {}ms", now.elapsed().as_millis());
}

struct Drawing {
    map: Vec<Vec<bool>>,
}

impl ToString for Drawing {
    fn to_string(&self) -> String {
        self.map.iter().map(|row| row.iter().map(|&v| if v {'#'} else {'.'}).join("")).join("\n")
    }
}

impl From<&str> for Drawing {
    fn from(value: &str) -> Self {
        let map: Vec<Vec<bool>> = value.lines()
            .map(
                |line|
                line.chars()
                    .map(|c| c == '#')
                    .collect()
            )
            .collect();
        Drawing { map }
    }
}

fn day13_1(input: &str) -> usize {
    todo!()
}

fn day13_2(input: &str) -> usize {
    todo!()
}
