use std::fs;
use std::env;

fn run(input: &mut Vec<usize>, steps: usize) {
    let mut ptr = 0;
    for _ in 0..steps {
        input[(ptr + 7) % 9] += input[ptr]; 
        ptr = (ptr + 1) % 9;
    }
    println!("{}", input.iter().sum::<usize>())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let mut fish: Vec<usize> = vec![0; 9];

    for num in fs::read_to_string("input.txt").expect("Could not read file").split(',') {
        let days = num.parse::<usize>().unwrap();
        fish[days] += 1;
    }

    match version.as_str() {
        "a" => run(&mut fish, 80),
        "b" => run(&mut fish, 256),
        _ => panic!("Invalid input")
    }
}
