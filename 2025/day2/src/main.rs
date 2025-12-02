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
// Any ID which is made only of some sequence of digits repeated twice is invalid.
// So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.
fn is_id_valid(id: i64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();
    if len % 2 != 0 {
        return true; // Odd length IDs are valid
    }
    let (first_half, second_half) = id_str.split_at(len / 2);
    first_half != second_half
}

// Determines the sum of all invalid ids that appear in the given ranges. 
fn solutioner_for_part_1(input: &str) -> String {
    let mut sum_invalid_ids: i64 = 0;
    let ranges_line = input.lines().next().unwrap();
    for range in ranges_line.split(',') {
        let parts: Vec<&str> = range.split('-').collect();
        let start: i64 = parts[0].parse().unwrap();
        let end: i64 = parts[1].parse().unwrap();
        for id in start..=end {
            if !is_id_valid(id) {
                sum_invalid_ids += id;
            }
        }
    }
    sum_invalid_ids.to_string()
}
// An Id is invalid if it is made only of some dequence of digits repeated at least twice 
fn is_id_valid_updated(id: i64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();
    for sub_len in 1..=(len / 2) {
        if len % sub_len == 0 {
            let (first_sub, _rest) = id_str.split_at(sub_len);
            if first_sub.repeat(len / sub_len) == id_str {
                return false; // Found a repeating sequence
            }
        }
    }
    true // No repeating sequence found
}

// Determines the number of times the pointer "clicks" position 0
fn solutioner_for_part_2(input: &str) -> String {
    let mut sum_invalid_ids: i64 = 0;
    let ranges_line = input.lines().next().unwrap();
    for range in ranges_line.split(',') {
        let parts: Vec<&str> = range.split('-').collect();
        let start: i64 = parts[0].parse().unwrap();
        let end: i64 = parts[1].parse().unwrap();
        for id in start..=end {
            if !is_id_valid_updated(id) {
                sum_invalid_ids += id;
            }
        }
    }
    sum_invalid_ids.to_string()

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
