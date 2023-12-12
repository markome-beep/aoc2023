use std::{time::Instant, usize};

use itertools::Itertools;

fn main() {
    let now = Instant::now();
    println!("{}", day11_1(include_str!("../../input/day11.input")));
    println!("Time Taken: {}ms", now.elapsed().as_millis());
    println!("{}", day11_2(include_str!("../../input/day11.input")));
    println!("Time Taken: {}ms", now.elapsed().as_millis());
}

struct Space {
    map: Vec<Vec<bool>>,
    empty_rows: Vec<bool>,
    empty_cols: Vec<bool>
}

impl Space {
    fn expansion(&self, p1: (usize, usize), p2: (usize, usize)) -> usize {
        (p1.0.min(p2.0)..p1.0.max(p2.0)).fold(0, |acc, row| if self.empty_rows[row] {acc + 1} else {acc}) + 
        (p1.1.min(p2.1)..p1.1.max(p2.1)).fold(0, |acc, col| if self.empty_cols[col] {acc + 1} else {acc}) 
    }

    fn calc_distances(&self, expansion_coeff: usize) -> usize {
        let points: Vec<Vec<(usize, usize)>> = self.map.iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, &p)| if p { Some((i, j)) } else { None })
                    .collect()
            }).collect();
        let points: Vec<(usize, usize)> = points.into_iter().flatten().collect();

        let mut sum = 0;
        for ((x1, y1), (x2, y2)) in points.iter().tuple_combinations::<(_, _)>() {
            sum += x1.max(x2) - x1.min(x2) + y1.max(y2) - y1.min(y2) + self.expansion((*x1, *y1), (*x2, *y2))*expansion_coeff; 
        }
        sum
    }
}

impl ToString for Space {
    fn to_string(&self) -> String {
        self.map.iter().map(|row| row.iter().map(|&v| if v {'#'} else {'.'}).join("")).join("\n")
    }
}

impl From<&str> for Space {
    fn from(value: &str) -> Self {
        let map: Vec<Vec<bool>> = value.lines()
            .map(
                |line|
                line.chars()
                    .map(|c| c == '#')
                    .collect()
            )
            .collect();
        let empty_rows = map.iter()
                .map(|row| row.iter().all(|point| !point))
                .collect();
        let empty_cols = (0..map[0].len())
                .map(|col| (0..map.len()).all(|row| !map[row][col]))
                .collect();
        Space { map, empty_rows, empty_cols }
    }
}

fn day11_1(input: &str) -> usize {
    let space = Space::from(input);
    space.calc_distances(1)
}

fn day11_2(input: &str) -> usize { 
    let space = Space::from(input);
    space.calc_distances(1000000-1)
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1() { 
        let input = 
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    
        let r = day11_1(input);
        assert_eq!(r, 374);
    }

    #[test]
    fn part2() { 
        let input = 
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    
        let r = Space::from(input);
        assert_eq!(r.calc_distances(9), 1030);
    }
}
