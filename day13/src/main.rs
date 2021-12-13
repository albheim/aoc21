use std::fs;
use std::env;
use std::collections::HashSet;

fn run(dots: &mut HashSet<(usize, usize)>, folds: &Vec<(usize, usize)>) {
    for fold in folds {
        if fold.0 != 0 {
            for dot in dots.iter() {
                if dot.0 > fold.0 {
                    dots.remove(dot);
                    dots.insert((fold.0-dot.0, dot.1));
                }
            }
        } else {

        }
    }
    println!("{}", 0);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let (dot_data, fold_data) = input.split_once("\n\n").unwrap();

    let mut dots: HashSet<(usize, usize)> = HashSet::new();
    
    for line in dot_data.lines() {
        let (a, b) = line.split_once(',').unwrap();
        dots.insert((a.parse().unwrap(), b.parse().unwrap()));
    }

    let folds: Vec<(usize, usize)> = fold_data.lines().map(|line| {
        let (a, b) = line.split_once('=').unwrap();
        let b = b.parse().unwrap();
        if a.chars().nth(a.len()-1) == Some('x') {
            (b, 0)
        } else {
            (0, b)
        }
    }).collect();

    match version.as_str() {
        "a" => run(&mut dots, &folds),
        "b" => run(&mut dots, &folds),
        _ => panic!("Invalid input")
    }
}
