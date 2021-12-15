use std::fs;
use std::env;
use std::collections::VecDeque;

fn solve(dists: &mut Vec<Vec<[usize; 2]>>) -> usize {
    let xlen = dists.len();
    let ylen = dists[0].len();

    let mut floodfill: VecDeque<(usize, usize)> = VecDeque::new();

    dists[xlen - 1][ylen - 1][1] = dists[xlen - 1][ylen - 1][0];
    floodfill.push_back((xlen - 1 , ylen - 1));
    while let Some((x, y)) = floodfill.pop_front() {
        if x > 0 {
            let newval = dists[x][y][1] + dists[x - 1][y][0];
            if newval < dists[x - 1][y][1] {
                dists[x - 1][y][1] = newval;
                floodfill.push_back((x - 1, y));
            }
        }
        if x < xlen - 1 {
            let newval = dists[x][y][1] + dists[x + 1][y][0];
            if newval < dists[x + 1][y][1] {
                dists[x + 1][y][1] = newval;
                floodfill.push_back((x + 1, y));
            }
        }
        if y > 0 {
            let newval = dists[x][y][1] + dists[x][y - 1][0];
            if newval < dists[x][y - 1][1] {
                dists[x][y - 1][1] = newval;
                floodfill.push_back((x, y - 1));
            }
        }
        if y < ylen - 1 {
            let newval = dists[x][y][1] + dists[x][y + 1][0];
            if newval < dists[x][y + 1][1] {
                dists[x][y + 1][1] = newval;
                floodfill.push_back((x, y + 1));
            }
        }
    }
    return dists[0][0][1] - dists[0][0][0];
}

fn run(dists: &mut Vec<Vec<[usize; 2]>>, scale: usize) {
    let collen = dists.len();
    let rowlen = dists[0].len();
    for row in 0..collen {
        for _ in 1..scale {
            let start = dists[row].len() - rowlen;
            dists[row].extend_from_within(start..);
            for i in (start + rowlen)..dists[row].len() {
                dists[row][i][0] = dists[row][i][0] % 9 + 1;
            }
        }
    }
    for row in 0..(scale - 1)*collen {
        dists.push(dists[row].iter().map(|p| [p[0] % 9 + 1, p[1]]).collect());
    }
    println!("{}", solve(dists));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let mut dists: Vec<Vec<[usize; 2]>> = fs::read_to_string("input.txt")
        .expect("Could not read file")
        .lines()
        .map(|line| line.chars()
            .map(|c| [c as usize - 0x30, std::usize::MAX])
            .collect()
        ).collect();

    match version.as_str() {
        "a" => run(&mut dists, 1),
        "b" => run(&mut dists, 5),
        _ => panic!("Invalid input")
    }
}

