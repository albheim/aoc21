use std::env;
use std::fs;
use std::collections::HashMap;
use std::cmp;

fn run_a(commands: Vec<(bool, Vec<[i128; 2]>)>) {
    let mut grid: HashMap<(i128, i128, i128), bool> = HashMap::new();
    for (switch, range) in commands {
        for x in cmp::max(-50, range[0][0])..(cmp::min(50, range[0][1]) + 1) {
            for y in cmp::max(-50, range[1][0])..(cmp::min(50, range[1][1]) + 1) {
                for z in cmp::max(-50, range[2][0])..(cmp::min(50, range[2][1]) + 1) {
                    *grid.entry((x, y, z)).or_insert(false) = switch;
                }
            }
        }
    }
    grid.retain(|_, v| *v);
    println!("{}", grid.len());
}

fn volume(range: &Vec<[i128; 2]>) -> i128 {
    range.iter().map(|range| range[1] - range[0]).product()
}

fn overlap(rangea: &Vec<[i128; 2]>, rangeb: &Vec<[i128; 2]>) -> i128 {
    println!("{:?} {:?}", rangea, rangeb);
    let xmin = cmp::max(rangea[0][0], rangeb[0][0]);
    let xmax = cmp::min(rangea[0][1], rangeb[0][1]);
    let ymin = cmp::max(rangea[1][0], rangeb[1][0]);
    let ymax = cmp::min(rangea[1][1], rangeb[1][1]);
    let zmin = cmp::max(rangea[2][0], rangeb[2][0]);
    let zmax = cmp::min(rangea[2][1], rangeb[2][1]);
    let dx = cmp::max(0, xmax - xmin);
    let dy = cmp::max(0, ymax - ymin);
    let dz = cmp::max(0, zmax - zmin);
    return dx * dy * dz;
}

fn run_b(commands: Vec<(bool, Vec<[i128; 2]>)>) {
    let mut count: i128 = 0;
    for i in 0..commands.len() {
        let (switch, range) = &commands[i];
        if *switch {
            count += volume(range);
            println!("adding {}", volume(range));
        }
        // If switch
        for j in 0..i {
            let (switch_old, range_old) = &commands[j];
            let overlap = overlap(range, range_old);
            println!("overlap {}", overlap);
            if overlap > 0 {
                if *switch {
                    if *switch_old {
                        count -= overlap;
                        println!("removing");
                    } else {
                        count += overlap;
                        println!("adding");
                    }
                } else if *switch_old {
                    count -= overlap;
                    println!("removing");
                }
            }
        }
    }

    println!("{}", count);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let commands = fs::read_to_string("test.txt")
        .expect("Could not read file")
        .lines()
        .map(|line| {
            let (switch, region) = line.split_once(" ").unwrap();
            let switch = switch == "on";
            let region = region.split(',')
                .map(|coord| {
                    let (a, b) = coord[2..].split_once("..").unwrap();
                    [a.parse().unwrap(), b.parse().unwrap()]
                }).collect();
            (switch, region)
        }).collect();

    match version.as_str() {
        "a" => run_a(commands),
        "b" => run_b(commands),
        _ => panic!("Invalid input")
    }
}
