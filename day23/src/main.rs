use std::env;
use std::fs;

fn search(map: &mut Vec<Vec<char>>) {
    let mincost = std::usize::MAX_VALUE;
    let types = ['A', 'B', 'C', 'D'];

    // split in two steps, move out and move in

    // for each that can move out
    for i in 0..4 {
        let x = 2 * i + 3;
        let mut correct = true;
        while map[y][x] == '.' {
            y += 1;
        }
        let top = y;
        while map[y][x] != '#' {
            correct &= (map[y][x] == types[i]);
        }
        if !correct {
            let mut xs = x;
            while x > 0
            // do change
            // recurse
            // undo change
        }
    }

    // for each that can move in
    for {
        // do change
        // recurse
        // undo change
    }
    
    if mincost == std::usize::MAX_VALUE {
        mincost = 0;
    }
    return mincost;
}

fn run(map: &mut Vec<Vec<char>>) {
    let mincost = search(map);
    println!("{}", mincost);
}

fn run_b(map: &mut Vec<Vec<char>>) {
    let mut a = "  #D#C#B#A#  ".chars().collect();
    let mut b = "  #D#B#A#C#  ".chars().collect();
    map.insert(3, a);
    map.insert(4, b);
    run(map);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let mut map = fs::read_to_string("test.txt")
        .expect("Could not read file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    match version.as_str() {
        "a" => run(&mut map),
        "b" => run_b(&mut map),
        _ => panic!("Invalid input")
    }
}
