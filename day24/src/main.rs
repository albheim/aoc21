use std::env;
use std::fs;
use std::cmp;

/*
steps = [(a0, b0, c0), ..., (a13, b13, c13)]
num = w0 w1 w2 ... w13

z0 = 0
z(i+1) = if zi % 26 + bi == wi { floor(zi / ai) } else { floor(zi / ai) * 26 + wi + ci }

We have a is either 1 or 26, and there are seven each, always need to go down when possible.
Each one where a is 1 will multiply by 26 and add something less than 26.
Each one where a is 26 will divide with floor, and thus reverse one of the previous steps.
There is a solution iff each step with 26 can undo a step with 1, and this solution assume the input is such.

Since 
wi + ci < 26 
for all values given, we know that all steps will be reversable since the floor division when params[0] is 26
will remove last wi + ci, so if all inputs that can fulfill
input = zi % 26 + bi
does so it will finish correctly.
This means that 
zx = (...) * 26 + wi + ci
and the corresponding input is then
wx = wi + ci

Use a stack and push on ai = 1, pop otherwise. Match the digits and fix it up.
*/

fn run(steps: &Vec<[i64; 3]>, compfun: &dyn Fn(i64) ->  i64) {
    let mut stack = Vec::new();
    let mut num = [0; 14];
    for i in 0..steps.len() {
        if steps[i][0] == 1 {
            stack.push((i, steps[i][2]))
        } else {
            let (idx, val) = stack.pop().expect("Input did not look as expected");
            let d = val + steps[i][1];
            num[i] = compfun(-d);
            num[idx] = compfun(d);
        }
    }
    let num = num.iter().fold(0, |tot, digit| tot * 10 + digit);
    println!("{}", num);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines: Vec<&str> = input.lines().collect();

    let mut steps = Vec::new();

    for i in 0..14 {
        let a = lines[18 * i + 4].split(' ').last().unwrap().parse().unwrap();
        let b = lines[18 * i + 5].split(' ').last().unwrap().parse().unwrap();
        let c = lines[18 * i + 15].split(' ').last().unwrap().parse().unwrap();
        steps.push([a, b, c]);
    }

    match version.as_str() {
        "a" => run(&steps, &|x| cmp::min(9, 9-x)),
        "b" => run(&steps, &|x| cmp::max(1, 1-x)),
        _ => panic!("Invalid input")
    }
}
