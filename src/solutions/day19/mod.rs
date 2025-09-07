// Day 19: A Series of Tubes
// https://adventofcode.com/2017/day/19

fn find_path(grid: &Vec<Vec<char>>) -> (String, usize) {
    // Find starting position (only | in first row)
    let mut x = 0i32;
    let mut y = 0i32;
    for (col, &ch) in grid[0].iter().enumerate() {
        if ch == '|' {
            y = col as i32;
            break;
        }
    }
    
    // Direction: 0=down, 1=right, 2=up, 3=left
    let mut dir = 0;
    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];
    
    let mut letters = String::new();
    let mut steps = 0;
    
    loop {
        // Move in current direction
        x += dx[dir];
        y += dy[dir];
        steps += 1;
        
        // Check bounds
        if x < 0 || y < 0 || x >= grid.len() as i32 || y >= grid[0].len() as i32 {
            break;
        }
        
        let cell = grid[x as usize][y as usize];
        
        match cell {
            ' ' => break, // End of path
            '|' | '-' => {}, // Continue straight
            '+' => {
                // Need to turn - find the perpendicular direction that's not empty
                let left_dir = (dir + 3) % 4;
                let right_dir = (dir + 1) % 4;
                
                let left_x = x + dx[left_dir];
                let left_y = y + dy[left_dir];
                let right_x = x + dx[right_dir];
                let right_y = y + dy[right_dir];
                
                if left_x >= 0 && left_y >= 0 && 
                   left_x < grid.len() as i32 && left_y < grid[0].len() as i32 {
                    let left_cell = grid[left_x as usize][left_y as usize];
                    if left_cell != ' ' {
                        dir = left_dir;
                        continue;
                    }
                }
                
                if right_x >= 0 && right_y >= 0 && 
                   right_x < grid.len() as i32 && right_y < grid[0].len() as i32 {
                    let right_cell = grid[right_x as usize][right_y as usize];
                    if right_cell != ' ' {
                        dir = right_dir;
                    }
                }
            },
            c if c.is_ascii_alphabetic() => {
                letters.push(c);
            },
            _ => {},
        }
    }
    
    (letters, steps)
}

pub fn solve_part1(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let (letters, _) = find_path(&grid);
    letters
}

pub fn solve_part2(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let (_, steps) = find_path(&grid);
    steps.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
";
        assert_eq!(solve_part1(input), "ABCDEF");
    }
    
    #[test]
    fn test_part2_example() {
        let input = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
";
        assert_eq!(solve_part2(input), "38");
    }
    
    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("src/solutions/day19/input.txt").unwrap();
        assert_eq!(solve_part1(&input), "DWNBGECOMY");
    }
    
    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("src/solutions/day19/input.txt").unwrap();
        assert_eq!(solve_part2(&input), "17228");
    }
}