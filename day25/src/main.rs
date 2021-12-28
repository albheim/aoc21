use std::env;
use std::fs;

fn shiftright(map: &mut Vec<Vec<char>>, row: usize) -> bool {
    let mut change = false;
    let first = map[row][0];
    let rowlen = map[row].len();
    let mut idx = 0;
    while idx < rowlen - 1 {
        let nextidx = (idx + 1) % rowlen;
        if map[row][idx] == '>' && map[row][nextidx] == '.' {
            map[row][idx] = '.';
            map[row][nextidx] = '>';
            change = true;
            idx += 1;
        }
        idx += 1;
    }
    if idx == rowlen - 1 && map[row][rowlen - 1] == '>' && first == '.' {
        map[row][rowlen - 1] = '.';
        map[row][0] = '>';
        change = true;
    }
    return change;
}

fn shiftdown(map: &mut Vec<Vec<char>>, col: usize) -> bool {
    let mut change = false;
    let first = map[0][col];
    let collen = map.len();
    let mut idx = 0;
    while idx < collen - 1 {
        let nextidx = (idx + 1) % collen;
        if map[idx][col] == 'v' && map[nextidx][col] == '.' {
            map[idx][col] = '.';
            map[nextidx][col] = 'v';
            change = true;
            idx += 1;
        }
        idx += 1;
    }
    if idx == collen - 1 && map[collen - 1][col] == 'v' && first == '.' {
        map[collen - 1][col] = '.';
        map[0][col] = 'v';
        change = true;
    }
    return change;
}

fn run(map: &mut Vec<Vec<char>>) {
    let mut update = true;
    let mut iter = 0;
    let rows = map.len();
    let cols = map[0].len();
    while update {
        iter += 1;
        update = false;
        for row in 0..rows {
            let a = shiftright(map, row);
            update |= a;
        }
        for col in 0..cols {
            let a = shiftdown(map, col);
            update |= a;
        }
    }
    println!("{}", iter);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let mut map = fs::read_to_string("input.txt")
        .expect("Could not read file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    match version.as_str() {
        "a" => run(&mut map),
        "b" => {},
        _ => panic!("Invalid input")
    }
}
