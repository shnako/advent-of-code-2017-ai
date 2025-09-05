// Day 4: High-Entropy Passphrases
// https://adventofcode.com/2017/day/4

use std::collections::HashSet;

/// Solve part 1: Count valid passphrases (no duplicate words)
pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| is_valid_passphrase(line))
        .count()
}

/// Check if a passphrase is valid (contains no duplicate words)
fn is_valid_passphrase(passphrase: &str) -> bool {
    let words: Vec<&str> = passphrase.split_whitespace().collect();
    let unique_words: HashSet<&str> = words.iter().copied().collect();

    // If the set size equals the vector size, there are no duplicates
    words.len() == unique_words.len()
}

/// Solve part 2: Count valid passphrases (no anagrams)
pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| is_valid_passphrase_no_anagrams(line))
        .count()
}

/// Check if a passphrase is valid (contains no anagrams)
fn is_valid_passphrase_no_anagrams(passphrase: &str) -> bool {
    let words: Vec<String> = passphrase
        .split_whitespace()
        .map(|word| {
            // Sort the characters in each word to create a canonical form
            // Two words are anagrams if they have the same canonical form
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort();
            chars.iter().collect()
        })
        .collect();

    let unique_words: HashSet<String> = words.iter().cloned().collect();

    // If the set size equals the vector size, there are no anagrams
    words.len() == unique_words.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        // Test the examples from the puzzle description
        assert!(is_valid_passphrase("aa bb cc dd ee"));
        assert!(!is_valid_passphrase("aa bb cc dd aa"));
        assert!(is_valid_passphrase("aa bb cc dd aaa"));
    }

    #[test]
    fn test_part1_count() {
        let test_input = "aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa";
        assert_eq!(solve_part1(test_input), 2);
    }

    #[test]
    fn test_part1_input() {
        let input = include_str!("input.txt");
        assert_eq!(solve_part1(input), 466);
    }

    #[test]
    fn test_part2_examples() {
        // Test the examples from the puzzle description
        assert!(is_valid_passphrase_no_anagrams("abcde fghij"));
        assert!(!is_valid_passphrase_no_anagrams("abcde xyz ecdab"));
        assert!(is_valid_passphrase_no_anagrams("a ab abc abd abf abj"));
        assert!(is_valid_passphrase_no_anagrams("iiii oiii ooii oooi oooo"));
        assert!(!is_valid_passphrase_no_anagrams("oiii ioii iioi iiio"));
    }

    #[test]
    fn test_part2_count() {
        let test_input = "abcde fghij\nabcde xyz ecdab\na ab abc abd abf abj\niiii oiii ooii oooi oooo\noiii ioii iioi iiio";
        assert_eq!(solve_part2(test_input), 3);
    }

    #[test]
    fn test_part2_input() {
        let input = include_str!("input.txt");
        assert_eq!(solve_part2(input), 251);
    }
}
