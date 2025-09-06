// Hash utilities for Advent of Code solutions

/// Compute the full Knot Hash of an input string
/// Returns a 32-character hexadecimal string
pub fn knot_hash(input: &str) -> String {
    // Convert input string to ASCII codes
    let mut lengths: Vec<usize> = input.trim().bytes().map(|b| b as usize).collect();

    // Add the standard suffix
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    // Initialize the list with numbers from 0 to 255
    let mut list: Vec<u8> = (0..=255).collect();

    let mut current_position = 0;
    let mut skip_size = 0;

    // Run 64 rounds
    for _ in 0..64 {
        knot_hash_round(&mut list, &lengths, &mut current_position, &mut skip_size);
    }

    // Create dense hash by XORing blocks of 16
    let mut dense_hash = Vec::new();
    for block_start in (0..256).step_by(16) {
        let mut xor_result = 0u8;
        for i in 0..16 {
            xor_result ^= list[block_start + i];
        }
        dense_hash.push(xor_result);
    }

    // Convert to hexadecimal string
    use std::fmt::Write;
    let mut result = String::with_capacity(32);
    for &b in &dense_hash {
        write!(&mut result, "{:02x}", b).unwrap();
    }
    result
}

/// Helper function to perform one round of the knot hash algorithm
fn knot_hash_round(
    list: &mut [u8],
    lengths: &[usize],
    current_position: &mut usize,
    skip_size: &mut usize,
) {
    let list_size = list.len();

    for &length in lengths {
        if length > list_size {
            continue;
        }

        // Reverse the order of `length` elements starting at current_position
        let mut indices = Vec::new();
        for i in 0..length {
            indices.push((current_position.wrapping_add(i)) % list_size);
        }

        // Extract the values to reverse
        let mut values: Vec<u8> = indices.iter().map(|&i| list[i]).collect();
        values.reverse();

        // Put the reversed values back
        for (i, &idx) in indices.iter().enumerate() {
            list[idx] = values[i];
        }

        // Move current position forward by length + skip_size
        *current_position = (current_position
            .wrapping_add(length)
            .wrapping_add(*skip_size))
            % list_size;

        // Increase skip size
        *skip_size = skip_size.wrapping_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knot_hash() {
        // Test the examples from Day 10 Part 2
        assert_eq!(knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(knot_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(knot_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(knot_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}