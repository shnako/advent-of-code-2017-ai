// Day 21: Fractal Art
// https://adventofcode.com/2017/day/21

use std::collections::HashMap;

type Grid = Vec<Vec<char>>;

fn parse_pattern(s: &str) -> Grid {
    s.split('/').map(|row| row.chars().collect()).collect()
}

fn pattern_to_string(grid: &Grid) -> String {
    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("/")
}

fn rotate(grid: &Grid) -> Grid {
    let n = grid.len();
    let mut result = vec![vec!['.'; n]; n];
    for i in 0..n {
        for j in 0..n {
            result[j][n - 1 - i] = grid[i][j];
        }
    }
    result
}

fn flip(grid: &Grid) -> Grid {
    grid.iter().map(|row| row.iter().rev().copied().collect()).collect()
}

fn get_all_transforms(grid: &Grid) -> Vec<Grid> {
    let mut result = Vec::new();
    let mut current = grid.clone();
    
    for _ in 0..4 {
        result.push(current.clone());
        result.push(flip(&current));
        current = rotate(&current);
    }
    
    result
}

fn enhance(grid: &Grid, rules: &HashMap<String, Grid>) -> Grid {
    let size = grid.len();
    let block_size = if size % 2 == 0 { 2 } else { 3 };
    let blocks_per_side = size / block_size;
    let new_block_size = block_size + 1;
    let new_size = blocks_per_side * new_block_size;
    
    let mut result = vec![vec!['.'; new_size]; new_size];
    
    for block_row in 0..blocks_per_side {
        for block_col in 0..blocks_per_side {
            // Extract block
            let mut block = vec![vec!['.'; block_size]; block_size];
            for i in 0..block_size {
                for j in 0..block_size {
                    block[i][j] = grid[block_row * block_size + i][block_col * block_size + j];
                }
            }
            
            // Find matching rule
            let enhanced = get_all_transforms(&block)
                .iter()
                .find_map(|b| rules.get(&pattern_to_string(b)))
                .expect("No matching rule found")
                .clone();
            
            // Place enhanced block
            for i in 0..new_block_size {
                for j in 0..new_block_size {
                    result[block_row * new_block_size + i][block_col * new_block_size + j] = enhanced[i][j];
                }
            }
        }
    }
    
    result
}

fn count_on(grid: &Grid) -> usize {
    grid.iter().flat_map(|row| row.iter()).filter(|&&c| c == '#').count()
}

pub fn solve_part1(input: &str) -> String {
    let mut rules = HashMap::new();
    
    for line in input.lines() {
        if line.contains(" => ") {
            let parts: Vec<&str> = line.split(" => ").collect();
            let from = parse_pattern(parts[0]);
            let to = parse_pattern(parts[1]);
            
            // Store all transformations
            for transform in get_all_transforms(&from) {
                rules.insert(pattern_to_string(&transform), to.clone());
            }
        }
    }
    
    let mut grid = parse_pattern(".#./..#/###");
    
    for _ in 0..5 {
        grid = enhance(&grid, &rules);
    }
    
    count_on(&grid).to_string()
}

pub fn solve_part2(input: &str) -> String {
    let mut rules = HashMap::new();
    
    for line in input.lines() {
        if line.contains(" => ") {
            let parts: Vec<&str> = line.split(" => ").collect();
            let from = parse_pattern(parts[0]);
            let to = parse_pattern(parts[1]);
            
            // Store all transformations
            for transform in get_all_transforms(&from) {
                rules.insert(pattern_to_string(&transform), to.clone());
            }
        }
    }
    
    let mut grid = parse_pattern(".#./..#/###");
    
    for _ in 0..18 {
        grid = enhance(&grid, &rules);
    }
    
    count_on(&grid).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
        // After 2 iterations, should have 12 pixels on
        let mut rules = HashMap::new();
        
        for line in input.lines() {
            if line.contains(" => ") {
                let parts: Vec<&str> = line.split(" => ").collect();
                let from = parse_pattern(parts[0]);
                let to = parse_pattern(parts[1]);
                
                for transform in get_all_transforms(&from) {
                    rules.insert(pattern_to_string(&transform), to.clone());
                }
            }
        }
        
        let mut grid = parse_pattern(".#./..#/###");
        
        for _ in 0..2 {
            grid = enhance(&grid, &rules);
        }
        
        assert_eq!(count_on(&grid), 12);
    }
}