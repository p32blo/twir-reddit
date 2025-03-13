use feed_rs::parser;
use feed_rs::parser::ParseFeedResult;

use std::error::Error;

fn extract_urls(bytes: &[u8]) -> ParseFeedResult<()> {
    let feed = parser::parse(bytes)?;
    let first = &feed.entries[0];

    if let Some(content) = &first.content {
        if let Some(body) = &content.body {
            println!("{}", &body)
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let filename = args.get(1).expect("Error getting the 1st arg");
    let bytes = std::fs::read(filename)?;

    Ok(extract_urls(&bytes)?)
}
