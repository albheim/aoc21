use std::env;
use std::fs;

fn getx(a: char) -> usize {
    match a {
        'A' => 3,
        'B' => 5,
        'C' => 7,
        'D' => 9,
        _ => panic!("Invalid value")
    }
}

fn move_down(map: &Vec<Vec<char>>, pos: [usize; 2]) -> Vec<[usize; 4]> {
    let mut x = pos[0];
    let mut y = 1;

    let xtarg = getx(map[pos[0]][pos[1]]);
    let step = if x > xtarg { 1 } else { -1 };

    while map[y][x] == '.' {
        // Precalc would be faster
        if xtarg < x {
            x -= 1;
        } else {
            x += 1;
        }
        if x == xtarg {
            while map[y + 1][x] == '.' {
                y += 1;
            }
            return Vec::from([[pos[0], pos[1], x, y]]);
        }
    }
    return Vec::new();
}

fn move_up(map: &Vec<Vec<char>>, pos: [usize; 2]) -> Vec<[usize; 4]> {
    let mut moves = Vec::new();
    let mut x = pos[0];
    while map[1][x] == '.' {
        if map[2][x] == '#' {
            moves.push([pos[0], pos[1], x, 1]);
        }
        x -= 1;
    }
    x = pos[0];
    while map[1][x] == '.' {
        if x % 2 == 0 {
            moves.push([pos[0], pos[1], x, 1]);
        }
        x += 1;
    }
    moves
}

/* Start and end coords for move */
fn possible_moves() -> Vec<[usize; 4]> {
    let mut moves = Vec::new();
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
    for i in {
        // do change
        // recurse
        // undo change
    }
} 

fn search(map: &mut Vec<Vec<char>>) {
    let mincost = std::usize::MAX_VALUE;
    let types = ['A', 'B', 'C', 'D'];

    
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
