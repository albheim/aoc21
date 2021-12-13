use std::fs;
use std::env;

fn fold(dots: &mut Vec<(usize, usize)>, folds: &Vec<&str>, nfolds: usize) {
    for i in 0..nfolds {
        let (a, b) = folds[i].split_once('=').unwrap();
        if a == "fold along x" {
            let b = b.parse().unwrap();
            for j in 0..dots.len() {
                if dots[j].0 > b {
                    dots[j] = (2 * b - dots[j].0, dots[j].1);
                }
            }
        } else {
            let b = b.parse().unwrap();
            for j in 0..dots.len() {
                if dots[j].1 > b {
                    dots[j] = (dots[j].0, 2 * b - dots[j].1);
                }
            }
        }
    }
    dots.sort();
    dots.dedup();
}

fn run_a(dots: &mut Vec<(usize, usize)>, folds: &Vec<&str>) {
    fold(dots, folds, 1);
    println!("{}", dots.len());
}

fn run_b(dots: &mut Vec<(usize, usize)>, folds: &Vec<&str>) {
    fold(dots, folds, folds.len());
    /* 
    // Prints well, but makes assumptions
    for i in 0..10 {
        for j in 0..100 {
            if dots.contains(&(j, i)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    */
    // Prints vertical and mirrored
    let mut lastx = 0;
    let mut lasty = 0;
    for dot in dots {
        if lastx != dot.0 {
            lastx = dot.0;
            lasty = 0;
            println!("");
        }
        print!("{: >1$}", "#", dot.1 - lasty + 1);
        lasty = dot.1 + 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let (dot_data, fold_data) = input.split_once("\n\n").unwrap();

    let mut dots: Vec<(usize, usize)> = Vec::new();
    
    for line in dot_data.lines() {
        let (a, b) = line.split_once(',').unwrap();
        dots.push((a.parse().unwrap(), b.parse().unwrap()));
    }

    let folds: Vec<&str> = fold_data.lines().collect();

    match version.as_str() {
        "a" => run_a(&mut dots, &folds),
        "b" => run_b(&mut dots, &folds),
        _ => panic!("Invalid input")
    }
}
