use reqwest;
use scraper::{Html, Selector};
use colored::*;
use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Opts {
    word: Vec<String>,

    /// Number of def and examples to show
    #[arg(short, long, default_value_t = 3)]
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

fn parse_and_output(body: &str, opts: &Opts) {
    let document = Html::parse_document(body);

    let word_selector = Selector::parse("h2, .dhw").unwrap();
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

    let mut cnt: u8 = 0;
    // todo 正则的处理不是很优雅
    let re_space = Regex::new(r" +").unwrap();
    let re_punctuation = Regex::new(r"\s(\p{P})").unwrap();

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
            let def_text = def.text().map(|s| s.trim()).collect::<Vec<_>>().join(" ");
            let def_text = re_punctuation.replace_all(&def_text, "$1");
            let def_text = re_space.replace_all(&def_text, " ");
            println!("Def: {}", def_text.italic().yellow());
        }
        // For each definition, find and print all associated example sentences
        for example in def_block.select(&example_selector) {
            let example_text = example.text().map(|s| s.trim()).collect::<Vec<_>>().join(" ");
            let example_text = re_punctuation.replace_all(&example_text, "$1");
            let example_text = re_space.replace_all(&example_text, " ");
            println!("-  {}", example_text.italic().green());
        }
    

        cnt += 1;
        if !opts.all && cnt >= opts.number {
            break;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command line arguments
    let opts = parse_args().await?;

    let word = if opts.word.len() <= 4 {
        opts.word[0..].join("-")
    } else {
        return Err(("Too many words provided. Please provide 2 to 4 words separated by hyphens").into());
    };
    let url = construct_url(&word);
    let body = send_request(&url).await?;

    parse_and_output(&body, &opts);

    Ok(())
}
