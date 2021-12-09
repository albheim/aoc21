use std::fs;
use std::env;

fn run_a(input: &String) {
    let map: Vec<Vec<usize>> = input.lines()
        .map(|line| line.chars()
            .map(|c| c as usize - 0x30).collect()
        ).collect();

    let mut score = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len(){
            if (row <= 0 || map[row][col] < map[row - 1][col]) &&
                (row >= map.len() - 1 || map[row][col] < map[row + 1][col]) &&
                (col <= 0 || map[row][col] < map[row][col - 1]) &&
                (col >= map[row].len() - 1 || map[row][col] < map[row][col + 1]) {
                score += map[row][col] + 1;
            }
        }
    }
    println!("{}", score);
}

fn check_size(map: &mut Vec<Vec<(usize, bool)>>, x: usize, y: usize) -> usize {
    if map[x][y].0 == 9 || map[x][y].1 {
        return 0;
    }

    let mut total = 1;
    map[x][y] = (map[x][y].0, true);
    if x > 0 {
        total += check_size(map, x - 1, y);
    }
    if x < map.len() - 1 {
        total += check_size(map, x + 1, y);
    }
    if y > 0 {
        total += check_size(map, x, y - 1);
    }
    if y < map.len() - 1 {
        total += check_size(map, x, y + 1);
    }
    return total
}

fn run_b(input: &String) {
    let mut map: Vec<Vec<(usize, bool)>> = input.lines()
        .map(|line| line.chars()
            .map(|c| (c as usize - 0x30, false)).collect()
        ).collect();

    let mut sizes = [0; 3];
    let mut size;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            size = check_size(&mut map, x, y);
            for i in 0..sizes.len() {
                if size > sizes[i] {
                    let tmp = sizes[i];
                    sizes[i] = size;
                    size = tmp;
                }
            }
        }
    }

    println!("{}", sizes[0] * sizes[1] * sizes[2]);
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

