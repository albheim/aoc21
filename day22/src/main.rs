use std::env;
use std::fs;
use std::cmp;

fn overlap(v1: [i128; 6], v2: [i128; 6]) -> bool {
    let xmin = cmp::max(v1[0], v2[0]);
    let xmax = cmp::min(v1[1], v2[1]);
    let ymin = cmp::max(v1[2], v2[2]);
    let ymax = cmp::min(v1[3], v2[3]);
    let zmin = cmp::max(v1[4], v2[4]);
    let zmax = cmp::min(v1[5], v2[5]);
    return xmax >= xmin && ymax >= ymin && zmax >= zmin;
}

fn run(commands: Vec<(bool, [i128; 6])>) {
    let mut volumes = Vec::new();
    for (flag, volume) in commands {
        let mut i = 0;
        let mut old_len = volumes.len(); // Only loop to curren tlen
        while i < old_len {
            if overlap(volume, volumes[i]) {
                let mut overlap = volumes[i]; // overlap is copy
                // Modify old piece to not overlap, add new piece at end of volumes
                // Cut in x
                loop {
                    if volume[0] > overlap[0] && volume[0] <= overlap[1] {
                        // Cut overlap at volume[0]
                        volumes.push([overlap[0], volume[0] - 1, overlap[2], overlap[3], overlap[4], overlap[5]]);
                        overlap[0] = volume[0];
                    } else if volume[1] >= overlap[0] && volume[1] < overlap[1] {
                        // Cut overlap at volume[1]
                        volumes.push([volume[1] + 1, overlap[1], overlap[2], overlap[3], overlap[4], overlap[5]]);
                        overlap[1] = volume[1];
                    } else if volume[2] > overlap[2] && volume[2] <= overlap[3] {
                        // Cut overlap at volume[2]
                        volumes.push([overlap[0], overlap[1], overlap[2], volume[2] - 1, overlap[4], overlap[5]]);
                        overlap[2] = volume[2];
                    } else if volume[3] >= overlap[2] && volume[3] < overlap[3] {
                        // Cut overlap at volume[3]
                        volumes.push([overlap[0], overlap[1], volume[3] + 1, overlap[3], overlap[4], overlap[5]]);
                        overlap[3] = volume[3];
                    } else if volume[4] > overlap[4] && volume[4] <= overlap[5] {
                        // Cut overlap at volume[4]
                        volumes.push([overlap[0], overlap[1], overlap[2], overlap[3], overlap[4], volume[4] - 1]);
                        overlap[4] = volume[4];
                    } else if volume[5] >= overlap[4] && volume[5] < overlap[5] {
                        // Cut overlap at volume[3]
                        volumes.push([overlap[0], overlap[1], overlap[2], overlap[3], volume[5] + 1, overlap[5]]);
                        overlap[5] = volume[5];
                    } else {
                        // If overlap, but no cut, volume[i] is contained in volume and can thus be removed
                        volumes.remove(i); 
                        old_len -= 1;
                        break;
                    }
                }
            } else {
                i += 1; 
            }
        }
        // If turned on add whole new volume, all overlaps has been removed from already existing
        if flag {
            volumes.push(volume);
        }
    }

    let vol: i128 = volumes.iter().map(|v| (v[1] - v[0] + 1) * (v[3] - v[2] + 1) * (v[5] - v[4] + 1)).sum();
    println!("{}", vol);
}

fn run_a(commands: Vec<(bool, [i128; 6])>) {
    let mut subcommands = Vec::new();
    for command in commands {
        if command.1.iter().all(|c| c.abs() <= 50) {
            subcommands.push(command);
        } 
    }
    run(subcommands);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let commands: Vec<(bool, [i128; 6])> = fs::read_to_string("input.txt")
        .expect("Could not read file")
        .lines()
        .map(|line| {
            let (switch, region) = line.split_once(" ").unwrap();
            let switch = switch == "on";
            let region: Vec<[i128; 2]> = region.split(',')
                .map(|coord| {
                    let (a, b) = coord[2..].split_once("..").unwrap();
                    [a.parse().unwrap(), b.parse().unwrap()]
                }).collect();
            (switch, [region[0][0], region[0][1], region[1][0], region[1][1], region[2][0], region[2][1]])
        }).collect();

    match version.as_str() {
        "a" => run_a(commands),
        "b" => run(commands),
        _ => panic!("Invalid input")
    }
}
