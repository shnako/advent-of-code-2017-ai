// Advent of Code 2017 - Solution Runner
// Runs all implemented solutions for testing and verification

use advent_of_code_2017::solutions;
use advent_of_code_2017::utils::input;
use anyhow::Result;
use std::env;
use std::time::Instant;

const MAX_DAY: u32 = 12;

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

    for day in 1..=MAX_DAY {
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
    let fallback_title = format!("Day {}", day);
    let title = std::fs::read_to_string(&mod_path)
        .map_err(|e| anyhow::anyhow!("Failed to read module file for day {}: {}", day, e))?
        .lines()
        .next()
        .map(|line| line.trim_start_matches("// "))
        .unwrap_or(&fallback_title)
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
        4 => run_day_usize(
            &formatted_title,
            solutions::day04::solve_part1,
            solutions::day04::solve_part2,
            day,
        ),
        5 => run_day_u32(
            &formatted_title,
            solutions::day05::solve_part1,
            solutions::day05::solve_part2,
            day,
        ),
        6 => run_day_usize(
            &formatted_title,
            solutions::day06::solve_part1,
            solutions::day06::solve_part2,
            day,
        ),
        7 => run_day_string_i32(
            &formatted_title,
            solutions::day07::solve_part1,
            solutions::day07::solve_part2,
            day,
        ),
        8 => run_day_i32(
            &formatted_title,
            solutions::day08::solve_part1,
            solutions::day08::solve_part2,
            day,
        ),
        9 => run_day_i32(
            &formatted_title,
            solutions::day09::solve_part1,
            solutions::day09::solve_part2,
            day,
        ),
        10 => run_day_u32_string(
            &formatted_title,
            solutions::day10::solve_part1,
            solutions::day10::solve_part2,
            day,
        ),
        11 => run_day_i32(
            &formatted_title,
            solutions::day11::solve_part1,
            solutions::day11::solve_part2,
            day,
        ),
        12 => run_day_usize(
            &formatted_title,
            solutions::day12::solve_part1,
            solutions::day12::solve_part2,
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

fn run_day_usize(
    title: &str,
    solve1: fn(&str) -> usize,
    solve2: fn(&str) -> usize,
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

fn run_day_string_i32(
    title: &str,
    solve1: fn(&str) -> String,
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

fn run_day_u32_string(
    title: &str,
    solve1: fn(&str) -> u32,
    solve2: fn(&str) -> String,
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
