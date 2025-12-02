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

// Solution for part one goes here
fn solutioner_for_part_1(input: &str) -> String {
    // Placeholder return 
    input.to_string()
}

// Solution for part two goes here
fn solutioner_for_part_2(input: &str) -> String {
    // Placeholder return 
    input.to_string()
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
