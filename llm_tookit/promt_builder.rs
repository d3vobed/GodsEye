extern crate reqwest;
extern crate scraper;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use scraper::{Html, Selector};

// Struct to represent extracted data (avoiding usernames/passwords)
#[derive(Debug)]
struct ExtractedData {
    words: HashSet<String>,
    emails: HashSet<String>, // Placeholder, not implemented for safety
}

impl ExtractedData {
    fn new() -> Self {
        ExtractedData {
            words: HashSet::new(),
            emails: HashSet::new(), // Placeholder
        }
    }
}

// Function to crawl a website and extract data
fn crawl_website(url: &str, depth: u32, max_word_length: u32, min_word_length: u32) -> Result<ExtractedData, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get(url).send()?;

    if response.status().is_success() {
        let body = response.text()?;
        let parsed_html = Html::parse_document(&body);

        let mut extracted_data = ExtractedData::new();

        // Extract words (filtered by length)
        let word_selector = Selector::all("text()");
        for word in parsed_html.select(&word_selector) {
            let word_text = word.inner_html().trim().to_string();
            if word_text.len() >= min_word_length as usize && word_text.len() <= max_word_length as usize {
                extracted_data.words.insert(word_text);
            }
        }

        // Placeholder for email extraction (not implemented)
        // let email_selector = Selector::and(Selector::Tag("a"), Selector::Attr("href", "mailto:"));
        // for email in parsed_html.select(&email_selector) {
        //     let email_text = email.attr("href").unwrap().trim_start_matches("mailto:");
        //     extracted_data.emails.insert(email_text.to_string());
        // }

        // Recursively crawl links within depth limit (avoid sensitive data)
        if depth > 0 {
            let link_selector = Selector::and(Selector::Tag("a"), Selector::Attr("href"));
            for link in parsed_html.select(&link_selector) {
                let mut new_url = link.attr("href").unwrap().to_string();
                if !new_url.starts_with("http") {
                    new_url = format!("{}/{}", url, new_url);
                }
                crawl_website(&new_url, depth - 1, max_word_length, min_word_length)?;
            }
        }

        Ok(extracted_data)
    } else {
        Err(reqwest::Error::new(format!("Failed to get response from URL: {}", url)))
    }
}

// Function to write extracted data to a wordlist file
fn write_wordlist(data: &ExtractedData, filename: &str) -> Result<(), std::io::Error> {
    let mut file = BufWriter::new(File::create(filename)?);
    for word in &data.words {
        writeln!(file, "{}", word)?;
    }
    Ok(())
}

// Usage example (modify options as needed)
fn main() {
    let url = "https://example.com";
    let depth = 1;
    let max_word_length = 10;
    let min_word_length = 3;
    let wordlist_file = "wordlist.txt";

    let extracted_data = crawl_website(url, depth, max_word_length, min_word_length).unwrap();
    write_wordlist(&extracted_data, wordlist_file).unwrap();

    println!("Wordlist saved to: {}", wordlist_file);
}
