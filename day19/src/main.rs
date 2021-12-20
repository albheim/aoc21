use std::env;
use std::fs;

fn rotate(p: [isize; 3], rot: usize) -> [isize; 3] {
    // rotation based on which ends up in positive x direction, then four up configs from that
    match rot {
        // x pos
        0 => [p[0], p[1], p[2]],
        1 => [p[0], -p[2], p[1]],
        2 => [p[0], -p[1], -p[2]],
        3 => [p[0], p[2], -p[1]],
        // x neg
        4 => [-p[0], -p[1], p[2]],
        5 => [-p[0], -p[2], -p[1]],
        6 => [-p[0], p[1], -p[2]],
        7 => [-p[0], p[2], p[1]],
        // y pos
        8 => [p[1], -p[0], p[2]],
        9 => [p[1], -p[2], -p[0]],
        10 => [p[1], p[0], -p[2]],
        11 => [p[1], p[2], p[0]],
        // y neg
        12 => [-p[1], p[0], p[2]],
        13 => [-p[1], -p[2], p[0]],
        14 => [-p[1], -p[0], -p[2]],
        15 => [-p[1], p[2], -p[0]],
        // z pos
        16 => [p[2], p[1], -p[0]],
        17 => [p[2], p[0], p[1]],
        18 => [p[2], -p[1], p[0]],
        19 => [p[2], -p[0], -p[1]],
        // z neg
        20 => [-p[2], p[1], p[0]],
        21 => [-p[2], -p[0], p[1]],
        22 => [-p[2], -p[1], -p[0]],
        23 => [-p[2], p[0], -p[1]],
        _ => panic!("invalid rotation")
    }
}

fn diff(a: [isize; 3], b: [isize; 3]) -> [isize; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn add(a: [isize; 3], b: [isize; 3]) -> [isize; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn check_overlap(a: &mut Vec<[isize; 3]>, b: &Vec<[isize; 3]>) -> [isize; 3] {
    // A is always in base orientation, new probes are added to it with time
    // Should be possible to shorten this
    for i in 0..a.len() { //[..a.len()-11] {
        for j in 0..b.len() {
            for rot in 0..24 {
                let mut count = 0;
                let offset = diff(a[i], rotate(b[j], rot));
                for l in 0..b.len() {
                    let b_rot_offset = add(offset, rotate(b[l], rot));
                    for k in 0..a.len() { //[..a.len()-11] {
                        if a[k][0] == b_rot_offset[0] && a[k][1] == b_rot_offset[1] && a[k][2] == b_rot_offset[2] {
                            // println!("at {:?} for rot {} against {:?}", a[k], rot, b[l]);
                            count += 1;
                            break;
                        }
                    }
                }
                // println!("{}", count);
                if count >= 12 {
                    // figure out transform and save?
                    // println!("transform {:?}", offset);
                    // println!("asdf");
                    for l in 0..b.len() {
                        let b_rot_offset = add(offset, rotate(b[l], rot));
                        let mut found = false;
                        for k in 0..a.len() { //[..a.len()-11] {
                            if a[k][0] == b_rot_offset[0] && a[k][1] == b_rot_offset[1] && a[k][2] == b_rot_offset[2] {
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            a.push(b_rot_offset);
                        }
                    }
                    return offset;
                }
            }
        }
    }
    return [0, 0, 0];
}

fn run_a(scanners: &mut Vec<Vec<[isize; 3]>>) {
    let mut base = scanners[0].clone();
    let mut idxs: Vec<usize> = (1..scanners.len()).collect();
    let mut i = 0;
    //println!("{}", check_overlap(&mut base, &scanners[1]));
    loop {
        i %= idxs.len();
        println!("checking {}", idxs[i]);
        let scanner_pos = check_overlap(&mut base, &scanners[idxs[i]]);
        if scanner_pos[0] != 0 || scanner_pos[1] != 0 || scanner_pos[2] != 0 {
            println!("removed");
            idxs.remove(i);
            if idxs.len() == 0 {
                break;
            }
        } else {
            i += 1;
        }
    }
    println!("{}", base.len());
}

fn run_b(scanners: &Vec<Vec<[isize; 3]>>) {
    let mut base = scanners[0].clone();
    let mut idxs: Vec<usize> = (1..scanners.len()).collect();
    let mut i = 0;
    let mut scanner_positions = vec![[0, 0, 0]];
    //println!("{}", check_overlap(&mut base, &scanners[1]));
    loop {
        i %= idxs.len();
        println!("checking {}", idxs[i]);
        let scanner_pos = check_overlap(&mut base, &scanners[idxs[i]]);
        if scanner_pos[0] != 0 || scanner_pos[1] != 0 || scanner_pos[2] != 0 {
            scanner_positions.push(scanner_pos);
            println!("removed");
            idxs.remove(i);
            if idxs.len() == 0 {
                break;
            }
        } else {
            i += 1;
        }
    }
    let mut maxlen = 0;
    for i in 0..scanner_positions.len() {
        for j in 0..scanner_positions.len() {
            let dist = (scanner_positions[i][0] - scanner_positions[j][0]).abs() + 
                (scanner_positions[i][1] - scanner_positions[j][1]).abs() + 
                (scanner_positions[i][2] - scanner_positions[j][2]).abs();
            if dist > maxlen {
                maxlen = dist;
            }
        }
    }
    println!("{}", base.len());
    println!("{}", maxlen);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let mut scanners: Vec<Vec<[isize; 3]>> = fs::read_to_string("input.txt")
        .expect("Could not read file")
        .split("\n\n")
        .map(|probe_chunk| probe_chunk.lines().skip(1)
            .map(|line| {
                let pos: Vec<isize> = line.split(',').map(|num| num.parse().unwrap()).collect();
                [pos[0], pos[1], pos[2]]
            }).collect()
        ).collect();

    match version.as_str() {
        "a" => run_a(&mut scanners),
        "b" => run_b(&mut scanners),
        "test" => run_a(&mut vec![vec![[1, 1, 1], [-1, 2, 3]], vec![[-1, -1, -1], [3, 3, 3]]]),
        _ => panic!("Invalid input")
    }
}
