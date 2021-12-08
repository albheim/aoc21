use std::fs;
use std::env;
use std::cmp;

fn run_a(crabs: &mut Vec<isize>) {
    crabs.sort_unstable();
    let target = crabs[499];
    let cost = crabs.iter().fold(0, |acc, x| acc + (target - x).abs());
    println!("{}", cost);
}

fn run_b(crabs: &Vec<isize>) {
    // Goal should be between mean and median of values, use this to guide search?
    
    let cost = crabs.iter().fold(std::isize::MAX, |last, target| {
        let cost = crabs.iter().fold(0, |acc, x| {
            let d = (target - x).abs();
            acc + (d * d + d) / 2
        });
        cmp::min(last, cost)
    });
    println!("{}", cost);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let mut crabs: Vec<isize> = fs::read_to_string("input.txt")
        .expect("Could not read file")
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    match version.as_str() {
        "a" => run_a(&mut crabs),
        "b" => run_b(&crabs),
        _ => panic!("Invalid input")
    }
}
