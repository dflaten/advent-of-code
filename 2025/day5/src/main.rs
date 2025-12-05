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
    part: i64,
}

fn split_input_into_two_arrays(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut sections = input.split("\n\n");
    let first_section = sections.next().unwrap_or("");
    let second_section = sections.next().unwrap_or("");

    let fresh_id_ranges = first_section
        .lines()
        .filter_map(|range| {
            let (start_str, end_str) = range.split_once('-')?;
            let start: i64 = start_str.parse().ok()?;
            let end: i64 = end_str.parse().ok()?;
            (start <= end).then_some((start, end))
        })
        .collect();

    let ids_to_check = second_section
        .lines()
        .filter_map(|id| id.parse::<i64>().ok())
        .collect();

    (fresh_id_ranges, ids_to_check)
}


// Determine which ids are valid based on the ranges provided. 
fn solutioner_for_part_1(input: &str) -> String {
    let (fresh_id_ranges, ids_to_check) = split_input_into_two_arrays(input);
    let mut count_of_valid_ids = 0;
    for id in ids_to_check {
        let mut is_valid = false;
        for range in &fresh_id_ranges {
            if id >= range.0 && id <= range.1 {
                is_valid = true;
                break;
            } 
        }
        if is_valid {
            count_of_valid_ids += 1;
        }
    }
    count_of_valid_ids.to_string()
}

// Determine how many ingredient ids are considered to be fresh given the ranges provided
fn solutioner_for_part_2(input: &str) -> String {
    let (fresh_id_ranges, _ids_to_check) = split_input_into_two_arrays(input);

    let mut ranges = fresh_id_ranges;
    ranges.sort_unstable_by_key(|r| r.0);

    // Merge overlapping ranges
    let mut merged = Vec::<(i64, i64)>::new();
    for range in ranges {
        if let Some(last) = merged.last_mut() {
            if range.0 <= last.1 + 1 {
                // Overlapping or adjacent - merge
                last.1 = last.1.max(range.1);
            } else {
                // Not overlapping - add new range
                merged.push(range);
            }
        } else {
            merged.push(range);
        }
    }

    let count: i64 = merged.iter()
        .map(|(start, end)| end - start + 1)
        .sum();

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
