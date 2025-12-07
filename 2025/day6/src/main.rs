use clap::Parser;
use std::fs::File;
use std::io::{self, Read, Write};
use polars::prelude::*;

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

// Input is space separated integer values except for last line which is space
// separated strings which are one character each and are operators like +, -, *, /
fn parse_string_into_dataframe_and_list_of_operators(
    input: &str,
) -> PolarsResult<(DataFrame, Vec<String>)> {
    let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();

    let numeric_lines = &lines[..lines.len() - 1];
    let operator_line = lines[lines.len() - 1];

    let mut columns: Vec<Vec<i64>> = Vec::new();

    for line in numeric_lines {
        let nums: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if columns.is_empty() {
            columns = vec![Vec::new(); nums.len()];
        }

        for (col_idx, &num) in nums.iter().enumerate() {
            columns[col_idx].push(num);
        }
    }

    // Parse operators
    let operators: Vec<String> = operator_line
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    // Create DataFrame
    let series_vec: Vec<Column> = columns
        .into_iter()
        .enumerate()
        .map(|(i, col)| Series::new(format!("col_{}", i + 1).into(), col).into())
        .collect();

    let df = DataFrame::new(series_vec)?;
    Ok((df, operators))
}

// Given input like: 
// 123 328  51 64 
//  45 64  387 23 
//   6 98  215 314
// *   +   *   + 
// Take the numbers in each column and apply the operator at the bottom to them
fn solutioner_for_part_1(input: &str) -> PolarsResult<String> {
    let (df, operators) = parse_string_into_dataframe_and_list_of_operators(input)?;
    let mut total = 0i64;
    for (i, operator) in operators.iter().enumerate() {
        let column = df
            .column(&format!("col_{}", i + 1))?
            .i64()?;
        let numbers_to_perform_operation_on: Vec<i64> = column
            .into_iter()
            .filter_map(|v| v)
            .collect();
        let column_total = match operator.as_str() {
            "+" => numbers_to_perform_operation_on.iter().sum(),
            "*" => numbers_to_perform_operation_on.iter().product(),
            _ => 0,
        };
        total += column_total;
    }
    Ok(total.to_string())
}

// Given input like: 
// 123 328  51 64 
//  45 64  387 23 
//   6 98  215 314
// *   +   *   + 
// Take the numbers in each column and transform them so that a new number is created
// using the digit at each number so the last column becomes 4, 431, and 623
// or for another example the 2nd column becomes 8, 248, and 369 then 
// perform the operation in the final row on those new numbers.
fn solutioner_for_part_2(input: &str) -> PolarsResult<String> {
    let mut total = 0i64;
    let char_matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let operators_row = char_matrix
        .last()
        .ok_or_else(|| PolarsError::ComputeError("No operator row found".into()))?;
    // create a data sructure that can hold a list of operands and a list of numbers associated with each operand
    let mut operands_and_numbers: Vec<(String, Vec<String>)> = Vec::new(); 
    for (col_idx, &operator) in operators_row.iter().enumerate() {
        if !operator.is_whitespace() {
            operands_and_numbers.push((operator.to_string(), Vec::new()));
        }
        let mut new_number_str = String::new();
        for row_idx in 0..(char_matrix.len() - 1) {
            let digit_char = char_matrix[row_idx]
                .get(col_idx)
                .ok_or_else(|| PolarsError::ComputeError("Index out of bounds".into()))?;
            if digit_char.is_digit(10) {
                new_number_str.push(*digit_char);
            }
        }
        operands_and_numbers
            .last_mut()
            .ok_or_else(|| PolarsError::ComputeError("No operand found to associate number with".into()))?
            .1
            .push(new_number_str.clone());
    }
    //println!("Operands map: {:?}", operands_and_numbers);
    // get the total by performing the operation on each list of numbers and summing the result
    for (operator, numbers) in operands_and_numbers {
        let nums_as_i64: Vec<i64> = numbers
            .iter()
            .filter_map(|s| s.parse().ok())
            .collect();
        let column_total = match operator.as_str() {
            "+" => nums_as_i64.iter().sum(),
            "*" => nums_as_i64.iter().product(),
            _ => 0,
        };
        total += column_total;
        // println!(
        //     "Operator: {}, Numbers: {:?}, Column total: {}",
        //     operator, nums_as_i64, column_total
        // );
    }
    Ok(total.to_string())
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
        _ => Err(PolarsError::ComputeError("Invalid part specified".into())),
    }
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let mut output_file = File::create(&output)?;
    output_file.write_all(processed.as_bytes())?;

    println!("Successfully determined solution {} -> {}", args.input, output);

    Ok(())
}
