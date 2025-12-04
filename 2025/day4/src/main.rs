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
// Converts a string of lines of . and @ characters into a 2D vector of strings
fn convert_string_into_two_d_vector(input: &str) -> Vec<Vec<String>> {
    let mut grid: Vec<Vec<String>> = Vec::new();
    for line in input.lines() {
        let row: Vec<String> = line.chars().map(|c| c.to_string()).collect();
        grid.push(row);
    }
    grid
}

// check if the eight directions arround a position have 4 or more @ characters
fn has_four_or_more_adjacent_tp(
    grid: &Vec<Vec<String>>,
    x: usize,
    y: usize,
    directions: &Vec<(isize, isize)>,
) -> bool {
    let mut adjacent_count = 0;
    for (dx, dy) in directions {
        let ni = x as isize + dx;
        let nj = y as isize + dy;
        if ni >= 0
            && ni < grid.len() as isize
            && nj >= 0
            && nj < grid[x].len() as isize
            && grid[ni as usize][nj as usize] == "@"
        {
            adjacent_count += 1;
        }
    }
    adjacent_count >= 4
}

// Build a grid and count the number of @ characters that have less than 4 adjacent @ characters 
fn solutioner_for_part_1(input: &str) -> String {
    let grid = convert_string_into_two_d_vector(input);
    let directions = vec![
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1), (1, -1),
        (1, 0), (1, 1),
    ];
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == "@" {
                if !has_four_or_more_adjacent_tp(&grid, i, j, &directions) {
                    count += 1;
                }
            }
        }
    }
    count.to_string()
}


// Keep removing @ characters that have less than 4 adjacent @ characters 
// until no more can be removed
fn solutioner_for_part_2(input: &str) -> String {
    let mut grid = convert_string_into_two_d_vector(input);
    let directions = vec![
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    let mut tp_positions: Vec<(usize, usize)> = vec![];

    // Collect all @ positions
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == "@" {
                tp_positions.push((i, j));
            }
        }
    }

    let mut count = 0;
    let mut changed = true;

    // Keep removing until no more valid tp available 
    while changed {
        changed = false;
        let mut to_remove = vec![];

        // Find all positions with < 4 adjacent tp 
        for &(x, y) in &tp_positions {
            if !has_four_or_more_adjacent_tp(&grid, x, y, &directions) {
                to_remove.push((x, y));
            }
        }

        // Remove them
        if !to_remove.is_empty() {
            changed = true;
            count += to_remove.len();

            for (x, y) in to_remove {
                grid[x][y] = "#".to_string();
                tp_positions.retain(|&pos| pos != (x, y));
            }
        }
    }

    count.to_string()
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
