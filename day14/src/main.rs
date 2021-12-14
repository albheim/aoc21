use std::fs;
use std::env;
use std::collections::HashMap;

fn run(template: &str, rules: &HashMap<(char, char), char>, depth: usize) {
    let mut counts: HashMap<(char, char), usize> = HashMap::new();
    for key in template.as_bytes().windows(2) {
        *counts.entry((key[0] as char, key[1] as char)).or_insert(0) += 1;
    }
    let mut newcounts = HashMap::new();

    for _ in 0..depth {
        for (key, value) in counts.iter() {
            let c = rules[key];
            let key1 = (key.0, c);
            let key2 = (c, key.1);
            *newcounts.entry(key1).or_insert(0) += *value;
            *newcounts.entry(key2).or_insert(0) += *value;
        }
        counts.clear();
        for (key, value) in newcounts.iter() {
            counts.insert(*key, *value);
        }
        newcounts.clear();
    }
    newcounts.insert((' ', template.chars().nth(0).unwrap()), 1);
    for (key, value) in counts.into_iter() {
        *newcounts.entry((' ', key.1)).or_insert(0) += value;
    }
    let mut max = 0;
    let mut min = std::usize::MAX;
    for n in newcounts.into_values() {
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
        "b" => run(template, &rules, 40),
        _ => panic!("Invalid input")
    }
}

