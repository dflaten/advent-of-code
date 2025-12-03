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

// Find the largest two digit number in the string of digits
// The two digits do not have to be adjacent but do have to be in the same order as the string
fn find_largest_two_digit_number(input: &str) -> String {
    let digits: Vec<char> = input.chars().collect();
    let mut largest = "00".to_string();

    for i in 0..digits.len() {
        for j in (i + 1)..digits.len() {
            let candidate = format!("{}{}", digits[i], digits[j]);
            if candidate > largest {
                largest = candidate;
            }
        }
    }
    largest

}
// Split the input into lines of digits and then add the largest two digit 
// number to a running total, returning the total as a string
fn solutioner_for_part_1(input: &str) -> String {
    let lines = input.lines();
    let mut total = 0;
    // print line
    
    for line in lines {
        println!("Processing line: {}", line);
        let largest = find_largest_two_digit_number(line);
        println!("Largest two-digit number in line: {}", largest);
        total += largest.parse::<i32>().unwrap();
    }
    total.to_string()
}
// Find the largest 12 digit subsequence while keeping original order
fn find_largest_twelve_digit_number(input: &str) -> String {
    let digits: Vec<char> = input.chars().collect();
    let n = digits.len();
    let target_len = 12;

    // dp[i][k] = best length-k subsequence using digits starting at i (inclusive)
    // using dynamic programming to build the solution
    let mut dp: Vec<Vec<Option<String>>> = vec![vec![None; target_len + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = Some(String::new());
    }

    for i in (0..n).rev() {
        for k in 1..=target_len {
            // Option 1: skip current digit
            let exclude = dp[i + 1][k].clone();
            // Option 2: take current digit if we can still fill remaining length
            let include = if k <= n - i {
                dp[i + 1][k - 1]
                    .as_ref()
                    .map(|suffix| format!("{}{}", digits[i], suffix))
            } else {
                None
            };

            dp[i][k] = [include, exclude]
                .into_iter()
                .flatten()
                .max();

        }
    }

    dp[0][target_len]
        .clone()
        .expect("input always contains at least 12 digits")
}
// Find the sum of the largest twelve digit numbers from each line of input 
fn solutioner_for_part_2(input: &str) -> String {
    let lines = input.lines();
    let mut total = 0;
    // print line
    
    for line in lines {
        println!("Processing line: {}", line);
        let largest = find_largest_twelve_digit_number(line);
        println!("Largest twelve-digit number in line: {}", largest);
        total += largest.parse::<i64>().unwrap();
    }
    total.to_string()
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
