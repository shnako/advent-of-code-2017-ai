// Day 14: Disk Defragmentation
// https://adventofcode.com/2017/day/14

use crate::utils::hash::knot_hash;

/// Convert a hexadecimal string to a binary string
fn hex_to_binary(hex: &str) -> String {
    hex.chars()
        .map(|c| {
            let digit = c.to_digit(16).unwrap();
            format!("{:04b}", digit)
        })
        .collect()
}

/// Solve part 1: Count the number of used squares in the 128x128 grid
/// Approach: Generate 128 knot hashes (one per row), convert each from hex to binary,
/// and count all the '1' bits across all hashes.
pub fn solve_part1(input: &str) -> u32 {
    let key = input.trim();
    let mut used_count = 0;
    
    // Generate 128 rows
    for row in 0..128 {
        // Create the hash input: key-row
        let hash_input = format!("{}-{}", key, row);
        
        // Get the knot hash
        let hash = knot_hash(&hash_input);
        
        // Convert to binary and count the '1's
        let binary = hex_to_binary(&hash);
        used_count += binary.chars().filter(|&c| c == '1').count() as u32;
    }
    
    used_count
}

/// Build the 128x128 grid of used/free squares
fn build_grid(key: &str) -> Vec<Vec<bool>> {
    let mut grid = Vec::with_capacity(128);
    
    for row in 0..128 {
        let hash_input = format!("{}-{}", key, row);
        let hash = knot_hash(&hash_input);
        let binary = hex_to_binary(&hash);
        
        // Convert binary string to boolean vector
        let row_bits: Vec<bool> = binary.chars().map(|c| c == '1').collect();
        grid.push(row_bits);
    }
    
    grid
}

/// Perform flood fill starting from (row, col) to mark all connected cells in this region
fn flood_fill(grid: &mut Vec<Vec<bool>>, visited: &mut Vec<Vec<bool>>, row: usize, col: usize) {
    // Check bounds and if cell is already visited or not used
    if row >= 128 || col >= 128 || visited[row][col] || !grid[row][col] {
        return;
    }
    
    // Mark as visited
    visited[row][col] = true;
    
    // Visit all four adjacent cells (up, down, left, right)
    if row > 0 {
        flood_fill(grid, visited, row - 1, col);
    }
    if row < 127 {
        flood_fill(grid, visited, row + 1, col);
    }
    if col > 0 {
        flood_fill(grid, visited, row, col - 1);
    }
    if col < 127 {
        flood_fill(grid, visited, row, col + 1);
    }
}

/// Solve part 2: Count the number of regions (connected components) in the grid
pub fn solve_part2(input: &str) -> u32 {
    let key = input.trim();
    
    // Build the grid
    let grid = build_grid(key);
    
    // Track visited cells
    let mut visited = vec![vec![false; 128]; 128];
    
    // Count regions
    let mut region_count = 0;
    
    // Scan the entire grid
    for row in 0..128 {
        for col in 0..128 {
            // If we find an unvisited used square, it's the start of a new region
            if grid[row][col] && !visited[row][col] {
                region_count += 1;
                flood_fill(&mut grid.clone(), &mut visited, row, col);
            }
        }
    }
    
    region_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::input;

    #[test]
    fn test_hex_to_binary() {
        assert_eq!(hex_to_binary("0"), "0000");
        assert_eq!(hex_to_binary("1"), "0001");
        assert_eq!(hex_to_binary("e"), "1110");
        assert_eq!(hex_to_binary("f"), "1111");
        assert_eq!(hex_to_binary("a0c2017"), "1010000011000010000000010111");
    }

    #[test]
    fn test_part1_examples() {
        // The example uses key "flqrgnkx" and expects 8108 used squares
        assert_eq!(solve_part1("flqrgnkx"), 8108);
    }

    #[test]
    fn test_part1_input() {
        let input = input::read_input("src/solutions/day14/input.txt")
            .expect("Failed to read input file");
        let answer = solve_part1(&input);
        assert_eq!(answer, 8140);
    }

    #[test]
    fn test_part2_examples() {
        // The example uses key "flqrgnkx" and expects 1242 regions
        assert_eq!(solve_part2("flqrgnkx"), 1242);
    }

    #[test]
    fn test_part2_input() {
        let input = input::read_input("src/solutions/day14/input.txt")
            .expect("Failed to read input file");
        let answer = solve_part2(&input);
        assert_eq!(answer, 1182);
    }
}