use std::env;
use std::fs;

fn get_num(image: &Vec<Vec<bool>>, i: isize, j: isize, outside: bool) -> usize {
    let mut n = 0;
    let ilen = image.len() as isize;
    let jlen = image[0].len() as isize;
    for di in -1..2 {
        let itot = i + di;
        for dj in -1..2 {
            let jtot = j + dj;
            n <<= 1;
            if itot >= 0 && itot < ilen && jtot >= 0 && jtot < jlen {
                n |= image[itot as usize][jtot as usize] as usize;
            } else {
                n |= outside as usize;
            }
        }
    }
    return n;
}

fn enhance(image: &mut Vec<Vec<bool>>, enhancer: &Vec<bool>, outside: bool) -> bool{
    let mut newimage = Vec::new();
    for i in 0..image.len()+2 {
        let mut tmp = Vec::new();
        for j in 0..image[0].len()+2 {
            let num = get_num(image, i as isize - 1, j as isize - 1, outside);
            tmp.push(enhancer[num]);
        }
        newimage.push(tmp);
    }
    *image = newimage;
    // Return new outside value
    if outside {
        return enhancer[255];
    } else {
        return enhancer[0];
    }
}

fn run(image: &mut Vec<Vec<bool>>, enhancer: &Vec<bool>, iterations: usize) {
    let mut outside = false;
    for _ in 0..iterations {
        outside = enhance(image, enhancer, outside);
    }
    let mut count = 0;
    for row in image {
        for pixel in row {
            if *pixel {
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
    let (enhancer, image) = input.split_once("\n\n").unwrap();

    let enhancer = enhancer.chars().map(|c| c == '#').collect();

    let mut image = image.lines()
        .map(|line| line.chars()
            .map(|pixel| pixel == '#').collect()
        ).collect();

    match version.as_str() {
        "a" => run(&mut image, &enhancer, 2),
        "b" => run(&mut image, &enhancer, 50),
        _ => panic!("Invalid input")
    }
}
