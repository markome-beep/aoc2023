use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;
use std::cmp::Ordering::{Less, Greater};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn main() {
    let now = Instant::now();
    println!("{}", day7_1(include_str!("../../input/day7.input")));
    println!("Time Taken: {}ms", now.elapsed().as_micros());
    println!("{}", day7_2(include_str!("../../input/day7.input")));
    println!("Time Taken: {}ms", now.elapsed().as_micros());
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    hand_type: u32,
    cards: Vec<u32>,
    bet: u32
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type > other.hand_type {
            Greater
        }
        else if self.hand_type < other.hand_type {
            Less
        }
        else {
            self.cards.iter()
                .zip(other.cards.iter())
                .fold_while(Greater, |_acc, (s, o)| {
                    if s == o { Continue(Greater) } else if s > o { Done(Greater) } else { Done(Less) }
                })
                .into_inner()
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_hand_type(cards: &Vec<u32>) -> u32 {
    let mut counts = HashMap::new();
    let mut j = 0;
    for c in cards.iter() {
        if c == &0 {
            j += 1;
            continue;
        }
        counts.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    counts.iter_mut().for_each(|(_, val)| *val += j);
    
    let Some(max_val) = counts.values().max() else { return 7 };

    match (counts.len() as u32, max_val) {
        (1, _) => 7,
        (2, 4) => 6,
        (2, 3) => 5,
        (3, 3) => 4,
        (3, 2) => 3,
        (4, _) => 2,
        (5, _) => 1,
        _ => 0
    }
}

fn read_hand(line: &str, map: &HashMap<char, u32>) -> Hand {

    let Some((cards, bet)) = line.split_once(" ") else { panic!("Invalid Line") };
    
    let bet = bet.parse().unwrap();
    let cards = cards.chars()
        .map(|c| {
            if let Some(r) = c.to_digit(10) {
                r
            }
            else {
                if let Some(&r) = map.get(&c) {
                    r 
                }
                else { 0 }
            }
        }).collect();
    let hand_type = get_hand_type(&cards);

    Hand { cards, hand_type, bet }
}

fn day7_1(intput: &str) -> u32 {
    let mut heap: BinaryHeap<Hand> = BinaryHeap::new();
    let map = HashMap::from([
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14)
    ]);
    for line in intput.lines() {
        heap.push(read_hand(line, &map));
    }
    let mut sum = 0;
    let mut rank = heap.len() as u32;
    while let Some(hand) = heap.pop() {
        sum += rank * hand.bet;
        rank -= 1;
    }
    sum
}

fn day7_2(intput: &str) -> u32 {
    let mut heap: BinaryHeap<Hand> = BinaryHeap::new();
    let map = HashMap::from([
        ('T', 10),
        ('J', 0),
        ('Q', 12),
        ('K', 13),
        ('A', 14)
    ]);
    for line in intput.lines() {
        heap.push(read_hand(line, &map));
    }
    let mut sum = 0;
    let mut rank = heap.len() as u32;
    while let Some(hand) = heap.pop() {
        sum += rank * hand.bet;
        rank -= 1;
    }
    sum
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn part1() { 
        let result = day7_1("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");

        assert_eq!(result, 6440);
    }

    #[test]
    fn part2() { 
        let result = day7_2("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");

        assert_eq!(result, 5905);
    }
}





