use std::env;
use std::fs;

fn run_a(input: &Vec<i64>) {
    let mut alpha = 0;
    let mut gamma = 0;
    for i in 0..12 {
        let mut val = 0;
        for num in input {
            val += (num & (1 << (11 - i))) >> (11 - i);
        }
        if 2 * val > input.len() as i64 {
            alpha += 1 << (11 - i);
        } else {
            gamma += 1 << (11 - i);
        }
    }
    println!("{}", alpha * gamma);
}

fn run_b(input: &Vec<i64>) {
    let mut a = input.clone();
    for i in (0..12).rev() {
        let countones: i64 = a.iter().map(|num| (num & (1 << i)) >> i).sum();
        let saveval = (2 * countones >= a.len() as i64) as i64;
        a = a.into_iter().filter(|num| saveval == (num & (1 << i)) >> i).collect();
    }
    let oxy = a.first().expect("No values left");

    let mut b = input.clone();
    for i in (0..12).rev() {
        let countones: i64 = b.iter().map(|num| (num & (1 << i)) >> i).sum();
        let saveval = (2 * countones < b.len() as i64) as i64;
        b = b.into_iter().filter(|num| saveval == (num & (1 << i)) >> i).collect();
        if b.len() == 1 {
            break;
        }
    }
    let co2 = b.first().expect("No values left");

    println!("{}", oxy * co2);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input: Vec<i64> = fs::read_to_string("input.txt")
        .expect("Could not read file")
        .lines() 
        .map(|line| i64::from_str_radix(line, 2).unwrap())
        .collect(); 

    match version.as_str() {
        "a" => run_a(&input),
        "b" => run_b(&input),
        _ => panic!("Invalid input")
    }
}

