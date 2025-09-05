// Advent of Code 2017 - Solution Runner
// Runs all implemented solutions for testing and verification

use advent_of_code_2017::solutions;
use advent_of_code_2017::utils::input;
use anyhow::Result;
use std::env;
use std::time::Instant;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let day: u32 = args[1]
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid day number"))?;
        run_specific_day(day)?;
    } else {
        run_all_solutions()?;
    }

    Ok(())
}

fn run_all_solutions() -> Result<()> {
    println!("🎄 Advent of Code 2017 - Running All Solutions 🎄\n");

    for day in 1..=3 {
        run_day(day)?;
    }

    println!("\n✅ All solutions completed!");
    Ok(())
}

fn run_specific_day(day: u32) -> Result<()> {
    println!("🎄 Advent of Code 2017 - Running Day {} 🎄\n", day);
    run_day(day)?;
    Ok(())
}

fn run_day(day: u32) -> Result<()> {
    // Check if day solution exists by trying to read the module file
    let mod_path = format!("src/solutions/day{:02}/mod.rs", day);
    if !std::path::Path::new(&mod_path).exists() {
        println!("❌ Day {} not implemented yet", day);
        return Ok(());
    }

    // Read the title from the first line of the module file
    let title = std::fs::read_to_string(&mod_path)
        .map_err(|e| anyhow::anyhow!("Failed to read module file for day {}: {}", day, e))?
        .lines()
        .next()
        .unwrap_or(&format!("// Day {}", day))
        .trim_start_matches("// ")
        .to_string();

    let formatted_title = format!("📅 {}", title);

    // Call appropriate solver based on day (hardcoded for return type compatibility)
    match day {
        1 => run_day_u32(
            &formatted_title,
            solutions::day01::solve_part1,
            solutions::day01::solve_part2,
            day,
        ),
        2 => run_day_u32(
            &formatted_title,
            solutions::day02::solve_part1,
            solutions::day02::solve_part2,
            day,
        ),
        3 => run_day_i32(
            &formatted_title,
            solutions::day03::solve_part1,
            solutions::day03::solve_part2,
            day,
        ),
        _ => {
            println!("❌ Day {} not implemented yet", day);
            Ok(())
        }
    }
}

fn run_day_u32(
    title: &str,
    solve1: fn(&str) -> u32,
    solve2: fn(&str) -> u32,
    day_num: u32,
) -> Result<()> {
    println!("{}", title);
    let input_path = format!("src/solutions/day{:02}/input.txt", day_num);
    let input = input::read_input(&input_path)
        .map_err(|e| anyhow::anyhow!("Failed to read input for day {}: {}", day_num, e))?;

    let start = Instant::now();
    let p1 = solve1(&input);
    let d1 = start.elapsed();
    println!("  Part 1: {} ({}µs)", p1, d1.as_micros());

    let start = Instant::now();
    let p2 = solve2(&input);
    let d2 = start.elapsed();
    println!("  Part 2: {} ({}µs)", p2, d2.as_micros());

    println!("  ✅ Day {} completed!\n", day_num);
    Ok(())
}

fn run_day_i32(
    title: &str,
    solve1: fn(&str) -> i32,
    solve2: fn(&str) -> i32,
    day_num: u32,
) -> Result<()> {
    println!("{}", title);
    let input_path = format!("src/solutions/day{:02}/input.txt", day_num);
    let input = input::read_input(&input_path)
        .map_err(|e| anyhow::anyhow!("Failed to read input for day {}: {}", day_num, e))?;

    let start = Instant::now();
    let p1 = solve1(&input);
    let d1 = start.elapsed();
    println!("  Part 1: {} ({}µs)", p1, d1.as_micros());

    let start = Instant::now();
    let p2 = solve2(&input);
    let d2 = start.elapsed();
    println!("  Part 2: {} ({}µs)", p2, d2.as_micros());

    println!("  ✅ Day {} completed!\n", day_num);
    Ok(())
}
