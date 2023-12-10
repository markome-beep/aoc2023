use std::{time::Instant, usize, collections::HashSet};

fn main() {
    let now = Instant::now();
    println!("{}", day10_1(include_str!("../../input/day10.input")));
    println!("Time Taken: {}ms", now.elapsed().as_micros());
    println!("{}", day10_2(include_str!("../../input/day10.input")));
    println!("Time Taken: {}ms", now.elapsed().as_micros());
}

fn find_s(map: &Vec<Vec<char>>) -> ((usize, usize), char) {
    let mut start = (0, 0);

    's: for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'S' { start = (i, j); break 's; }
        } 
    }

    for dir in ['E', 'W', 'S', 'N'] {
        let mut temp = start;
        match dir {
            'E' => { temp = (temp.0, temp.1+1) },
            'W' => { temp = (temp.0, temp.1-1) },
            'S' => { temp = (temp.0+1, temp.1) },
            'N' => { temp = (temp.0-1, temp.1) },
            _ => ()
        }
        if let Some(_) = make_move(&map, temp, dir) { return (temp, dir) };
    }
    (start, ' ')
}

fn find_and_replace_s(map: &Vec<Vec<char>>) -> Option<((usize, usize), char, char, (usize, usize))> {
    let mut start = (0, 0);

    's: for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'S' { start = (i, j); break 's; }
        } 
    }

    let mut connections = vec![];

    for dir in ['E', 'W', 'S', 'N'] {
        let mut temp = start;
        match dir {
            'E' => { temp = (temp.0, temp.1+1) },
            'W' => { temp = (temp.0, temp.1-1) },
            'S' => { temp = (temp.0+1, temp.1) },
            'N' => { temp = (temp.0-1, temp.1) },
            _ => ()
        }
        if let Some(_) = make_move(&map, temp, dir) { connections.push(dir); };
    }

    dbg!(&connections);

    match (connections[0], connections[1]) {
        ('E', a) => 
            match a {
            'W' => Some(((start.0, start.1+1), 'E', '-', start)),
            'S' => Some(((start.0, start.1+1), 'E', 'F', start)),
            'N' => Some(((start.0, start.1+1), 'E', 'L', start)),
            _ => None
        }
        ('W', a) => 
            match a {
            'S' => Some(((start.0, start.1-1), 'W', '7', start)),
            'N' => Some(((start.0, start.1-1), 'W', 'J', start)),
            _ => None
        }
        ('S', 'N') => Some(((start.0+1, start.1), 'S', '|', start)),
        _ => None
    }
}

fn make_move(map: &Vec<Vec<char>>, position: (usize, usize), entry: char) -> Option<char> {
    match map[position.0][position.1] {
       '|' | '-' => {
            Some(entry)
        },
        'L' => {
            match entry {
                'W' => Some('N'),
                'S' => Some('E'),
                _ => None
            }
        },
        'J' => {
            match entry {
                'E' => Some('N'),
                'S' => Some('W'),
                _ => None 
            }
        },
        '7' => {
            match entry {
                'N' => Some('W'),
                'E' => Some('S'),
                _ => None 
            }
        },
        'F' => {
            match entry {
                'N' => Some('E'),
                'W' => Some('S'),
                _ => None 
            }
        },
        _ => None
    } 
}

fn make_map(input: &str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = input.trim().lines()
        .map(|line| 
            ".".chars()
                .chain(line.chars()
                    .chain(".".chars())
                ).collect()
        )
        .collect();
    map.insert(0, vec!['.'; map[0].len()]);
    map.push(vec!['.'; map[0].len()]);
    map
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map.iter() {
        for c in row.iter() { print!("{c} ") }
        println!();
    }
}

fn day10_1(input: &str) -> u32 {
    let map = make_map(input);
//    print_map(&map);
    let (mut pos, mut entry) = find_s(&map);
    let mut sum = 1;

    loop {
        entry = make_move(&map, pos, entry).unwrap();
        sum += 1;
        match entry {
            'E' => { pos = (pos.0, pos.1+1) },
            'W' => { pos = (pos.0, pos.1-1) },
            'S' => { pos = (pos.0+1, pos.1) },
            'N' => { pos = (pos.0-1, pos.1) },
            _ => ()
        }
        if map[pos.0][pos.1] == 'S' {
            break;
        }
    }
    sum/2
}

fn day10_2(input: &str) -> u32 {
    let mut map = make_map(input);
    print_map(&map);
    let (mut pos, mut entry, c, origin) = find_and_replace_s(&map).unwrap();
    
    let mut path = HashSet::new();

    loop {
        path.insert(pos);
        entry = make_move(&map, pos, entry).unwrap();
        match entry {
            'E' => { pos = (pos.0, pos.1+1) },
            'W' => { pos = (pos.0, pos.1-1) },
            'S' => { pos = (pos.0+1, pos.1) },
            'N' => { pos = (pos.0-1, pos.1) },
            _ => ()
        }
        if map[pos.0][pos.1] == 'S' {
            break;
        }
    }

    map[origin.0][origin.1] = c;
    path.insert(origin);
    print_map(&map);
    let mut sum = 0;
    for i in 1..map.len()-1 {
        for j in 1..map[0].len()-1 {
            if path.contains(&(i, j)) { continue; }
            if is_inside(&map[i], (i, j), &path) { sum += 1; println!("{:?}: {:?}", (i, j), &map[i][j]); }
        }
    }
    sum
}

fn is_inside(map: &Vec<char>, pos: (usize, usize), path: &HashSet<(usize, usize)>) -> bool {
    let mut crosses = 0;
    let mut partial = None;
    for j in 0..pos.1 {
        if path.contains(&(pos.0, j)) {
            match map[j] {
                '|' => crosses += 1,
                'F' => partial = Some('F'),
                'L' => partial = Some('L'),
                'J' => {
                    if partial == Some('F') { crosses += 1 }
                    partial = None;
                }
                '7' => { 
                    if partial == Some('L') { crosses += 1 }
                    partial = None;
                }
                _ => ()
            }
        }
        print!("{} ", map[j]);
    }
    println!();
    crosses % 2 == 1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1() { 
        let result = day10_1(
"
.....
.S-7.
.|.|.
.L-J.
.....
"
        );

        assert_eq!(result, 4);
    }
    #[test]
    fn part1_2() { 
        let result = day10_1(
"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"
        );

        assert_eq!(result, 8);
    }

    #[test]
    fn part2() { 
        let result = day10_2(
"
S-7
|.|
L-J
"
        );

        assert_eq!(result, 1);
    }

    #[test]
    fn part2_2() { 
        let result = day10_2(
"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"
        );

        assert_eq!(result, 1);
    }

    #[test]
    fn part2_3() { 
        let result = day10_2(
"
S------7
|F----7|
||OOOO||
||OOOO||
|L-7F-J|
|II||II|
L--JL--J
"
        );

        assert_eq!(result, 4);
    }

    #[test]
    fn part2_4() { 
        let result = day10_2(
"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"
        );

        assert_eq!(result, 8);
    }
    
    #[test]
    fn part2_5() { 
        let result = day10_2(
"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"
        );

        assert_eq!(result, 10);
    }
}
