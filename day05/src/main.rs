use std::env;
use std::fs;
use std::collections::HashMap;
use std::cmp;
use num::signum;

fn check_lines(input: &Vec<((i32, i32), (i32, i32))>, diagonals: bool) {
    let mut count: HashMap<(i32, i32), i32> = HashMap::new();
    let mut total = 0;

    for coordinates in input {
        let ((x1, x2), (y1, y2)) = coordinates;
        if !diagonals && *x1 != *x2 && *y1 != *y2 {
            continue;
        }
        let dx = x2 - x1;
        let dy = y2 - y1;
        let xstep = signum(dx);
        let ystep = signum(dy);
        let nstep = cmp::max(dx * xstep, dy * ystep) + 1;
        for i in 0..nstep {
            let x = *x1 + i * xstep;
            let y = *y1 + i * ystep;
            let val = count.entry((x, y)).or_insert(0);
            *val += 1;
            if *val == 2 {
                total += 1;
            }
        }
    }
    println!("{}", total);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input: Vec<((i32, i32), (i32, i32))> = fs::read_to_string("input.txt")
        .expect("Could not read file")
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(" -> ").unwrap();
            let (x1, y1) = a.split_once(",").unwrap();
            let (x2, y2) = b.split_once(",").unwrap();

            ((x1.parse().unwrap(), x2.parse().unwrap()),
             (y1.parse().unwrap(), y2.parse().unwrap()))
        }).collect();

    match version.as_str() {
        "a" => check_lines(&input, false),
        "b" => check_lines(&input, true),
        _ => panic!("Invalid input")
    }
}
