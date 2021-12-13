use std::fs;
use std::env;

fn run_a(input: &String) {
    let mut stack: Vec<char> = Vec::new();
    let mut score = 0;

    for line in input.lines() {
        stack.clear();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => if stack.pop().unwrap() != '(' {
                    score += 3;
                    break
                },
                ']' => if stack.pop().unwrap() != '[' {
                    score += 57;
                    break
                },
                '}' => if stack.pop().unwrap() != '{' {
                    score += 1197;
                    break
                },
                '>' => if stack.pop().unwrap() != '<' {
                    score += 25137;
                    break
                },
                _ => panic!("Invalid char found")
            }
        }
    }
    println!("{}", score);
}

fn run_b(input: &String) {
    let mut stack: Vec<char> = Vec::new();
    let mut scores: Vec<usize>  = Vec::new();

    for line in input.lines() {
        let mut removed = false;
        stack.clear();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => if stack.pop().unwrap() != '(' {
                    removed = true;
                    break
                },
                ']' => if stack.pop().unwrap() != '[' {
                    removed = true;
                    break
                },
                '}' => if stack.pop().unwrap() != '{' {
                    removed = true;
                    break
                },
                '>' => if stack.pop().unwrap() != '<' {
                    removed = true;
                    break
                },
                _ => panic!("Invalid char found")
            }
        }
        if !removed {
            let mut score = 0;
            for c in stack.iter().rev() {
                score *= 5;
                match c {
                    '(' => score += 1,
                    '[' => score += 2,
                    '{' => score += 3,
                    '<' => score += 4,
                    _ => panic!("Invalid char found")
                }
            }
            scores.push(score);
        }
    }
    scores.sort();
    println!("{}", scores[scores.len() >> 1]);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input: String = fs::read_to_string("input.txt").expect("Could not read file");

    match version.as_str() {
        "a" => run_a(&input),
        "b" => run_b(&input),
        _ => panic!("Invalid input")
    }
}
