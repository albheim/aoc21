use std::env;
use std::fs;

fn run_a(input: &Vec<(bool,i64)>) {
    let mut horizontal = 0;
    let mut depth = 0;
    for (depth_step, val) in input {
        if *depth_step {
            depth += val;
        } else {
            horizontal += val;
        }
    }
    println!("{}", depth * horizontal);
}

fn run_b(input: &Vec<(bool,i64)>) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (aim_step, val) in input {
        if *aim_step {
            aim += val;
        } else {
            horizontal += val;
            depth += aim * val;
        }
    }
    println!("{}", depth * horizontal);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input: Vec<(bool, i64)> = fs::read_to_string("input.txt")
        .expect("Could not read file")
        .lines() 
        .map(|line| {
            let i = line.find(' ').expect("No space found");
            let a = &line[0..i];
            let b = line[i+1..].parse::<i64>().expect("Integer parsing error"); 
            match a {
                "forward" => (false, b),
                "down" => (true, b),
                "up" => (true, -b),
                _ => panic!("error in parsing")
            }
        })
        .collect(); 

    match version.as_str() {
        "a" => run_a(&input),
        "b" => run_b(&input),
        _ => panic!("Invalid input")
    }
}
