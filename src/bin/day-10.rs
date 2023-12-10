use std::time::Instant;

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

fn day10_1(input: &str) -> u32 {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
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
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1() { 
        let result = day10_1(
".....
.S-7.
.|.|.
.L-J.
....."
        );

        assert_eq!(result, 4);
    }

    #[test]
    fn part2() { 
        let result = day10_2("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45");

        assert_eq!(result, 2);
    }
}
