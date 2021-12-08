use std::fs;
use std::env;

fn run_a(input: &String) {
    let appearances: usize = input.lines()
        .map(|line| line.split_once('|').unwrap().1.split_whitespace()
            .map(|word| match word.len() {
                2 => 1,
                4 => 1,
                3 => 1,
                7 => 1,
                _ => 0
            }).sum::<usize>()
        )
        .sum();

    println!("{}", appearances);
}

fn map_to_number(num: &str, mapping: &[char; 7]) -> usize {
    match num.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        5 => if num.contains(mapping[1]) {
                5
            } else if num.contains(mapping[5]) {
                3
            } else {
                2
            },
        6 => if !num.contains(mapping[3]) {
                0
            } else if num.contains(mapping[2]) {
                9
            } else {
                6
            },
        7 => 8,
        _ => panic!("Something is wrong")
    }
}

fn process_line(line: &str) -> usize {
    let (a, b) = line.split_once('|').unwrap();

    // Same numbering as in task
    let mut segments = ['0'; 7];

    // String len to string
    let mut nums: Vec<Vec<&str>> = vec![Vec::new(); 8];
    for num in a.split_whitespace() {
        let idx: usize = num.len();
        nums[idx].push(num);
    }
    
    // Check one (two segments, only in list)
    let one: Vec<char> = nums[2][0].chars().collect();
    // Compare to number 0, 6, 9
    let count = nums[6].iter().filter(|num| num.contains(one[0])).count();
    if count == 3 {
        segments[2] = one[1];
        segments[5] = one[0];
    } else  {
        segments[2] = one[0];
        segments[5] = one[1];
    }

    // Check seven (three segments, only in list)
    let seven = nums[3][0];
    for c in seven.chars() {
        if !one.contains(&c) {
            segments[0] = c;
            break;
        }
    }

    // Check 4 (four segments, only in list)
    let four = nums[4][0];
    let in_four_not_in_one: Vec<char> = four.chars().filter(|c| !one.contains(c)).collect();
    // Compare to number 2, 3, 5
    let count = nums[5].iter().filter(|num| num.contains(in_four_not_in_one[0])).count();
    if count == 3 {
        segments[1] = in_four_not_in_one[1];
        segments[3] = in_four_not_in_one[0];
    } else  {
        segments[1] = in_four_not_in_one[0];
        segments[3] = in_four_not_in_one[1];
    }

    // Last 2 segments
    let last_2_seg: Vec<char> = "abcdefg".chars().filter(|c| !segments.contains(c)).collect();
    // Compare to number 0, 6, 9
    let count = nums[6].iter().filter(|num| num.contains(last_2_seg[0])).count();
    if count == 3 {
        segments[4] = last_2_seg[1];
        segments[6] = last_2_seg[0];
    } else  {
        segments[4] = last_2_seg[0];
        segments[6] = last_2_seg[1];
    }

    b.split_whitespace().enumerate().map(|(i, num)| usize::pow(10, 3 - i as u32) * map_to_number(&num, &segments)).sum()
}

fn run_b(input: &String) {
    let total: usize = input.lines()
        .map(|line| process_line(line))
        .sum();

    println!("{}", total);
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
