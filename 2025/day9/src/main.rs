use clap::Parser;
use std::collections::HashSet;
use std::sync::Arc;
use std::thread;
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

// Given a a string input, which contains a pair of coordinates on each line
// representing '#' characters in a grid: 
// Find the biggest rectangle you can make using the '#' characters as opposite
// corners of the rectangle. You only need to use two characters as oppisate corners. 
// return the area of the biggest rectangle as a string.
fn solutioner_for_part_1(input: &str) -> String {
    // Deduplicate coordinates to keep the working set lean.
    let mut coords_set = HashSet::<(i64, i64)>::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            if let (Ok(x), Ok(y)) = (
                parts[0].trim().parse::<i64>(),
                parts[1].trim().parse::<i64>(),
            ) {
                coords_set.insert((x, y));
            }
        }
    }

    let coords: Vec<(i64, i64)> = coords_set.into_iter().collect();
    let n = coords.len();
    if n < 2 {
        return "0".to_string();
    }

    // Parallel brute-force over pairs; good enough for a few thousand points,
    // but substantially faster than a single-threaded double loop.
    let coords = Arc::new(coords);
    let threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let chunk_size = (n + threads - 1) / threads;

    let mut handles = Vec::new();
    for chunk_start in (0..n).step_by(chunk_size) {
        let chunk_end = (chunk_start + chunk_size).min(n);
        let coords_ref = Arc::clone(&coords);
        handles.push(thread::spawn(move || {
            let mut local_max: i64 = 0;
            for i in chunk_start..chunk_end {
                let (x1, y1) = coords_ref[i];
                for j in i + 1..coords_ref.len() {
                    let (x2, y2) = coords_ref[j];
                    // Inclusive grid cells: 1x1 rectangle has area 1.
                    let width = (x2 - x1).abs() + 1;
                    let height = (y2 - y1).abs() + 1;
                    let area = width * height;
                    if area > local_max {
                        local_max = area;
                    }
                }
            }
            local_max
        }));
    }

    let max_area = handles
        .into_iter()
        .filter_map(|h| h.join().ok())
        .max()
        .unwrap_or(0);

    max_area.to_string()
}

