use std::env;
use std::fs;
use std::cmp;

fn run_a(mut a: usize, mut b: usize) {
    let mut diestate = 1;

    let mut ascore = 0;
    let mut bscore = 0;

    loop {
        a += (diestate + 1) * 3;
        a = (a - 1) % 10 + 1;
        diestate += 3;
        ascore += a;
        if ascore >= 1000 {
            println!("{}", bscore * (diestate - 1));
            break;
        }
        b += (diestate + 1) * 3;
        b = (b - 1) % 10 + 1;
        diestate += 3;
        bscore += b;
        if bscore >= 1000 {
            println!("{}", ascore * (diestate - 1));
            break;
        }
    }
}

fn play(astate: usize, ascore: usize, bstate: usize, bscore: usize) -> (u128, u128) {
    if bscore >= 21 {
        return (0, 1);
    }
    let mut awins = 0;
    let mut bwins = 0;
    for (v, n) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let newstate = (astate + v - 1) % 10 + 1;
        let newscore = ascore + newstate;
        let wins = play(bstate, bscore, newstate, newscore);
        awins += n * wins.1;
        bwins += n * wins.0;
    }
    return (awins, bwins);
}

fn run_b(a: usize, b: usize) {
    let wins = play(a, 0, b, 0);
    println!("{}", cmp::max(wins.0, wins.1));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input = fs::read_to_string("input.txt")
        .expect("Could not read file");
    let (a, b) = input.split_once("\n").unwrap();
    let a = a.split_once(": ").unwrap().1.parse().unwrap();
    let b = b.split_once(": ").unwrap().1.parse().unwrap();

    match version.as_str() {
        "a" => run_a(a, b),
        "b" => run_b(a, b),
        _ => panic!("Invalid input")
    }
}
