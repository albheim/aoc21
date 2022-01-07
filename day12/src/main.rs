use std::fs;
use std::env;
use std::collections::HashMap;

fn dfs(graph: &HashMap<&str, Vec<&str>>, visited: &mut HashMap<&str, bool>, current: &str, double_visit_used: bool) -> usize {
    // Save current state for reset later
    let isvisited = *visited.get(current).unwrap();

    // Cases where we can't continue
    if isvisited && (double_visit_used || current == "start") {
        return 0;
    } else if current == "end" {
        return 1;
    }

    let mut n_paths = 0;
    // Update if we used extra visit this time
    let double_visit_used = double_visit_used || isvisited;

    // Only update visited flag for lowercase
    let update = current == current.to_lowercase();
    if update {
        *visited.get_mut(current).unwrap() = true;
    }
    // Loop over edges and count paths
    for node in graph.get(current).unwrap() {
        n_paths += dfs(graph, visited, node, double_visit_used);
    }
    // Reset visited flag
    if update {
        *visited.get_mut(current).unwrap() = isvisited;
    }
    return n_paths;
}

fn run(graph: &HashMap<&str, Vec<&str>>, visited: &mut HashMap<&str, bool>, double_visit_used: bool) {
    let n_paths = dfs(graph, visited, "start", double_visit_used);
    println!("{}", n_paths);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut visited: HashMap<&str, bool> = HashMap::new();
    
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    
    for line in input.lines() {
        let (a, b) = line.split_once("-").unwrap();
        visited.insert(a, false);
        visited.insert(b, false);
        let item = graph.entry(a).or_insert_with(|| Vec::new());
        item.push(b); 
        let item = graph.entry(b).or_insert_with(|| Vec::new());
        item.push(a); 
    }

    match version.as_str() {
        "a" => run(&graph, &mut visited, true),
        "b" => run(&graph, &mut visited, false),
        _ => panic!("Invalid input")
    }
}
