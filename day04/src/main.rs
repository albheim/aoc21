use std::env;
use std::fs;

fn check_bingo(crossed: &Vec<Vec<bool>>) -> bool {
    let mut row = vec![true; crossed.len()];
    for i in 0..crossed.len() {
        let mut col = true;
        for j in 0..crossed[i].len() {
            row[j] &= crossed[i][j];
            col &= crossed[i][j];
        }
        if col {
            return true;
        }
    }
    for j in 0..crossed[0].len() {
        if row[j] {
            return true;
        }
    }
    return false;
}

fn run_bingo(numbers: &Vec<i32>, tables: &Vec<Vec<Vec<i32>>>, mut target: usize) {
    let mut crossed = vec![vec![vec![false; tables[0][0].len()]; tables[0].len()]; tables.len()];
    let mut done = vec![false; tables.len()];
    for num in numbers {
        for i in 0..tables.len() {
            if done[i] {
                continue;
            }
            let mut changed = false;
            for j in 0..tables[i].len() {
                for k in 0..tables[i][j].len() {
                    if tables[i][j][k] == *num {
                        crossed[i][j][k] = true;
                        changed = true;
                    }
                }
            }
            if changed {
                if check_bingo(&crossed[i]) {
                    done[i] = true;
                    target -= 1;
                    if target == 0 {
                        let mut sum = 0;
                        for j in 0..tables[i].len() {
                            for k in 0..tables[i][j].len() {
                                if !crossed[i][j][k] {
                                    sum += tables[i][j][k];
                                }
                            }
                        }
                        println!("{}", num * sum);
                        return;
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input: String = fs::read_to_string("input.txt").expect("Could not read file");

    let chunks: Vec<&str> = input.split("\n\n").collect();

    let numbers: Vec<i32> = chunks[0].split(',').map(|num| num.parse().unwrap()).collect();

    let tables: Vec<Vec<Vec<i32>>> = chunks[1..].iter()
        .map(|chunk| chunk.lines()
            .map(|line| line.split_whitespace()
                .map(|num| num.parse().unwrap()).collect()
            ).collect()
        ).collect();

    match version.as_str() {
        "a" => run_bingo(&numbers, &tables, 1),
        "b" => run_bingo(&numbers, &tables, tables.len()),
        _ => panic!("Invalid input")
    }
}
