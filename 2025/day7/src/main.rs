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
// Converts a string of characters into a 2D vector of strings
fn convert_string_into_two_d_vector(input: &str) -> Vec<Vec<String>> {
    let mut grid: Vec<Vec<String>> = Vec::new();
    for line in input.lines() {
        let row: Vec<String> = line.chars().map(|c| c.to_string()).collect();
        grid.push(row);
    }
    grid
}

// Build a grid and count the number of splits the beam makes when encountering ^ characters
fn solutioner_for_part_1(input: &str) -> String {
    let grid = convert_string_into_two_d_vector(input);

    // Find S character on the first row
    let mut start_x: Option<usize> = None;
    for (j, cell) in grid[0].iter().enumerate() {
        if cell == "S" {
            start_x = Some(j);
            break;
        }
    }
    let current_beam_start = match start_x {
        Some(x) => x,
        None => return "No starting position found".to_string(),
    };

    let mut number_of_splits = 0;
    let mut beam_positions: Vec<usize> = vec![current_beam_start];
    let width = grid[0].len();

    for i in 0..grid.len() {
        let mut new_beam_positions: Vec<usize> = Vec::new();

        for &pos in &beam_positions {
            if grid[i][pos] == "^" {
                // Split beam into left 
                if pos > 0 {
                    new_beam_positions.push(pos - 1);
                }
                // and right
                if pos + 1 < width {
                    new_beam_positions.push(pos + 1);
                }
                number_of_splits += 1;
            } else {
                // Continue beam straight down
                new_beam_positions.push(pos);
            }
        }

        // Dedupe positions
        new_beam_positions.sort_unstable();
        new_beam_positions.dedup();

        beam_positions = new_beam_positions;
    }

    number_of_splits.to_string()
}


// Build a grid and make paths from the S counter spliting the beam when 
// encountering ^ characters. Count the number of unique paths that are created
// by the splitting of the beams. Where a path is started at S and ends 
// when the path gets to the edge of the grid.
fn solutioner_for_part_2(input: &str) -> String {
    let grid = convert_string_into_two_d_vector(input);

    let start_x = match grid[0].iter().position(|cell| cell == "S") {
        Some(x) => x,
        None => return "No starting position found".to_string(),
    };

    let width = grid[0].len();
    let height = grid.len();

    // DP over rows: ways[r][c] is number of paths that reach column c on row r.
    let mut current_counts = vec![0u128; width];
    current_counts[start_x] = 1;

    for i in 1..height {
        let mut next_counts = vec![0u128; width];

        // Update counts for the next row based on current row
        for (pos, count) in current_counts.iter().enumerate() {
            if *count == 0 {
                continue;
            }

            if grid[i][pos] == "^" {
                if pos > 0 {
                    next_counts[pos - 1] += *count;
                }
                if pos + 1 < width {
                    next_counts[pos + 1] += *count;
                }
            } else {
                next_counts[pos] += *count;
            }
        }

        current_counts = next_counts;
    }

    current_counts.iter().sum::<u128>().to_string()
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
