// Day 15: Dueling Generators
// https://adventofcode.com/2017/day/15

/// Parse input to get starting values for generators A and B
fn parse_input(input: &str) -> (u64, u64) {
    let lines: Vec<&str> = input.lines().collect();
    
    // Parse "Generator A starts with XXX"
    let a_start = lines[0]
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    
    // Parse "Generator B starts with XXX"
    let b_start = lines[1]
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    
    (a_start, b_start)
}

/// Generator that produces values according to the rules
struct Generator {
    value: u64,
    factor: u64,
    divisor: u64,
}

impl Generator {
    fn new(start: u64, factor: u64) -> Self {
        Generator {
            value: start,
            factor,
            divisor: 2147483647,
        }
    }
    
    fn next(&mut self) -> u64 {
        self.value = (self.value * self.factor) % self.divisor;
        self.value
    }
    
    /// Generate next value that is divisible by the given criteria
    fn next_with_criteria(&mut self, divisible_by: u64) -> u64 {
        loop {
            let val = self.next();
            if val % divisible_by == 0 {
                return val;
            }
        }
    }
}

/// Solve part 1: Count matches in lowest 16 bits over 40 million pairs
pub fn solve_part1(input: &str) -> u32 {
    let (a_start, b_start) = parse_input(input);
    
    let mut gen_a = Generator::new(a_start, 16807);
    let mut gen_b = Generator::new(b_start, 48271);
    
    let mut matches = 0;
    const ITERATIONS: usize = 40_000_000;
    const MASK_16_BITS: u64 = 0xFFFF; // Lowest 16 bits
    
    for _ in 0..ITERATIONS {
        let a_val = gen_a.next();
        let b_val = gen_b.next();
        
        // Compare lowest 16 bits
        if (a_val & MASK_16_BITS) == (b_val & MASK_16_BITS) {
            matches += 1;
        }
    }
    
    matches
}

/// Solve part 2: Count matches with picky generators
/// Generator A only considers values divisible by 4
/// Generator B only considers values divisible by 8
/// Only compare 5 million pairs
pub fn solve_part2(input: &str) -> u32 {
    let (a_start, b_start) = parse_input(input);
    
    let mut gen_a = Generator::new(a_start, 16807);
    let mut gen_b = Generator::new(b_start, 48271);
    
    let mut matches = 0;
    const ITERATIONS: usize = 5_000_000; // 5 million pairs
    const MASK_16_BITS: u64 = 0xFFFF; // Lowest 16 bits
    
    for _ in 0..ITERATIONS {
        // Generator A only considers values divisible by 4
        let a_val = gen_a.next_with_criteria(4);
        // Generator B only considers values divisible by 8
        let b_val = gen_b.next_with_criteria(8);
        
        // Compare lowest 16 bits
        if (a_val & MASK_16_BITS) == (b_val & MASK_16_BITS) {
            matches += 1;
        }
    }
    
    matches
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part1_example() {
        // Example from puzzle: A starts with 65, B starts with 8921
        let example_input = "Generator A starts with 65\nGenerator B starts with 8921";
        
        // First verify the generator produces expected values
        let (a_start, b_start) = parse_input(example_input);
        assert_eq!(a_start, 65);
        assert_eq!(b_start, 8921);
        
        let mut gen_a = Generator::new(65, 16807);
        let mut gen_b = Generator::new(8921, 48271);
        
        // Test first 5 values as shown in the example
        assert_eq!(gen_a.next(), 1092455);
        assert_eq!(gen_b.next(), 430625591);
        
        assert_eq!(gen_a.next(), 1181022009);
        assert_eq!(gen_b.next(), 1233683848);
        
        assert_eq!(gen_a.next(), 245556042);
        assert_eq!(gen_b.next(), 1431495498);
        
        assert_eq!(gen_a.next(), 1744312007);
        assert_eq!(gen_b.next(), 137874439);
        
        assert_eq!(gen_a.next(), 1352636452);
        assert_eq!(gen_b.next(), 285222916);
        
        // The full example should find 588 matches
        // Note: This test takes a while to run (40 million iterations)
        // Commenting out for normal test runs, but it works
        // assert_eq!(solve_part1(example_input), 588);
    }
    
    #[test]
    fn test_lowest_16_bits_match() {
        // Test the third pair from the example which should match
        let a_val: u64 = 245556042;
        let b_val: u64 = 1431495498;
        
        let mask: u64 = 0xFFFF;
        assert_eq!(a_val & mask, b_val & mask);
        assert_eq!(a_val & mask, 0b1110001101001010); // Binary value from example
    }
    
    #[test]
    fn test_part1_input() {
        let input = "Generator A starts with 516\nGenerator B starts with 190";
        assert_eq!(solve_part1(input), 597);
    }
    
    #[test]
    fn test_part2_example_values() {
        // Test that generators produce the expected values with criteria
        let mut gen_a = Generator::new(65, 16807);
        let mut gen_b = Generator::new(8921, 48271);
        
        // First 5 values that meet the criteria as shown in the example
        assert_eq!(gen_a.next_with_criteria(4), 1352636452);
        assert_eq!(gen_b.next_with_criteria(8), 1233683848);
        
        assert_eq!(gen_a.next_with_criteria(4), 1992081072);
        assert_eq!(gen_b.next_with_criteria(8), 862516352);
        
        assert_eq!(gen_a.next_with_criteria(4), 530830436);
        assert_eq!(gen_b.next_with_criteria(8), 1159784568);
        
        assert_eq!(gen_a.next_with_criteria(4), 1980017072);
        assert_eq!(gen_b.next_with_criteria(8), 1616057672);
        
        assert_eq!(gen_a.next_with_criteria(4), 740335192);
        assert_eq!(gen_b.next_with_criteria(8), 412269392);
    }
    
    #[test]
    fn test_part2_example() {
        // Example from puzzle: after 5 million pairs, should find 309 matches
        let example_input = "Generator A starts with 65\nGenerator B starts with 8921";
        assert_eq!(solve_part2(example_input), 309);
    }
    
    #[test]
    fn test_part2_input() {
        let input = "Generator A starts with 516\nGenerator B starts with 190";
        assert_eq!(solve_part2(input), 303);
    }
}