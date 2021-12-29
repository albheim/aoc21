use std::env;
use std::fs;

fn step(z: isize, input: isize, params: [isize; 3]) -> isize {
    let mut newz = z / params[0];
    if z % 26 + params[1] != input {
        newz = 26 * newz + input + params[2];
    }
    return newz;
}

fn find_number(z: isize, params: &Vec<[isize; 3]>, idx: usize) -> isize {
    if idx == params.len() {
        if z == 0 {
            return 1;
        } else {
            return 0;
        }
    }
    if params[idx][0] != 1 {
        let input = z % 26 + params[idx][1];
        if input > 0 && input < 9 {
            let a = find_number(step(z, input, params[idx]), params, idx + 1);
            if a != 0 {
                println!("a {} {} {} {}", idx, input, a, z);
                return 10_isize.pow((14 - idx) as u32) * input + a;
            } 
        }
    } 
    for i in (1..10).rev() {
        let a = find_number(step(z, i, params[idx]), params, idx + 1);
        if a != 0 {
            println!("b {} {} {} {}", idx, i, a, z);
            return 10_isize.pow((14 - idx) as u32) * i + a;
        } 
    }
    return 0;
}

fn run(steps: &Vec<[isize; 3]>) {
    let a = find_number(0, &steps, 0);
    println!("{}", a);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines: Vec<&str> = input.lines().collect();

    let mut steps = Vec::new();

    for i in 0..14 {
        let a = lines[18 * i + 4].split(' ').last().unwrap().parse().unwrap();
        let b = lines[18 * i + 5].split(' ').last().unwrap().parse().unwrap();
        let c = lines[18 * i + 15].split(' ').last().unwrap().parse().unwrap();
        steps.push([a, b, c]);
    }

    match version.as_str() {
        "a" => run(&steps),
        "b" => run(&steps),
        _ => panic!("Invalid input")
    }
}
