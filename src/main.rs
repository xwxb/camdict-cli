use std::env;
use reqwest;
use scraper::{Html, Selector};
use console::style;

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

    // // Define a CSS selector for the main definition.
    // let def_selector = Selector::parse(".def-block .ddef_d").unwrap();
    // Adjusted selectors based on actual HTML structure
    let word_selector = Selector::parse("span .hw").unwrap();
    let pos_selector = Selector::parse(".pos-header .pos").unwrap();
    let uk_pronunciation_selector = Selector::parse(".uk .pron").unwrap();
    // let us_pronunciation_selector = Selector::parse(".us .pron").unwrap();
    let example_selector = Selector::parse(".examp").unwrap();

    // output
    if let Some(word) = document.select(&word_selector).next() {
        println!("{}", style(word.text().collect::<Vec<_>>().join("")).bold().underlined().cyan());
    }
    if let Some(pos) = document.select(&pos_selector).next() {
        println!("{}", style(pos.text().collect::<Vec<_>>().join("")).green());
    }
    if let Some(pronunciation) = document.select(&uk_pronunciation_selector).next() {
        println!("{}", style(pronunciation.text().collect::<Vec<_>>().join("")).yellow());
    }
    if let Some(example) = document.select(&example_selector).next() {
        println!("Example: {}", style(example.text().collect::<Vec<_>>().join("")).italic().white());
    }

    Ok(())
}
