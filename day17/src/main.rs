use std::env;
use std::fs;
use std::cmp;
use regex::Regex;

fn run_a(_xrange: [isize; 2], yrange: [isize; 2]) {
    let vy = -(yrange[0] + 1);
    println!("{}", vy * (vy + 1) / 2);
}

fn run(mut vx: isize, mut vy: isize, xrange: [isize; 2], yrange: [isize; 2]) -> bool {
    let mut x = 0;
    let mut y = 0;
    while x <= xrange[1] && y >= yrange[0] {
        if x >= xrange[0] && y <= yrange[1] {
            return true;
        }
        x += vx;
        vx = cmp::max(0, vx - 1);
        y += vy;
        vy -= 1;
    }
    return false;
}

fn run_b(xrange: [isize; 2], yrange: [isize; 2]) {
    let vxmin = (-0.5 + (0.25 + 2.0 * xrange[0] as f64).sqrt()).ceil() as isize;
    let vxmax = xrange[1];
    let vymin = yrange[0];
    let vymax = -(yrange[0] + 1);

    let mut count = 0;
    for vx in vxmin..vxmax+1 {
        for vy in vymin..vymax+1 {
            if run(vx, vy, xrange, yrange) {
                count += 1;
            } 
        }
    }
    println!("{}", count);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let re = Regex::new(r"target area: x=(\d+)..(\d+), y=-(\d+)..-(\d+)").unwrap();
    let ans = re.captures(&input).unwrap();
    let xrange = [ans[1].parse::<isize>().unwrap(), ans[2].parse::<isize>().unwrap()];
    let yrange = [-ans[3].parse::<isize>().unwrap(), -ans[4].parse::<isize>().unwrap()];

    match version.as_str() {
        "a" => run_a(xrange, yrange),
        "b" => run_b(xrange, yrange),
        _ => panic!("Invalid input")
    }
}
