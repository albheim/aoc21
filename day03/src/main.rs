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

fn getbit(num: &i64, idx: i64) -> i64 {
    (num & (1 << (11 - idx))) >> (11 - idx)
}

fn run_b(input: &Vec<i64>) {
    let mut a = input.to_vec();
    let mut b = input.to_vec();
    for i in 0..12 {
        let mut aval = a.map();
        let mut bval = 0;
        for num in a {
            aval += getbit(&num, i);
        }
        for num in b {
            bval += getbit(&num, i);
        }

        a = a.into_iter().filter(|it| getbit(it, i) == (2 * aval > input.len() as i64) as i64).collect();
        b = b.into_iter().filter(|it| getbit(it, i) == (2 * bval > input.len() as i64) as i64).collect();
    }
    println!("{:?} {:?}", a, b);
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

