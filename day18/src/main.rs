use std::env;
use std::fs;

fn parse_node(line: &str, tree: &mut Vec<isize>, idx: &mut usize) {
    if &line[*idx..*idx+1] == "[" {
        *idx += 1; // []
        tree.push(-1);
        parse_node(line, tree, idx);
        *idx += 1; // ,
        parse_node(line, tree, idx);
        *idx += 1; // ]
        tree.push(-2);
    } else {
        let val = line[*idx..*idx+1].parse::<isize>().unwrap();
        *idx += 1; // digit 0-9
        tree.push(val);
    }
}

fn parse_number(line: &str) -> Vec<isize> {
    let mut idx = 0;
    let mut tree: Vec<isize> = Vec::new();
    parse_node(line, &mut tree, &mut idx);
    return tree;
}

fn explode(a: &mut Vec<isize>) -> bool {
    let mut depth = 0;
    let mut mem = 0;
    for i in 0..a.len() {
        if a[i] >= 0 {
            mem = i;
        } else if a[i] == -1 {
            depth += 1;
            if depth == 5 {
                if mem > 0 {
                    a[mem] += a[i + 1];
                }
                let mut j = i + 3;
                while j < a.len() && a[j] < 0 {
                    j += 1;
                }
                if j < a.len() {
                    a[j] += a[i+2];
                }
                a[i] = -3;
                a[i+1] = 0;
                a[i+2] = -3;
                a[i+3] = -3;
                return true;
            }
        } else if a[i] == -2 {
            depth -= 1;
        } 
    }
    return false;
}

fn split(a: &mut Vec<isize>) -> bool {
    for i in 0..a.len() {
        if a[i] > 9 {
            let x = a[i] >> 1;
            let y = a[i] - x;
            a[i] = -1;
            a.insert(i+1, x);
            a.insert(i+2, y);
            a.insert(i+3, -2);
            return true;
        }
    }
    return false;
}

fn magnitude(a: &Vec<isize>, idx: &mut usize) -> isize {
    if a[*idx] >= 0 {
        *idx += 1; // number
        return a[*idx-1];
    }
    *idx += 1; // [
    let x = magnitude(a, idx);
    let y = magnitude(a, idx);
    *idx += 1; // ]
    return 3 * x + 2 * y;
}


fn run_a(numbers: &mut Vec<Vec<isize>>) {
    let mut a = numbers[0].clone();
    for i in 1..numbers.len() {
        let mut tmp = vec![-1];
        tmp.append(&mut a);
        tmp.append(&mut numbers[i]);
        tmp.push(-2);
        a = tmp;
        loop {
            if explode(&mut a) {} else if split(&mut a) {} else {break;}
            a.retain(|&x| x != -3);
        }
    }
    let mut idx = 0;
    println!("{:?}", magnitude(&a, &mut idx));
}

fn run_b(numbers: &mut Vec<Vec<isize>>) {
    let mut maxval = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                let mut tmp = vec![-1];
                tmp.append(&mut numbers[i].clone());
                tmp.append(&mut numbers[j].clone());
                tmp.push(-2);
                loop {
                    if explode(&mut tmp) {} else if split(&mut tmp) {} else {break;}
                    tmp.retain(|&x| x != -3);
                }
                let mut idx = 0;
                let val = magnitude(&tmp, &mut idx);
                if val > maxval {
                    maxval = val;
                }
            }
        }
    }
    println!("{}", maxval);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let mut numbers = fs::read_to_string("input.txt")
        .expect("Could not read file")
        .lines()
        .map(|line| parse_number(&line))
        .collect(); 

    match version.as_str() {
        "a" => run_a(&mut numbers),
        "b" => run_b(&mut numbers),
        _ => panic!("Invalid input")
    }
}
