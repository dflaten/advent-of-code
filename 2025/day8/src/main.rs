use clap::Parser;
use std::fs::File;
use std::io::{self, Read, Write};
use std::collections::{HashMap, BinaryHeap};

#[derive(Parser, Debug, Clone, PartialEq)]
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
struct OrderedFloat(f64);

impl Eq for OrderedFloat {}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl PartialEq for OrderedFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}


// Each line contains one point in the format "x,y,z"
fn parse_list_of_points(input: &str) -> Vec<(i32, i32, i32)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().trim().parse::<i32>().unwrap();
            let y = parts.next().unwrap().trim().parse::<i32>().unwrap();
            let z = parts.next().unwrap().trim().parse::<i32>().unwrap();
            (x, y, z)
        })
        .collect()
}

// Given a list of 3D points, find the sizes of connected components
// where edges are formed between points that are among the `num_edges` shortest distances
// Return the product of the sizes of the three largest components
fn solutioner_for_part_1(input: &str) -> String {
    let num_edges: usize = 1000; // Just adjusted this manually from small input, 10 to larger input, 1000 
    let list_of_points = parse_list_of_points(input);
    let n = list_of_points.len();

    // Use a max-heap to track the global `num_edges` shortest edges (undirected)
    let mut min_heap = BinaryHeap::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let p = list_of_points[i];
            let q = list_of_points[j];
            let dx = (p.0 - q.0) as f64;
            let dy = (p.1 - q.1) as f64;
            let dz = (p.2 - q.2) as f64;
            let dist_sq = dx * dx + dy * dy + dz * dz;

            min_heap.push((OrderedFloat(dist_sq), i, j));
            if min_heap.len() > num_edges {
                // Remove the longest edge seen so far since we only want the shortest `num_edges` edges
                min_heap.pop();
            }
        }
    }

    // Extract edges
    let mut edges: Vec<_> = min_heap.into_iter()
        .map(|(OrderedFloat(d), i, j)| (d, i, j))
        .collect();
    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Union-Find structure to find connected components
    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank: Vec<usize> = vec![0; n];

    fn find(x: usize, parent: &mut [usize]) -> usize {
        if parent[x] != x {
            parent[x] = find(parent[x], parent);
        }
        parent[x]
    }

    fn union(x: usize, y: usize, parent: &mut [usize], rank: &mut [usize]) {
        let rx = find(x, parent);
        let ry = find(y, parent);
        if rx != ry {
            if rank[rx] < rank[ry] {
                parent[rx] = ry;
            } else if rank[rx] > rank[ry] {
                parent[ry] = rx;
            } else {
                parent[ry] = rx;
                rank[rx] += 1;
            }
        }
    }

    for (_, i, j) in &edges {
        union(*i, *j, &mut parent, &mut rank);
    }

    // Count circuit sizes
    let mut sizes: HashMap<usize, usize> = HashMap::new();
    for idx in 0..n {
        *sizes.entry(find(idx, &mut parent)).or_insert(0) += 1;
    }

    let mut size_vec: Vec<_> = sizes.values().copied().collect();
    size_vec.sort_unstable_by(|a, b| b.cmp(a));

    size_vec.iter().take(3).product::<usize>().to_string()
}


// Connect all points using closest-first spanning (Kruskal) and
// return the product of the X coordinates of the endpoints of the final merge.
fn solutioner_for_part_2(input: &str) -> String {
    let list_of_points = parse_list_of_points(input);
    let n = list_of_points.len();

    // Build all undirected edges with squared distances
    let mut edges: Vec<_> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let p = list_of_points[i];
            let q = list_of_points[j];
            let dx = (p.0 - q.0) as f64;
            let dy = (p.1 - q.1) as f64;
            let dz = (p.2 - q.2) as f64;
            let dist_sq = dx * dx + dy * dy + dz * dz;

            edges.push((OrderedFloat(dist_sq), i, j));
        }
    }

    // Sort ascending by distance for Kruskal
    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Union-Find structure to connect points 
    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank: Vec<usize> = vec![0; n];

    fn find(x: usize, parent: &mut [usize]) -> usize {
        if parent[x] != x {
            parent[x] = find(parent[x], parent);
        }
        parent[x]
    }

    fn union(x: usize, y: usize, parent: &mut [usize], rank: &mut [usize]) -> bool {
        let rx = find(x, parent);
        let ry = find(y, parent);
        if rx != ry {
            if rank[rx] < rank[ry] {
                parent[rx] = ry;
            } else if rank[rx] > rank[ry] {
                parent[ry] = rx;
            } else {
                parent[ry] = rx;
                rank[rx] += 1;
            }
            return true;
        }
        false
    }

    // Process edges in order, track the last successful connection 
    let mut merges = 0;
    let mut last_merge: Option<(usize, usize)> = None;
    for (_, i, j) in &edges {
        if union(*i, *j, &mut parent, &mut rank) {
            merges += 1;
            last_merge = Some((*i, *j));
            if merges == n - 1 {
                break;
            }
        }
    }

    if let Some((i, j)) = last_merge {
        let last_point = list_of_points[i];
        let second_last_point = list_of_points[j];
        (last_point.0 * second_last_point.0).to_string()
    } else {
        "0".to_string()
    }
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
