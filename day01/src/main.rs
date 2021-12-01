use std::env;
use std::fs;

fn sliding_window(input: Vec<i64>, winsize: usize) {
    let mut sum = 0;
    for i in winsize..input.len() {
        if input[i] > input[i-winsize] {
            sum += 1;
        }
    }
    println!("{}", sum);
}

fn main() {
    let input: Vec<i64> = fs::read_to_string("input.txt").unwrap().split_whitespace().map(str::to_string).map(|line| line.parse::<i64>().unwrap()).collect();

    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    if version == "a" {
        sliding_window(input, 1);
    } else {
        sliding_window(input, 3);
    }
}
