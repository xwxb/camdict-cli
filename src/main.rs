use reqwest;
use scraper::{Html, Selector};
use colored::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Opts {
    word: Vec<String>,

    /// Number of def and examples to show
    #[arg(short, long, default_value_t = 1)]
    number: u8,

    /// Show all definitions and examples
    #[arg(short, long, default_value_t = false)]
    all: bool,
}

async fn parse_args() -> Result<Opts, Box<dyn std::error::Error>> {
    // Parse the command line arguments
    let opts: Opts = Opts::parse();
    Ok(opts)
}

fn construct_url(word: &str) -> String {
    // Construct the URL
    format!("https://dictionary.cambridge.org/dictionary/english/{}", word)
}

async fn send_request(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Send the HTTP GET request
    let response = reqwest::get(url).await?;
    // Read the response body as text
    let body = response.text().await?;
    Ok(body)
}

fn parse_html(body: &str) {
    let document = Html::parse_document(body);

    let word_selector = Selector::parse("span .dhw").unwrap();
    let pos_selector = Selector::parse(".pos").unwrap();
    let uk_pronunciation_selector = Selector::parse(".uk .pron .ipa").unwrap();
    let us_pronunciation_selector = Selector::parse(".us .pron .ipa").unwrap();
    
    // Assuming each definition block contains both the definition and examples
    let def_blocks_selector = Selector::parse(".def-block").unwrap();
    let def_selector = Selector::parse(".def").unwrap(); // Adjust if necessary
    let example_selector = Selector::parse(".eg").unwrap(); // Adjust if necessary

    if let Some(word) = document.select(&word_selector).next() {
        print!(" {}    ", word.text().collect::<Vec<_>>().join(" ").bold().white());
    }
    if let Some(pos) = document.select(&pos_selector).next() {
        println!(" {}", pos.text().collect::<Vec<_>>().join(" ").italic().white());
    }
    if let Some(uk_pron) = document.select(&uk_pronunciation_selector).next() {
        print!("UK /{}/", uk_pron.text().collect::<Vec<_>>().join(" ").italic().cyan());
    }
    if let Some(us_pron) = document.select(&us_pronunciation_selector).next() {
        println!(" US /{}/    ", us_pron.text().collect::<Vec<_>>().join(" ").italic().cyan());
    }

    for def_block in document.select(&def_blocks_selector) {
        if let Some(def) = def_block.select(&def_selector).next() {
            // remove bad data
            // if def is begin with `→ `, continue
            let text = def.text().collect::<Vec<_>>()[0].trim();
            if text == "→" {
                continue;
            }

            // Add a separator for readability between different definitions
            println!("{}", "-".repeat(50).magenta());
            println!("Def: {}", def.text().collect::<Vec<_>>().join(" ").italic().yellow());
        }
        // For each definition, find and print all associated example sentences
        for example in def_block.select(&example_selector) {
            println!("-  {}", example.text().collect::<Vec<_>>().join(" ").italic().green());
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command line arguments
    let opts = parse_args().await?;

    let word_vec = opts.word;
    let word = if word_vec.len() <= 4 {
        word_vec[0..].join("-")
    } else {
        return Err(("Too many words provided. Please provide 2 to 4 words separated by hyphens").into());
    };
    
    let url = construct_url(&word);
    let body = send_request(&url).await?;

    parse_html(&body);

    Ok(())
}
