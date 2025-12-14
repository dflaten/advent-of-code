use clap::Parser;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Parser)]
#[command(name = "solutions_runner")]
#[command(about = "Reads from a file for input, determines solutions, and writes solution to another file.", long_about = None)]
struct Args {
    /// Input file path
    #[arg(short, long)]
    input: String,

    // Part to solve (1 or 2)
    #[arg(short, long, default_value_t = 1)]
    part: i32,
}

// given input like:
// aaa: you hhh
// you: bbb ccc
// bbb: ddd eee
// ccc: ddd eee fff
// parse them into a map where the key is the name before the colon
// and the value is a vector of names after the colon
fn parse_input_part_1(input: &str) -> std::collections::HashMap<String, Vec<String>> {
    let mut map = std::collections::HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let values: Vec<String> = parts[1]
                .trim()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            map.insert(key, values);
        }
    }
    map
}
// Find all the unique paths that lead from "you" to "out"
// Return the count of them.
fn solutioner_for_part_1(input: &str) -> String {
    let map = parse_input_part_1(input);
    let mut result = Vec::new();
    let mut stack = vec![("you".to_string(), vec!["you".to_string()])];
    while let Some((current, path)) = stack.pop() {
        if current == "out" {
            result.push(path.clone());
            continue;
        }
        if let Some(neighbors) = map.get(&current) {
            for neighbor in neighbors {
                if !path.contains(neighbor) {
                    let mut new_path = path.clone();
                    new_path.push(neighbor.clone());
                    stack.push((neighbor.clone(), new_path));
                }
            }
        }
    }
    println!("Number of paths found: {}", result.len());
    result.len().to_string()
}

use std::collections::HashMap;
use std::collections::HashSet;

// Parse input into a graph represented as an adjacency list
fn parse_input_part_2_owned(input: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        if let Some((key, values)) = line.split_once(':') {
            let key = key.trim().to_string();
            let values: Vec<String> = values.split_whitespace().map(|v| v.to_string()).collect();
            map.insert(key, values);
        }
    }
    map
}

fn topological_order<'a>(
    start: &'a str,
    graph: &'a HashMap<String, Vec<String>>,
) -> Result<Vec<String>, String> {
    fn dfs<'a>(
        node: &'a str,
        graph: &'a HashMap<String, Vec<String>>,
        visiting: &mut HashSet<&'a str>,
        visited: &mut HashSet<&'a str>,
        order: &mut Vec<String>,
    ) -> Result<(), String> {
        if visiting.contains(node) {
            return Err(format!("Cycle detected at {}", node));
        }
        if visited.insert(node) {
            visiting.insert(node);
            if let Some(neighbors) = graph.get(node) {
                for neighbor in neighbors {
                    dfs(neighbor, graph, visiting, visited, order)?;
                }
            }
            visiting.remove(node);
            order.push(node.to_string());
        }
        Ok(())
    }

    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    let mut order = Vec::new();
    dfs(start, graph, &mut visiting, &mut visited, &mut order)?;
    order.reverse(); // ensure parents appear before children
    Ok(order)
}

// Count all paths from "svr" to "out" that pass through both "dac" and "fft"
// (order agnostic) using DAG dynamic programming.
fn solutioner_for_part_2(input: &str) -> String {
    let graph = parse_input_part_2_owned(input);
    let Ok(topo) = topological_order("svr", &graph) else {
        return "Graph contains a cycle".to_string();
    };

    // DP that carries whether we've seen dac/fft along the path.
    // States are represented as bits: 0b01 => seen dac, 0b10 => seen fft.
    let mut counts: HashMap<&str, [u128; 4]> = HashMap::new();
    counts.insert("svr", [1, 0, 0, 0]);

    for node in &topo {
        if let Some(state_counts) = counts.get(node.as_str()).cloned() {
            if let Some(neighbors) = graph.get(node) {
                for neighbor in neighbors {
                    let new_state_counts = counts.entry(neighbor.as_str()).or_insert([0; 4]);
                    for (state, ways) in state_counts.iter().enumerate() {
                        if *ways == 0 {
                            continue;
                        }
                        let mut new_state = state as u8;
                        if neighbor == "dac" {
                            // Mark dac as seen
                            new_state |= 0b01;
                        }
                        if neighbor == "fft" {
                            // Mark fft as seen
                            new_state |= 0b10;
                        }
                        new_state_counts[new_state as usize] += ways;
                    }
                }
            }
        }
    }

    counts
        .get("out")
        .map(|arr| arr[3])
        .unwrap_or(0)
        .to_string()
}




/// Main function to read input file, process it, and write to output file
/// Uses command line arguments for input and which part you want to solve 
/// Example usage: cargo run -- --i input.txt --p 1
fn main() -> io::Result<()> {
    let args = Args::parse();
    let output: String = "output.txt".to_string();

    let mut input_file = File::open(&args.input)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    let processed = match args.part {
        1 => solutioner_for_part_1(&contents),
        2 => solutioner_for_part_2(&contents),
        _ => "Invalid part specified".to_string(),
    };

    let mut output_file = File::create(&output)?;
    output_file.write_all(processed.as_bytes())?;

    println!("Successfully determined solution {} -> {}", args.input, output);

    Ok(())
}
