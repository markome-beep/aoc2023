use std::time::Instant;
use itertools::Itertools;

fn main() {
    let now = Instant::now();
    println!("{}", day9_1(include_str!("../../input/day9.input")));
    println!("Time Taken: {}ms", now.elapsed().as_micros());
    println!("{}", day9_2(include_str!("../../input/day9.input")));
    println!("Time Taken: {}ms", now.elapsed().as_micros());
}

fn process_line(line: Vec<i32>) -> (i32, i32) {
    let mut ends: Vec<(i32, i32)> = vec![];
    let acc = (*line.first().unwrap(), *line.last().unwrap());
    let mut prev = line;
    let mut mult = -1;

    while !prev.iter().all(|&n| n==0) {
        let temp: Vec<i32> = prev.iter().tuple_windows().map(|(p, n)| n - p).collect();
        ends.push(
            (*temp.first().unwrap()*mult, *temp.last().unwrap())
        );
        mult *= -1;
        prev = temp;
    }

    ends.into_iter()
        .fold(
            acc, 
            |(accs, acce), (s, e)| (accs + s, acce + e)
        )
}

fn day9_1(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += process_line(
            line.split(" ")
                .filter_map(|num| num.parse().ok())
                .collect_vec()
        ).1;   
    }
    sum
}

fn day9_2(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += process_line(
            line.split(" ")
                .filter_map(|num| num.parse().ok())
                .collect_vec()
        ).0;
    }
    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1() { 
        let result = day9_1("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45");

        assert_eq!(result, 114);
    }

    #[test]
    fn part2() { 
        let result = day9_2("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45");

        assert_eq!(result, 2);
    }
}
