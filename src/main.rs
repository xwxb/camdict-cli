use std::env;
use reqwest;
use scraper::{Html, Selector};
// use console::style;
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the word from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a word to search");
        return Ok(());
    }
    let word = &args[1];

    // Construct the URL
    let url = format!("https://dictionary.cambridge.org/dictionary/english/{}", word);

    // Send the HTTP GET request
    let response = reqwest::get(&url).await?;

    // Read the response body as text
    let body = response.text().await?;

    let document = Html::parse_document(&body);

    let word_selector = Selector::parse("span .dhw").unwrap();
    let pos_selector = Selector::parse(".pos").unwrap();
    let uk_pronunciation_selector = Selector::parse(".uk .pron .ipa").unwrap();
    let us_pronunciation_selector = Selector::parse(".us .pron .ipa").unwrap();
    
    // Assuming each definition block contains both the definition and examples
    let def_blocks_selector = Selector::parse(".def-block").unwrap();
    let def_selector = Selector::parse(".def").unwrap(); // Adjust if necessary
    let example_selector = Selector::parse(".eg").unwrap(); // Adjust if necessary

    if let Some(word) = document.select(&word_selector).next() {
        println!(" {}", word.text().collect::<Vec<_>>().join(" ").bold().white());
    }
    if let Some(pos) = document.select(&pos_selector).next() {
        println!(" {}", pos.text().collect::<Vec<_>>().join(" ").italic().white());
    }
    if let Some(uk_pron) = document.select(&uk_pronunciation_selector).next() {
        println!("UK Pron: {}", uk_pron.text().collect::<Vec<_>>().join(" ").italic().white());
    }
    if let Some(us_pron) = document.select(&us_pronunciation_selector).next() {
        println!("US Pron: {}", us_pron.text().collect::<Vec<_>>().join(" ").italic().white());
    }

    for def_block in document.select(&def_blocks_selector) {
        // Add a separator for readability between different definitions
        println!("{}", "-".repeat(50).magenta());
        if let Some(def) = def_block.select(&def_selector).next() {
            println!("Def: {}", def.text().collect::<Vec<_>>().join(" ").italic().white());
        }
        // For each definition, find and print all associated example sentences
        for example in def_block.select(&example_selector) {
            println!("-  {}", example.text().collect::<Vec<_>>().join(" ").italic().white());
        }
    }

    Ok(())
}
