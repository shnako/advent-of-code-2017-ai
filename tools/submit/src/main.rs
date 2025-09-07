use anyhow::{anyhow, Context, Result};
use clap::Parser;
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, COOKIE, USER_AGENT};
use scraper::{Html, Selector};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Day number (1-25)")]
    day: u8,

    #[arg(help = "Part number (1 or 2)")]
    part: u8,

    #[arg(help = "Answer to submit")]
    answer: String,

    #[arg(short, long, default_value = "2017", help = "Year")]
    year: u16,

    #[arg(
        long,
        help = "Session cookie value (or set AOC_SESSION_COOKIE env var)"
    )]
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

    let session_cookie = args
        .session
        .or_else(|| std::env::var("AOC_SESSION_COOKIE").ok())
        .context("Session cookie required. Set AOC_SESSION_COOKIE env var or use --session")?;

    let client = Client::builder().timeout(Duration::from_secs(30)).build()?;

    println!(
        "Submitting answer '{}' for year {}, day {}, part {}...",
        args.answer, args.year, args.day, args.part
    );

    submit_answer(
        &client,
        &session_cookie,
        args.year,
        args.day,
        args.part,
        &args.answer,
    )?;

    Ok(())
}

fn submit_answer(
    client: &Client,
    session: &str,
    year: u16,
    day: u8,
    part: u8,
    answer: &str,
) -> Result<()> {
    let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);

    let form_data = format!("level={}&answer={}", part, answer);

    let response = client
        .post(&url)
        .header(USER_AGENT, "github.com/shnak/advent-of-code-2017-ai")
        .header(COOKIE, format!("session={}", session))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(form_data)
        .send()
        .context("Failed to submit answer")?;

    if response.status() != 200 {
        return Err(anyhow!(
            "Failed to submit answer: HTTP {}",
            response.status()
        ));
    }

    let html_content = response.text()?;
    let document = Html::parse_document(&html_content);

    let article_selector = Selector::parse("article").unwrap();

    for element in document.select(&article_selector) {
        let text = element.text().collect::<String>();

        if text.contains("That's the right answer!") {
            println!("✓ Correct answer! Well done!");
            if part == 1 {
                println!("Part 2 has been unlocked!");
            } else {
                println!("Day {} completed!", day);
            }
            return Ok(());
        } else if text.contains("That's not the right answer") {
            println!("✗ Incorrect answer");
            if text.contains("too high") {
                println!("  Your answer is too high");
            } else if text.contains("too low") {
                println!("  Your answer is too low");
            }

            if text.contains("Please wait") {
                let wait_text = text.split("Please wait").nth(1).unwrap_or("");
                if let Some(time) = wait_text.split("before trying again").next() {
                    println!("  Rate limited: Please wait{}", time);
                }
            }

            return Err(anyhow!("Answer was incorrect"));
        } else if text.contains("You gave an answer too recently") {
            println!("✗ Rate limited: You submitted too recently");
            return Err(anyhow!("Please wait before submitting again"));
        } else if text.contains("You don't seem to be solving the right level") {
            println!("✗ Wrong part: Have you already completed this part?");
            return Err(anyhow!("Check if you've already solved this part"));
        }
    }

    Err(anyhow!("Unexpected response format from AoC"))
}
