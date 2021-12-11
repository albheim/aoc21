use std::fs;
use std::env;

fn increment(grid: &mut Vec<Vec<(usize,bool)>>, x: usize, y: usize) -> usize {
    if !grid[x][y].1 {
        let n = grid[x][y].0 + 1;
        if n == 10 {
            let mut flashes = 1;
            grid[x][y] = (0, true);
            for i in 0..3 {
                for j in 0..3 {
                    if x + i > 0 && y + j > 0 && x + i <= grid.len() && y + j <= grid[0].len() {
                        flashes += increment(grid, x + i - 1, y + j - 1);
                    }
                }
            }
            return flashes;
        } else {
            grid[x][y] = (n, false);
        }
    }
    return 0;
}

fn reset_changed_status(grid: &mut Vec<Vec<(usize, bool)>>) {
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            grid[x][y] = (grid[x][y].0, false);
        }
    }
}

fn run_a(grid: &mut Vec<Vec<(usize,bool)>>) {
    let mut flashes = 0;
    for _ in 0..100 {
        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                flashes += increment(grid, x, y);
            }
        }
        reset_changed_status(grid);
    }
    println!("{}", flashes);
}

fn run_b(grid: &mut Vec<Vec<(usize,bool)>>) {
    let mut flashes = 0;
    let mut steps = 0;
    loop {
        steps += 1;
        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                flashes += increment(grid, x, y);
            }
        }
        if flashes == grid.len() * grid[0].len() {
            break;
        }
        flashes = 0;
        reset_changed_status(grid);
    }
    println!("{}", steps);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let mut grid: Vec<Vec<(usize,bool)>> = fs::read_to_string("input.txt")
        .expect("Could not read file")
        .lines()
        .map(|line| line.chars()
            .map(|c| (c as usize - 0x30, false)).collect()
        ).collect();

    match version.as_str() {
        "a" => run_a(&mut grid),
        "b" => run_b(&mut grid),
        _ => panic!("Invalid input")
    }
}

