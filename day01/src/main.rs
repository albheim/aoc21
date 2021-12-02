use std::env;
use std::fs;

fn sliding_window(input: &Vec<i64>, winsize: usize) {
    let mut sum = 0;
    for i in winsize..input.len() {
        if input[i] > input[i-winsize] {
            sum += 1;
        }
    }
    println!("{}", sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input: Vec<i64> = fs::read_to_string("input.txt")
        .unwrap() // Returns the string if reading went well, panics otherwise
        .lines() // Does splits on newline
        .map(|line| line.parse::<i64>().unwrap()) // Takes each element and parses it, again unwrap to return value or fail
        .collect(); // Before this it is just lazy, here it actually runs the previous steps

    match version.as_str() {
        // Pass by reference using &
        "a" => sliding_window(&input, 1), 
        "b" => sliding_window(&input, 3),
        // Catchall
        _ => panic!("Incorrect input, should be a or b.")
    }
}
