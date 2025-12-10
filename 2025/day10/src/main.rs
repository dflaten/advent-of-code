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

// Parse a single line into its target light array, button, and voltage definitions.
fn parse_line_into_vectors(line: &str) -> (Vec<char>, Vec<Vec<usize>>, Vec<usize>) {
    let light_array: Vec<char> = line
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_matches(|c| c == '[' || c == ']')
        .chars()
        .filter(|&c| c == '.' || c == '#')
        .collect();

    let mut button_vectors: Vec<Vec<usize>> = Vec::new();
    let mut voltagages: Vec<usize> = Vec::new();
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '(' {
            let mut nums_str = String::new();
            for ch in chars.by_ref() {
                if ch == ')' {
                    break;
                }
                nums_str.push(ch);
            }
            let nums: Vec<usize> = nums_str
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            if !nums.is_empty() {
                button_vectors.push(nums);
            }
        }
        else if c == '{' {
            let mut nums_str = String::new();
            for ch in chars.by_ref() {
                if ch == '}' {
                    break;
                }
                nums_str.push(ch);
            }
            voltagages = nums_str
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
        }
    }

    (light_array, button_vectors, voltagages)
}

// input is a text string where each line has something like this:
// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// The first part is an array bounded by square brackets, containing '.' and '#' 
// characters representing lights that should be on or off respectively. The 
// goal will be to end up in this state by pressing the buttons, which are the
// parts of the line in parenthesesis. Each parenthesis is one button which 
// toggles the lights in the positions of the numbers inside the parenthesis.
// The curly braces at the end represent voltagages beig tracked, but are not
// relevant to part 1.
// For part one, determine the minimum number of button presses needed to
// achieve the target light configuration for each line. Then return the sum 
// of these minimums as a string. 
fn solutioner_for_part_1(input: &str) -> String {
    let mut total_presses: usize = 0;

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (light_array, button_vectors) = parse_line_into_vectors(line);
        let light_count = light_array.len();
        let button_count = button_vectors.len();

        if light_count == 0 || button_count == 0 {
            continue;
        }

        // Convert target state to bitmask (# = 1, . = 0)
        let target_state: usize = light_array.iter().enumerate()
            .fold(0, |acc, (i, &c)| if c == '#' { acc | (1 << i) } else { acc });

        // Precompute button effects as bitmasks
        let button_masks: Vec<usize> = button_vectors.iter()
            .map(|button| {
                button.iter()
                    .filter(|&&idx| idx < light_count)
                    .fold(0, |acc, &idx| acc | (1 << idx))
            })
            .collect();

        let mut min_presses = usize::MAX;

        // Try all combinations
        for combo in 0usize..(1 << button_count) {
            let mut state = 0;
            for (button_idx, &mask) in button_masks.iter().enumerate() {
                if (combo & (1 << button_idx)) != 0 {
                    state ^= mask;
                }
            }

            if state == target_state {
                let presses = combo.count_ones() as usize;
                min_presses = min_presses.min(presses);
            }
        }

        if min_presses != usize::MAX {
            total_presses += min_presses;
        }
    }

    total_presses.to_string()
}

// placeholder for part 2 solution
fn solutioner_for_part_2(_input: &str) -> String {
    "Not implemented".to_string()
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
