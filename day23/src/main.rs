use std::env;
use std::fs;
use std::collections::HashMap;

fn getx(a: char) -> usize {
    match a {
        'A' => 3,
        'B' => 5,
        'C' => 7,
        'D' => 9,
        _ => panic!("Invalid value")
    }
}

fn move_down(map: &Vec<Vec<char>>, pos: [usize; 2], c: char) -> Vec<[usize; 4]> {
    let mut moves = Vec::new();
    let mut x = pos[0];
    while map[1][x] == '.' {
        x -= 1;
    }
    if map[1][x] == c {
        moves.push([x, 1, pos[0], pos[1]]);
    }
    x = pos[0];
    while map[1][x] == '.' {
        x += 1;
    }
    if map[1][x] == c {
        moves.push([x, 1, pos[0], pos[1]]);
    }
    return moves;
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
        if map[2][x] == '#' {
            moves.push([pos[0], pos[1], x, 1]);
        }
        x += 1;
    }
    return moves;
}

/* Start and end coords for move */
fn possible_moves(map: &Vec<Vec<char>>) -> Vec<[usize; 4]> {
    let mut moves = Vec::new();

    let chars = ['A', 'B', 'C', 'D'];
    for i in 0..chars.len() {
        let c = chars[i];
        let x = getx(c);
        let mut y = 2;
        let mut correct = true;
        while map[y][x] == '.' {
            y += 1;
        }
        let top = y;
        while map[y][x] != '#' {
            correct &= map[y][x] == c;
            y += 1;
        }
        //print!("in {} pocket ", c);
        if !correct { // We can move out of this one
            //println!("we can move up");
            moves.append(&mut move_up(map, [x, top]));
        } else if top > 2 { // Or we can move in to this one
            //println!("we can move down");
            moves.append(&mut move_down(map, [x, top - 1], c));
        }
    }

    //println!("moves {:?}", moves);
    moves
} 

// Keep a done vec, counting from -4 to 4
fn is_done(map: &mut Vec<Vec<char>>) -> bool {
    for c in ['A', 'B', 'C', 'D'] {
        let x = getx(c);
        let mut y = 2;
        while map[y][x] != '#' {
            if map[y][x] != c {
                return false;
            }
            y += 1;
        }
    }
    return true;
}

fn manhattandist(step: [usize; 4]) -> usize {
    let mut tot = 0;
    if step[0] > step[2] {
        tot += step[0] - step[2];
    } else {
        tot += step[2] - step[0];
    }
    if step[1] > step[3] {
        tot += step[1] - step[3];
    } else {
        tot += step[3] - step[1];
    }
    return tot;
}

fn search(map: &mut Vec<Vec<char>>, lookup: &mut HashMap<String, usize>) -> usize {
    let oldkey = to_key(map);
    let mut mincost = if lookup.contains_key(&oldkey) {
        lookup[&oldkey]
    } else {
        1000 * 1000
    };
    //println!("From state: \n{:?}\n{:?}\n{:?}\n{:?}\n{:?}", map[0], map[1], map[2], map[3], map[4]);
    for step in possible_moves(map) {
        let steps = manhattandist(step);
        let move_cost = steps * 10_usize.pow(((getx(map[step[1]][step[0]]) >> 1) - 1) as u32);
        //println!("step: {:?}, cost: {}", step, move_cost);

        // make move
        map[step[3]][step[2]] = map[step[1]][step[0]];
        map[step[1]][step[0]] = '.';
        //println!("Move: {:?}", step);

        // Fingerprint for this state
        let key = to_key(map);

        // Recurse search
        let rest_cost = if lookup.contains_key(&key) {
            lookup[&key]
        } else {
            search(map, lookup)
        };

        let tot_cost = move_cost + rest_cost;
        if tot_cost < mincost {
            mincost = tot_cost;
        }

        // undo move
        map[step[1]][step[0]] = map[step[3]][step[2]];
        map[step[3]][step[2]] = '.';
    }
    lookup.insert(oldkey, mincost);
    //println!("mincost: {}", mincost);
    return mincost;
}

fn to_key(map: &Vec<Vec<char>>) -> String {
    let mut a: String = map[1][1..12].iter().collect();
    let mut y = 2;
    while map[y][3] != '#' {
        a.push(map[y][3]);
        a.push(map[y][5]);
        a.push(map[y][7]);
        a.push(map[y][9]);
        y += 1;
    }
    a
}

fn run(map: &mut Vec<Vec<char>>) {
    let mut lookup = HashMap::new();
    let finished_key = format!("{}{}", ".".repeat(11), "ABCD".repeat(map.len() - 3));
    lookup.insert(finished_key, 0);
    let mincost = search(map, &mut lookup);
    println!("{}", mincost);
}

fn run_b(map: &mut Vec<Vec<char>>) {
    let a = "  #D#C#B#A#  ".chars().collect();
    let b = "  #D#B#A#C#  ".chars().collect();
    map.insert(3, a);
    map.insert(4, b);
    run(map);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let mut map = fs::read_to_string("input.txt")
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
