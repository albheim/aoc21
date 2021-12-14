use std::fs;
use std::env;
use std::collections::HashMap;

fn dfs(a: char, b: char, rules: &HashMap<(char, char), char>, counts: &mut HashMap<char, usize>, depth: usize) {
    if depth == 0 {
        return 
    }
    let c = rules[&(a, b)];
    *counts.entry(c).or_insert(0) += 1;
    dfs(a, c, rules, counts, depth - 1);
    dfs(c, b, rules, counts, depth - 1);
}

fn run(template: &str, rules: &HashMap<(char, char), char>, depth: usize) {
    let mut counts = HashMap::new();
    for c in template.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    for i in 1..template.len() {
        dfs(template.chars().nth(i-1).unwrap(), template.chars().nth(i).unwrap(), rules, &mut counts, depth);
    }
    let mut max = 0;
    let mut min = std::usize::MAX;
    for n in counts.into_values() {
        if n > max {
            max = n;
        } 
        if n < min {
            min = n;
        }
    }
    println!("{}", max - min);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let (template, unprocessed_rules) = input.split_once("\n\n").unwrap();

    let mut rules = HashMap::new();
    for line in unprocessed_rules.lines() {
        let (a, b) = line.split_once(" -> ").unwrap();
        rules.insert((a.chars().nth(0).unwrap(), a.chars().nth(1).unwrap()), b.chars().nth(0).unwrap());
    }
    
    match version.as_str() {
        "a" => run(template, &rules, 10),
        "b" => run(template, &rules, 25),
        _ => panic!("Invalid input")
    }
}

