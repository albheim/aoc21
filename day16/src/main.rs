use std::fs;
use std::env;

/*
FIX add bit-vec crate
*/

fn get_bin(data: &Vec<usize>, idx: &mut usize, len: usize) -> usize {
    let mut val = 0;
    for i in 0..len {
        val |= data[*idx + i] << (len - i - 1);
    }
    *idx += len;
    return val;
}

fn read_packet_versions(data: &Vec<usize>, idx: &mut usize) -> usize {
    let version = get_bin(data, idx, 3);
    let packet_id = get_bin(data, idx, 3);
    match packet_id {
        0b100 => {
            while data[*idx] == 1 {
                *idx += 5;
            }
            *idx += 5;
            version
        },
        _ => {
            let mut tot = version;
            if data[*idx] == 0 {
                *idx += 1;
                let mut n = get_bin(data, idx, 15);
                n += *idx;
                while *idx < n {
                    tot += read_packet_versions(data, idx);
                }
            } else {
                *idx += 1;
                let n = get_bin(data, idx, 11);
                for _ in 0..n {
                    tot += read_packet_versions(data, idx);
                }
            }
            tot
        },
    }
}

fn run_a(input: &str) {
    let mut data = Vec::new();
    for i in 0..input.len() {
        let val = usize::from_str_radix(&input[i..i+1], 16).unwrap();
        data.push((val & 0b1000) >> 3);
        data.push((val & 0b0100) >> 2);
        data.push((val & 0b0010) >> 1);
        data.push((val & 0b0001) >> 0);
    }
    let mut idx = 0;
    let tot = read_packet_versions(&data, &mut idx);
    println!("{}", tot);
}

fn parse_sub_packets(data: &Vec<usize>, idx: &mut usize) -> Vec<usize> {
    let mut values = Vec::new();
    *idx += 1;
    if data[*idx-1] == 0 {
        let mut n = get_bin(data, idx, 15);
        n += *idx;
        while *idx < n {
            values.push(parse_packet(data, idx));
        }
    } else {
        let n = get_bin(data, idx, 11);
        for _ in 0..n {
            values.push(parse_packet(data, idx));
        }
    }
    return values;
}

fn parse_packet(data: &Vec<usize>, idx: &mut usize) -> usize {
    let _version = get_bin(data, idx, 3);
    let packet_id = get_bin(data, idx, 3);
    if packet_id == 0b100 {
        let mut tot = 0;
        while data[*idx] == 1 {
            *idx += 1;
            tot = (tot << 4) | get_bin(data, idx, 4);
        }
        *idx += 1;
        tot = (tot << 4) | get_bin(data, idx, 4);
        tot
    } else {
        let nums = parse_sub_packets(data, idx);
        let val = match packet_id {
            0b000 => nums.iter().sum(),
            0b001 => nums.iter().product(),
            0b010 => *nums.iter().min().unwrap(),
            0b011 => *nums.iter().max().unwrap(),
            0b101 => (nums[0] > nums[1]) as usize,
            0b110 => (nums[0] < nums[1]) as usize,
            0b111 => (nums[0] == nums[1]) as usize,
            _ => panic!("Invalid operation")
        };
        val
    }
}

fn run_b(input: &str) {
    let mut data = Vec::new();
    for i in 0..input.len() {
        let val = usize::from_str_radix(&input[i..i+1], 16).unwrap();
        data.push((val & 0b1000) >> 3);
        data.push((val & 0b0100) >> 2);
        data.push((val & 0b0010) >> 1);
        data.push((val & 0b0001) >> 0);
    }
    let val = parse_packet(&data, &mut 0);
    println!("{}", val);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input = fs::read_to_string("input.txt").expect("Could not read file");

    match version.as_str() {
        "a" => run_a(&input),
        "b" => run_b(&input),
        "test" => run_b("38006F45291200"),
        _ => panic!("Invalid input")
    }
}

