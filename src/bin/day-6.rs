use itertools::Itertools;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let _file = "../../input/day4.input";
    println!("{}", day6_1(include_str!("../../input/day6.input")));
    println!("{}", now.elapsed().as_micros());
    println!("{}", day6_2(include_str!("../../input/day6.input")));
    println!("{}", now.elapsed().as_micros());
    println!("{}", day6_fast(include_str!("../../input/day6.input")));
    println!("{}", now.elapsed().as_micros());
}

fn read_input(input: &str) -> Vec<(usize, usize)> {

    let temp: Vec<Vec<&str>> = input.lines()
        .map(|line| line.split_whitespace().skip(1).collect())
        .collect();

    temp[0].iter()
        .zip(temp[1].iter())
        .map(|(s, t)| (s.parse().unwrap(), t.parse().unwrap())).collect()
}

fn read_input_2(input: &str) -> (u64, u64) {
    input.lines()
        .map(|f| {
            f.split_whitespace()
                .skip(1)
                .collect::<Vec<&str>>()
                .concat()
                .parse()
                .unwrap()
        })
        .collect_tuple()
        .unwrap()
}

fn day6_1(input: &str) -> u32 {
    let mut out = 1; 
    let races = read_input(input);
    for (t, d) in races {
        let mut valid = vec![false; t];
        for i in 0..t {
            valid[i] = i * (t - i) > d;
        }
        out *= valid.iter().filter(|i| **i).count();
    }
    out.try_into().unwrap()
}

fn day6_2(input: &str) -> u32 {
    let mut out = 0; 
    let (t, d) = read_input_2(input);
    for i in 0..t {
        out += if i * (t - i) > d { 1 } else { 0 };
    }
    out
}

fn day6_fast(input: &str) -> u32 {
    let (t, d) = read_input_2(input);
    let (t, d) = (t as f64, d as f64);
    (( t+2.0*d.sqrt() )*( t-2.0*d.sqrt() )).sqrt() as u32
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn part1() { todo!() }
}
