use std::env;
use std::fs;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug, Default, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn subtract(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x, 
            y: self.y - other.y, 
            z: self.z - other.z
        }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x, 
            y: self.y + other.y, 
            z: self.z + other.z
        }
    }

    fn dist(&self, other: &Self) -> u64 {
        let a = self.subtract(other);
        (a.x.abs() + a.y.abs() + a.z.abs()) as u64
    }

    fn rotation(&self, rot: u64) -> Self {
        // rotation based on which ends up in positive x direction, then four up configs from that
        match rot {
            // x pos
            0 => Self { x: self.x, y: self.y, z: self.z},
            1 => Self { x: self.x, y: -self.z, z: self.y},
            2 => Self { x: self.x, y: -self.y, z: -self.z},
            3 => Self { x: self.x, y: self.z, z: -self.y},
            // x neg
            4 => Self { x: -self.x, y: -self.y, z: self.z},
            5 => Self { x: -self.x, y: -self.z, z: -self.y},
            6 => Self { x: -self.x, y: self.y, z: -self.z},
            7 => Self { x: -self.x, y: self.z, z: self.y},
            // y pos
            8 => Self { x: self.y, y: -self.x, z: self.z},
            9 => Self { x: self.y, y: -self.z, z: -self.x},
            10 => Self { x: self.y, y: self.x, z: -self.z},
            11 => Self { x: self.y, y: self.z, z: self.x},
            // y neg
            12 => Self { x: -self.y, y: self.x, z: self.z},
            13 => Self { x: -self.y, y: -self.z, z: self.x},
            14 => Self { x: -self.y, y: -self.x, z: -self.z},
            15 => Self { x: -self.y, y: self.z, z: -self.x},
            // z pos
            16 => Self { x: self.z, y: self.y, z: -self.x},
            17 => Self { x: self.z, y: self.x, z: self.y},
            18 => Self { x: self.z, y: -self.y, z: self.x},
            19 => Self { x: self.z, y: -self.x, z: -self.y},
            // z neg
            20 => Self { x: -self.z, y: self.y, z: self.x},
            21 => Self { x: -self.z, y: -self.x, z: self.y},
            22 => Self { x: -self.z, y: -self.y, z: -self.x},
            23 => Self { x: -self.z, y: self.x, z: -self.y},
            _ => panic!("invalid rotation")
        }
    }
}

struct Scanner {
    probes: Vec<Position>, // List of probe positions
    pos: Position,
    fingerprint: Vec<u64>, // Sorted list of internal distances
}

impl Scanner {
    // If a scanner can match it will have at least 12 + 11 + ... + 1 = 78
    // internal distances matching eachother.
    fn can_match(&self, other: &Scanner) -> bool {
        let mut idx1 = 0; 
        let mut idx2 = 0;
        let mut found = 0;
        let target = 78; // 12 + 11 + ... + 1

        while idx1 < self.fingerprint.len() && idx2 < other.fingerprint.len() && found < target {
            if self.fingerprint[idx1] < other.fingerprint[idx2] {
                idx1 += 1;
            } else if self.fingerprint[idx1] > other.fingerprint[idx2] {
                idx2 += 1;
            } else {
                found += 1;
                idx1 += 1;
                idx2 += 1;
            }
        }

        return found >= target; 
    }

    // If it can match, we do the more expensive look to find the actual 
    // transformation needed.
    fn find_transform(&self, other: &Scanner) -> (Position, u64) {
        for i in 0..self.probes.len() { 
            for j in 0..other.probes.len() {
                for rot in 0..24 {
                    let mut count = 0;
                    let offset = self.probes[i].subtract(&other.probes[j].rotation(rot));
                    let mut matched = Vec::new();
                    for l in 0..other.probes.len() {
                        let updated_probe_position = offset.add(&other.probes[l].rotation(rot));
                        for k in 0..self.probes.len() { 
                            if self.probes[k] == updated_probe_position {
                                matched.push(self.probes[k]);
                                count += 1;
                                break;
                            }
                        }
                    }
                    if count >= 12 {
                        return (offset, rot);
                    }
                }
            }
        }
        panic!("Error, should not happen");
    }
}

fn match_scanners(scanners: &mut Vec<Scanner>) {
    let mut added = vec![false; scanners.len()];
    added[0] = true;

    let mut i = 0;
    loop {
        for j in 0..scanners.len() {
            if !added[j] && i != j && scanners[i].can_match(&scanners[j]) {
                added[j] = true;
                let (offset, rot) = scanners[i].find_transform(&scanners[j]);
                // Rotate all probes in j to use same coordinates as i
                for k in 0..scanners[j].probes.len() {
                    scanners[j].probes[k] = offset.add(&scanners[j].probes[k].rotation(rot));
                }
                scanners[j].pos = offset;
            }
        }
        if added.iter().all(|x| *x) {
            break;
        }
        loop {
            i = (i + 1) % scanners.len();
            if added[i] { 
                break;
            }
        }
    }
}

fn run_a(scanners: &mut Vec<Scanner>) {
    match_scanners(scanners);
    let mut unique = HashSet::new();
    for i in 0..scanners.len() {
        for j in 0..scanners[i].probes.len() {
            unique.insert(scanners[i].probes[j]);
        }
    }
    println!("{}", unique.len());
}

fn run_b(scanners: &mut Vec<Scanner>) {
    match_scanners(scanners);
    let mut maxdist = 0;
    for i in 0..scanners.len() {
        for j in (i + 1)..scanners.len() {
            let dist = scanners[i].pos.dist(&scanners[j].pos);
            if dist > maxdist {
                maxdist = dist;
            }
        }
    }
    println!("{}", maxdist);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let mut scanners: Vec<Scanner> = fs::read_to_string("input.txt")
        .expect("Could not read file")
        .split("\n\n")
        .map(|probe_chunk| {
            let probes: Vec<Position> = probe_chunk.lines().skip(1)
            .map(|line| {
                let pos: Vec<i64> = line.split(',').map(|num| num.parse().unwrap()).collect();
                Position { x: pos[0], y: pos[1], z: pos[2] }
            }).collect();
            let mut fingerprint = Vec::new();
            for i in 0..probes.len() {
                for j in 0..i {
                    fingerprint.push(probes[i].dist(&probes[j]))
                }
            }
            fingerprint.sort();
            Scanner { probes, pos: Position { x: 0, y: 0, z: 0}, fingerprint }
        }).collect();

    match version.as_str() {
        "a" => run_a(&mut scanners),
        "b" => run_b(&mut scanners),
        _ => panic!("Invalid input")
    }
}