// Given a a string input, which contains a pair of coordinates on each line
// representing '#' characters in a grid: 
// Find the biggest rectangle you can make using characters inside of the 
// shapes created using the '#' characters as the corners of the shapes.
// For example: 
// ..............
// .......#...#..
// ..............
// ..#....#......
// ..............
// ..#......#....
// ..............
// .........#.#..
// ..............
// That input would result in a shape of: 
// ..............
// .......#XXX#..
// .......XXXXX..
// ..#XXXX#XXXX..
// ..XXXXXXXXXX..
// ..#XXXXXX#XX..
// .........XXX..
// .........#X#..
// ..............
// Where you could make a rectangle out of any of the # or X characters. 
// 24 would be the area of the biggest rectangle you could make from this input.
// return the area of the biggest rectangle as a string.
fn solutioner_for_part_2(input: &str) -> String {
    // Deduplicate while preserving vertex order for polygon vertices.
    let mut seen = HashSet::<(i64, i64)>::new();
    let mut coords: Vec<(i64, i64)> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            if let (Ok(x), Ok(y)) = (
                parts[0].trim().parse::<i64>(),
                parts[1].trim().parse::<i64>(),
            ) {
                if seen.insert((x, y)) {
                    coords.push((x, y));
                }
            }
        }
    }
    let polygon_vertices_count = coords.len();
    if polygon_vertices_count < 3 {
        return "0".to_string();
    }

    // Coordinate compression for efficient scan.
    let mut xs: Vec<i64> = Vec::with_capacity(polygon_vertices_count * 2);
    let mut ys: Vec<i64> = Vec::with_capacity(polygon_vertices_count * 2);
    for &(x, y) in &coords {
        xs.push(x);
        xs.push(x + 1);
        ys.push(y);
        ys.push(y + 1);
    }
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();

    let cols = xs.len() - 1;
    let rows = ys.len() - 1;

    // Polygon edges (closed)
    let mut edges: Vec<((i64, i64), (i64, i64))> = Vec::with_capacity(polygon_vertices_count);
    for i in 0..polygon_vertices_count {
        edges.push((coords[i], coords[(i + 1) % polygon_vertices_count]));
    }

    // Point-in-polygon helper (inclusive of boundary)
    let inside = |px: f64, py: f64, edges: &Vec<((i64, i64), (i64, i64))>| -> bool {
        let mut inside_flag = false;
        for &((x1, y1), (x2, y2)) in edges {
            let (x1f, y1f) = (x1 as f64, y1 as f64);
            let (x2f, y2f) = (x2 as f64, y2 as f64);
            // Boundary check.
            let cross = (x2f - x1f) * (py - y1f) - (y2f - y1f) * (px - x1f);
            if cross.abs() < 1e-9
                && px >= x1f.min(x2f) - 1e-9
                && px <= x1f.max(x2f) + 1e-9
                && py >= y1f.min(y2f) - 1e-9
                && py <= y1f.max(y2f) + 1e-9
            {
                return true;
            }

            let intersects = ((y1f > py) != (y2f > py))
                && (px
                    < (x2f - x1f) * (py - y1f) / (y2f - y1f + f64::EPSILON) + x1f);
            if intersects {
                inside_flag = !inside_flag;
            }
        }
        inside_flag
    };

    // Build filled grid via point-in-polygon on cell lower-left corners (captures boundary).
    let mut filled: Vec<Vec<bool>> = vec![vec![false; rows]; cols];
    for row in 0..rows {
        let y_sample = ys[row] as f64;
        for col in 0..cols {
            let x_sample = xs[col] as f64;
            filled[col][row] = inside(x_sample, y_sample, &edges);
        }
    }

    // Weighted prefix sum for fast area checks.
    let mut prefix: Vec<Vec<i64>> = vec![vec![0; rows + 1]; cols + 1];
    for col in 0..cols {
        for row in 0..rows {
            let cell_area = (xs[col + 1] - xs[col]) * (ys[row + 1] - ys[row]);
            let val = if filled[col][row] { cell_area } else { 0 };
            prefix[col + 1][row + 1] =
                prefix[col][row + 1] + prefix[col + 1][row] - prefix[col][row] + val;
        }
    }

    // Helper to get area sum of rectangle in compressed grid (inclusive indices).
    let area_sum = |cx1: usize, cx2: usize, cy1: usize, cy2: usize, prefix: &Vec<Vec<i64>>| {
        prefix[cx2 + 1][cy2 + 1] - prefix[cx1][cy2 + 1] - prefix[cx2 + 1][cy1] + prefix[cx1][cy1]
    };

    // Map coordinate to column/row index (lower bound).
    let find_idx = |v: i64, arr: &Vec<i64>| -> usize {
        arr.binary_search(&v).unwrap_or_else(|idx| idx.saturating_sub(1))
    };

    // Exhaustive over input points as opposite corners, ensure rectangle fully filled.
    let mut max_area: i64 = 0;
    for i in 0..polygon_vertices_count {
        let (x1, y1) = coords[i];
        let cx1 = find_idx(x1, &xs);
        let cy1 = find_idx(y1, &ys);
        for j in i + 1..polygon_vertices_count {
            let (x2, y2) = coords[j];
            let cx2 = find_idx(x2, &xs);
            let cy2 = find_idx(y2, &ys);

            let (lx, rx) = if cx1 <= cx2 { (cx1, cx2) } else { (cx2, cx1) };
            let (ly, ry) = if cy1 <= cy2 { (cy1, cy2) } else { (cy2, cy1) };

            let width = xs[rx + 1] - xs[lx];
            let height = ys[ry + 1] - ys[ly];
            let target_area = width * height;
            let filled_area = area_sum(lx, rx, ly, ry, &prefix);

            if filled_area == target_area && target_area > max_area {
                max_area = target_area;
            }
        }
    }

    max_area.to_string()
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
