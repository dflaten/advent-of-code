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

use std::collections::HashMap;

#[derive(Debug)]
struct ParsedInput {
    presents: HashMap<usize, Vec<String>>,
    grids_with_present_count: Vec<((usize, usize), Vec<usize>)>,
}

/// Takes a text input that contains present patterns and grids with present 
/// count and returns a ParsedInput struct containing the parsed data.
fn parse_input_part_1(input: &str) -> ParsedInput {
    let mut presents: HashMap<usize, Vec<String>> = HashMap::new();
    let mut grids_with_present_count: Vec<((usize, usize), Vec<usize>)> = Vec::new();

    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Pattern definition (e.g., "0:"). Other lines may also end with ':' (e.g. prose),
        // so only treat it as a pattern if the prefix parses as a number.
        if let Some(id_str) = line.strip_suffix(':') {
            if let Ok(pattern_id) = id_str.parse::<usize>() {
                let mut pattern_lines = Vec::new();
                i += 1;

                // Collect pattern lines until we hit an empty line or a new section
                while i < lines.len() {
                    let pattern_line = lines[i];
                    if pattern_line.trim().is_empty() {
                        break;
                    }
                    if pattern_line.contains(':') && !pattern_line.starts_with(char::is_whitespace) {
                        break;
                    }
                    pattern_lines.push(pattern_line.to_string());
                    i += 1;
                }

                presents.insert(pattern_id, pattern_lines);
                while i < lines.len() && lines[i].trim().is_empty() {
                    i += 1;
                }
                continue;
            }
        }

        // Grid definition (e.g., "4x4: 0 0 0 0 2 0")
        if line.contains(':') && line.contains('x') {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let dimensions: Vec<&str> = parts[0].trim().split('x').collect();
                if dimensions.len() == 2 {
                    if let (Ok(width), Ok(height)) = (
                        dimensions[0].parse::<usize>(),
                        dimensions[1].parse::<usize>()
                    ) {
                        let numbers: Vec<usize> = parts[1]
                            .trim()
                            .split_whitespace()
                            .filter_map(|s| s.parse::<usize>().ok())
                            .collect();

                        grids_with_present_count.push(((width, height), numbers));
                    }
                }
            }
            i += 1;
        }
        else {
            i += 1;
        }
    }

    ParsedInput { presents, grids_with_present_count }
}

#[derive(Clone, Copy, Debug)]
struct PresentInfo {
    filled_cells: usize,
    bbox_width: usize,
    bbox_height: usize,
}

/// Takes pattern lines and calculates the filled cells and bounding box dimensions
fn present_info(pattern_lines: &[String]) -> PresentInfo {
    let mut filled_cells = 0usize;
    let mut min_x: Option<usize> = None;
    let mut max_x: Option<usize> = None;
    let mut min_y: Option<usize> = None;
    let mut max_y: Option<usize> = None;

    for (y, line) in pattern_lines.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character != '#' {
                continue;
            }
            filled_cells += 1;
            min_x = Some(min_x.map_or(x, |prev| prev.min(x)));
            max_x = Some(max_x.map_or(x, |prev| prev.max(x)));
            min_y = Some(min_y.map_or(y, |prev| prev.min(y)));
            max_y = Some(max_y.map_or(y, |prev| prev.max(y)));
        }
    }

    let (bbox_width, bbox_height) = match (min_x, max_x, min_y, max_y) {
        (Some(min_x), Some(max_x), Some(min_y), Some(max_y)) => {
            (max_x - min_x + 1, max_y - min_y + 1)
        }
        _ => (0, 0),
    };

    PresentInfo {
        filled_cells,
        bbox_width,
        bbox_height,
    }
}

/// Uses fast feasability check to determine if present can fit in the given board dimensions
fn can_fit_presents_quick(
    board_width: usize,
    board_height: usize,
    present_counts_by_id: &[usize],
    present_info_by_id: &HashMap<usize, PresentInfo>,
) -> bool {
    let board_area = board_width * board_height;
    let mut needed_cells = 0usize;

    for (present_id, count) in present_counts_by_id.iter().enumerate() {
        if *count == 0 {
            continue;
        }
        let Some(info) = present_info_by_id.get(&present_id) else {
            return false;
        };

        let fits_dimensionwise = (info.bbox_width <= board_width && info.bbox_height <= board_height)
            || (info.bbox_height <= board_width && info.bbox_width <= board_height);
        if !fits_dimensionwise {
            return false;
        }

        needed_cells = needed_cells.saturating_add(info.filled_cells.saturating_mul(*count));
        if needed_cells > board_area {
            return false;
        }
    }

    true
}



/// Solution for part one goes here
fn solutioner_for_part_1(input: &str) -> String {
    let ParsedInput { presents, grids_with_present_count } = parse_input_part_1(input);
    let mut fit_count = 0usize;

    let present_info_by_id: HashMap<usize, PresentInfo> = presents
        .iter()
        .map(|(present_id, pattern_lines)| (*present_id, present_info(pattern_lines)))
        .collect();

    for ((width, height), present_counts) in grids_with_present_count {
        if can_fit_presents_quick(width, height, &present_counts, &present_info_by_id) {
            fit_count += 1;
        }
    }
    fit_count.to_string()
}

/// Solution for part two goes here
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
