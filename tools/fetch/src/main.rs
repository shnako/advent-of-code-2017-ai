use anyhow::{anyhow, Context, Result};
use clap::Parser;
use reqwest::blocking::Client;
use reqwest::header::{COOKIE, USER_AGENT};
use scraper::{Html, Selector};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Day number (1-25)")]
    day: u8,

    #[arg(short, long, default_value = "2017", help = "Year")]
    year: u16,

    #[arg(short, long, help = "Part number (1 or 2)", default_value = "1")]
    part: u8,

    #[arg(long, help = "Session cookie value (or set AOC_SESSION_COOKIE env var)")]
    session: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.day < 1 || args.day > 25 {
        return Err(anyhow!("Day must be between 1 and 25"));
    }

    if args.part != 1 && args.part != 2 {
        return Err(anyhow!("Part must be 1 or 2"));
    }

    let session_cookie = args.session
        .or_else(|| std::env::var("AOC_SESSION_COOKIE").ok())
        .context("Session cookie required. Set AOC_SESSION_COOKIE env var or use --session")?;

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let day_dir = format!("src/solutions/day{:02}", args.day);
    fs::create_dir_all(&day_dir)?;

    println!("Fetching puzzle for year {}, day {}, part {}...", args.year, args.day, args.part);

    fetch_puzzle(&client, &session_cookie, args.year, args.day, &day_dir)?;
    
    thread::sleep(Duration::from_secs(2));
    
    fetch_input(&client, &session_cookie, args.year, args.day, &day_dir)?;

    println!("Successfully fetched puzzle and input to {}/", day_dir);
    Ok(())
}

fn fetch_puzzle(client: &Client, session: &str, year: u16, day: u8, dir: &str) -> Result<()> {
    let url = format!("https://adventofcode.com/{}/day/{}", year, day);
    
    let response = client
        .get(&url)
        .header(USER_AGENT, "github.com/shnak/advent-of-code-2017-ai")
        .header(COOKIE, format!("session={}", session))
        .send()
        .context("Failed to fetch puzzle")?;

    if response.status() == 400 || response.status() == 404 {
        return Err(anyhow!("Puzzle not yet available or invalid day"));
    }

    if response.status() != 200 {
        return Err(anyhow!("Failed to fetch puzzle: HTTP {}", response.status()));
    }

    let html_content = response.text()?;
    let document = Html::parse_document(&html_content);
    
    let article_selector = Selector::parse("article.day-desc").unwrap();
    let mut puzzle_content = String::new();
    
    puzzle_content.push_str(&url);
    puzzle_content.push_str("\n\n");
    
    for element in document.select(&article_selector) {
        let html = element.html();
        let mut markdown = html2md::parse_html(&html);
        
        // Fix the title formatting - remove backslashes and merge split title lines
        // The html2md library adds "\" before "---" and splits titles incorrectly
        markdown = markdown.replace("\\---", "---");
        
        // Fix titles that get split across lines with "----------"
        let lines: Vec<&str> = markdown.lines().collect();
        let mut fixed_lines = Vec::new();
        let mut i = 0;
        
        while i < lines.len() {
            if i + 1 < lines.len() && lines[i].starts_with("---") && lines[i + 1].trim() == "----------" {
                // Merge the title line with the separator line
                fixed_lines.push(lines[i].to_string());
                i += 2; // Skip the "----------" line
            } else {
                fixed_lines.push(lines[i].to_string());
                i += 1;
            }
        }
        
        puzzle_content.push_str(&fixed_lines.join("\n"));
        puzzle_content.push_str("\n");
    }

    let puzzle_path = Path::new(dir).join("puzzle.txt");
    fs::write(puzzle_path, puzzle_content)?;
    
    println!("  ✓ Puzzle description saved");
    Ok(())
}

fn fetch_input(client: &Client, session: &str, year: u16, day: u8, dir: &str) -> Result<()> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    
    let response = client
        .get(&url)
        .header(USER_AGENT, "github.com/shnak/advent-of-code-2017-ai")
        .header(COOKIE, format!("session={}", session))
        .send()
        .context("Failed to fetch input")?;

    if response.status() == 400 || response.status() == 404 {
        return Err(anyhow!("Please log in to get your personalized puzzle input"));
    }

    if response.status() != 200 {
        return Err(anyhow!("Failed to fetch input: HTTP {}", response.status()));
    }

    let input_content = response.text()?;
    
    if input_content.contains("Please don't repeatedly request") {
        return Err(anyhow!("Rate limited. Please wait before trying again"));
    }

    let input_path = Path::new(dir).join("input.txt");
    fs::write(input_path, input_content)?;
    
    println!("  ✓ Input data saved");
    Ok(())
}