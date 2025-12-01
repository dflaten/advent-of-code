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

    /// Output file path
    #[arg(short, long)]
    output: String,

    // Part to solve (1 or 2)
    #[arg(short, long, default_value_t = 1)]
    part: i32,
}
// Determines the new position of the pointer based on the current position and instruction
// 
// current_position is an integer between 0 and 99
// instruction is a string like "L10" or "R5"
// L means move left (decrease position), R means move right (increase position)
// returns the new position of the pointer, wrapping around at 0 and 99
fn determine_new_pointer_position(current_position: i32, instruction: &str) -> i32 {
    let (direction, distance_str) = instruction.split_at(1);
    let distance: i32 = distance_str.parse().unwrap();
    match direction {
        "L" => (current_position - distance + 100) % 100,
        "R" => (current_position + distance) % 100,
        _ => current_position, // Invalid instruction, no change
    }
}

// Determines the number of times the pointer lands on position 0
fn solutioner_for_part_1(input: &str) -> String {
    // Position of Pointer
    let mut position: i32 = 50;
    // First split each line into their own string in an array
    let lines: Vec<&str> = input.lines().collect();
    // For each line, determine the new position of the pointer
    let mut zero_count = 0;
    for line in lines {
        position = determine_new_pointer_position(position, line);
        if position == 0 {
            zero_count += 1;
        }
    }
    zero_count.to_string()
}

// Determines the number of times the pointer "clicks" position 0
fn solutioner_for_part_2(input: &str) -> String {
    let mut position: i32 = 50;
    let lines: Vec<&str> = input.lines().collect();
    let mut zero_count = 0;
    for line in lines {
        let (direction, distance_str) = line.split_at(1);
        let distance: i32 = distance_str.parse().unwrap();
        for _ in 0..distance {
            match direction {
                "L" => {
                    position = (position - 1 + 100) % 100;
                    if position == 0 {
                        zero_count += 1;
                    }
                }
                "R" => {
                    position = (position + 1) % 100;
                    if position == 0 {
                        zero_count += 1;
                    }
                }
                _ => {}
            }
        }
    }
    zero_count.to_string()
}

/// Main function to read input file, process it, and write to output file
/// Uses command line arguments for input and output file paths
/// Example usage: cargo run -- --i input.txt --o output.txt --p 1
fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut input_file = File::open(&args.input)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    let processed = match args.part {
        1 => solutioner_for_part_1(&contents),
        2 => solutioner_for_part_2(&contents),
        _ => "Invalid part specified".to_string(),
    };

    let mut output_file = File::create(&args.output)?;
    output_file.write_all(processed.as_bytes())?;

    println!("Successfully determined solution {} -> {}", args.input, args.output);

    Ok(())
}
