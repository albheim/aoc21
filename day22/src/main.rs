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
    return xmax > xmin && ymax > ymin && zmax > zmin;
}

// Maybe use, but messier code
fn inside(v1: [i128; 6], v2: [i128; 6]) -> bool {
    v1[0] > v2[0] && v1[1] < v2[1] && 
    v1[2] > v2[2] && v1[3] < v2[3] && 
    v1[4] > v2[4] && v1[5] < v2[5] 
}

fn run(commands: Vec<(bool, [i128; 6])>) {
    let mut volumes = Vec::new();
    for (flag, volume) in commands {
        println!("volume {:?} on {}", volume, flag);
        let mut i = 0;
        while i < volumes.len() {
            //println!("i = {}, vollen = {}", i, volumes.len());
            if overlap(volume, volumes[i]) {
                println!("  overlap with {:?}", volumes[i]);
                let vnew = volume;
                let vold = volumes[i];
                // Modify old piece to not overlap, add new piece at end of volumes
                // Cut in x
                if vnew[0] > vold[0] && vnew[0] < vold[1] {
                    println!("  cut1 at x={}", vnew[0]);
                    // Cut vold at vnew[0]
                    volumes[i] = [vold[0], vnew[0] - 1, vold[2], vold[3], vold[4], vold[5]];
                    volumes.push([vnew[0], vold[1], vold[2], vold[3], vold[4], vold[5]]);
                } else if vnew[1] > vold[0] && vnew[1] < vold[1] {
                    println!("  cut2 at x={}", vnew[1]);
                    // Cut vold at vnew[1]
                    volumes[i] = [vnew[1] + 1, vold[1], vold[2], vold[3], vold[4], vold[5]];
                    volumes.push([vold[0], vnew[1], vold[2], vold[3], vold[4], vold[5]]);
                } else if vnew[2] > vold[2] && vnew[2] < vold[3] {
                    println!("  cut1 at y={}", vnew[2]);
                    // Cut vold at vnew[2]
                    volumes[i] = [vold[0], vold[1], vold[2], vnew[2] - 1, vold[4], vold[5]];
                    volumes.push([vold[0], vold[1], vnew[2], vold[3], vold[4], vold[5]]);
                } else if vnew[3] > vold[2] && vnew[3] < vold[3] {
                    println!("  cut2 at y={}", vnew[3]);
                    // Cut vold at vnew[3]
                    volumes[i] = [vold[0], vold[1], vnew[3] + 1, vold[3], vold[4], vold[5]];
                    volumes.push([vold[0], vold[1], vold[2], vnew[3], vold[4], vold[5]]);
                } else if vnew[4] > vold[4] && vnew[4] < vold[5] {
                    println!("  cut1 at z={}", vnew[4]);
                    // Cut vold at vnew[4]
                    volumes[i] = [vold[0], vold[1], vold[2], vold[3], vold[4], vnew[4] - 1];
                    volumes.push([vold[0], vold[1], vold[2], vold[3], vnew[4], vold[5]]);
                } else if vnew[5] > vold[4] && vnew[5] < vold[5] {
                    println!("  cut2 at z={}", vnew[5]);
                    // Cut vold at vnew[3]
                    volumes[i] = [vold[0], vold[1], vold[2], vold[3], vnew[5] + 1, vold[5]];
                    volumes.push([vold[0], vold[1], vold[2], vnew[3], vold[4], vnew[5]]);
                } else {
                    // If overlap, but no cut, volume[i] is contained in volume and can thus be removed
                    volumes.remove(i); 
                    println!("  no more cut");
                }
            }
            i += 1;
        }
        // If turned on add whole new volume, all overlaps has been removed from already existing
        if flag {
            volumes.push(volume);
        }
        let vol: i128 = volumes.iter().map(|v| (v[1] - v[0] + 1) * (v[3] - v[2] + 1) * (v[5] - v[4] + 1)).sum();
        println!("current volume {}", vol);
        println!("volumes {:?}", volumes);
    }

    let vol: i128 = volumes.iter().map(|v| (v[1] - v[0] + 1) * (v[3] - v[2] + 1) * (v[5] - v[4] + 1)).sum();
    println!("{}", vol);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let commands = fs::read_to_string("test2.txt")
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
        "a" => {
            //commands.retain(|(flag, v)| v.iter().all(|c| c.abs() <= 50));
            run(commands)
        },
        "b" => run(commands),
        _ => panic!("Invalid input")
    }
}
