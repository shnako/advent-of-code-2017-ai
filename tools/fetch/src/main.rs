use anyhow::{anyhow, Context, Result};
use clap::Parser;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::{COOKIE, USER_AGENT};
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

    #[arg(
        short,
        long,
        help = "Part number (1, 2, or 'complete' for both parts)",
        default_value = "1"
    )]
    part: String,

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

    if args.part != "1" && args.part != "2" && args.part != "complete" {
        return Err(anyhow!("Part must be 1, 2, or 'complete'"));
    }

    let session_cookie = args
        .session
        .or_else(|| std::env::var("AOC_SESSION_COOKIE").ok())
        .context("Session cookie required. Set AOC_SESSION_COOKIE env var or use --session")?;

    let client = Client::builder().timeout(Duration::from_secs(30)).build()?;

    let day_dir = format!("src/solutions/day{:02}", args.day);
    fs::create_dir_all(&day_dir)?;

    let part_display = if args.part == "complete" {
        "complete puzzle".to_string()
    } else {
        format!("part {}", args.part)
    };
    println!(
        "Fetching puzzle for year {}, day {}, {}...",
        args.year, args.day, part_display
    );

    fetch_puzzle(
        &client,
        &session_cookie,
        args.year,
        args.day,
        &day_dir,
        &args.part,
    )?;

    thread::sleep(Duration::from_secs(2));

    fetch_input(&client, &session_cookie, args.year, args.day, &day_dir)?;

    println!("Successfully fetched puzzle and input to {}/", day_dir);
    Ok(())
}

fn fetch_puzzle(
    client: &Client,
    session: &str,
    year: u16,
    day: u8,
    dir: &str,
    part: &str,
) -> Result<()> {
    let url = format!("https://adventofcode.com/{}/day/{}", year, day);

    let response = client
        .get(&url)
        .header(
            USER_AGENT,
            "github.com/shnako/advent-of-code-2017-ai by contact@example.com",
        )
        .header(COOKIE, format!("session={}", session))
        .send()
        .context("Failed to fetch puzzle")?;

    if response.status() == 400 || response.status() == 404 {
        return Err(anyhow!("Puzzle not yet available or invalid day"));
    }

    if response.status() != 200 {
        return Err(anyhow!(
            "Failed to fetch puzzle: HTTP {}",
            response.status()
        ));
    }

    let html = response.text()?;

    // Convert HTML to plain text using regex patterns similar to the Go implementation
    let puzzle_text = if part == "complete" {
        extract_complete_puzzle(&html)?
    } else {
        extract_puzzle_part(&html, part)?
    };

    let mut final_content = String::new();
    final_content.push_str(&url);
    final_content.push_str("\n\n");
    final_content.push_str(&puzzle_text);

    let puzzle_path = Path::new(dir).join("puzzle.txt");
    // Ensure file ends with a newline
    if !final_content.ends_with('\n') {
        final_content.push('\n');
    }
    fs::write(puzzle_path, final_content)?;

    println!("  ✓ Puzzle description saved");
    Ok(())
}

fn extract_complete_puzzle(html: &str) -> Result<String> {
    // Extract the main content area
    let main_re = Regex::new(r"(?s)<main>(.*?)</main>").unwrap();
    let main_content = if let Some(cap) = main_re.captures(html) {
        cap[1].to_string()
    } else {
        html.to_string()
    };

    // Convert to text
    let mut text = html_to_text(&main_content);

    // Truncate after the completion message
    if let Some(pos) = text.find("Both parts of this puzzle are complete!") {
        let end_pos = text[pos..]
            .find("\n\n")
            .map(|i| pos + i)
            .unwrap_or(text.len());
        text = text[..end_pos].to_string();
    }

    Ok(text)
}

fn extract_puzzle_part(html: &str, part: &str) -> Result<String> {
    // For part 1 or 2, just extract the relevant article
    let article_re = Regex::new(r"(?s)<article[^>]*>(.*?)</article>").unwrap();
    let articles: Vec<_> = article_re.captures_iter(html).collect();

    let part_num = part.parse::<usize>().unwrap_or(1);
    if part_num <= articles.len() {
        let article_html = &articles[part_num - 1][1];
        Ok(html_to_text(article_html))
    } else {
        Err(anyhow!("Part {} not found", part))
    }
}

fn html_to_text(html: &str) -> String {
    let mut text = html.to_string();

    // Remove script and style elements completely
    let script_re = Regex::new(r"(?s)<script[^>]*>.*?</script>").unwrap();
    text = script_re.replace_all(&text, "").to_string();

    let style_re = Regex::new(r"(?s)<style[^>]*>.*?</style>").unwrap();
    text = style_re.replace_all(&text, "").to_string();

    // Replace spans that are just for styling
    let span_re = Regex::new(r"<span[^>]*>(.*?)</span>").unwrap();
    text = span_re.replace_all(&text, "$1").to_string();

    // Headers
    let h2_re = Regex::new(r"<h2[^>]*>(.*?)</h2>").unwrap();
    text = h2_re.replace_all(&text, "$1\n").to_string();

    // Articles need special handling - just remove tags
    text = text.replace("<article class=\"day-desc\">", "");
    text = text.replace("</article>", "\n");

    // Paragraphs - don't add leading newline, just trailing
    text = text.replace("<p>", "");
    text = text.replace("</p>", "\n\n");

    // Handle lists specially - we want them compact
    let ul_re = Regex::new(r"(?s)<ul>(.*?)</ul>").unwrap();
    text = ul_re
        .replace_all(&text, |caps: &regex::Captures| {
            let list_content = &caps[1];
            let li_re = Regex::new(r"(?s)<li>(.*?)</li>").unwrap();
            let items: Vec<String> = li_re
                .captures_iter(list_content)
                .map(|c| c[1].trim().to_string())
                .collect();
            format!("\n{}", items.join("\n"))
        })
        .to_string();

    // Remove formatting tags but keep content
    text = text.replace("<code>", "");
    text = text.replace("</code>", "");
    text = text.replace("<em>", "");
    text = text.replace("</em>", "");
    text = text.replace("<strong>", "");
    text = text.replace("</strong>", "");

    // Remove links but keep text
    let link_re = Regex::new(r"<a[^>]*>(.*?)</a>").unwrap();
    text = link_re.replace_all(&text, "$1").to_string();

    // Remove any remaining tags
    let tag_re = Regex::new(r"<[^>]+>").unwrap();
    text = tag_re.replace_all(&text, "").to_string();

    // Decode HTML entities
    text = text.replace("&lt;", "<");
    text = text.replace("&gt;", ">");
    text = text.replace("&amp;", "&");
    text = text.replace("&quot;", "\"");
    text = text.replace("&#39;", "'");
    text = text.replace("&nbsp;", " ");

    // Clean up excessive whitespace - reduce 3+ newlines to 2
    let whitespace_re = Regex::new(r"\n{3,}").unwrap();
    text = whitespace_re.replace_all(&text, "\n\n").to_string();

    // Also clean up the specific pattern of paragraph followed by list
    text = text.replace("\n\n\n", "\n\n");

    text.trim().to_string()
}

fn fetch_input(client: &Client, session: &str, year: u16, day: u8, dir: &str) -> Result<()> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let response = client
        .get(&url)
        .header(
            USER_AGENT,
            "github.com/shnako/advent-of-code-2017-ai by contact@example.com",
        )
        .header(COOKIE, format!("session={}", session))
        .send()
        .context("Failed to fetch input")?;

    if response.status() == 400 || response.status() == 404 {
        return Err(anyhow!(
            "Please log in to get your personalized puzzle input"
        ));
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
