// Input parsing utilities

use std::fs;

/// Read input file as string
pub fn read_input(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(fs::read_to_string(file_path)?)
}

/// Parse lines from input string
pub fn parse_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}
