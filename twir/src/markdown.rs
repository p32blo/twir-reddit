use serde_json::Value;
use std::fs::File;
use std::io::{self, BufRead};
use ureq;

use ureq::http::HeaderValue;

#[derive(Debug)]
struct RedditPost {
    title: String,
    url: String,
    num_comments: i64,
    ups: i64,
    downs: i64,
    score: i64,
}

fn get_urls(filename: &str) -> Vec<String> {
    let file = File::open(&filename).expect("opening file");
    let lines = io::BufReader::new(file).lines();
    lines
        .map_while(Result::ok)
        .map(|x| x.trim().trim_end_matches("/").to_string()) // cleanup
        .collect()
}

fn call(after: Option<&str>) -> Result<(), ureq::Error> {
    let url = "http://www.reddit.com/r/rust/new.json";

    let mut req = ureq::get(url).header("sort", "new");

    if let Some(param) = after {
        let headers = req.headers_mut().unwrap();
        headers.insert("after", HeaderValue::from_str(param).unwrap());
    }

    let mut resp = ureq::get(url).call().expect("error getting response");
    let v: Value = resp.body_mut().read_json()?;

    if let Some(children) = v["data"]["children"].as_array() {
        for child in children {
            let data = &child["data"];
            let post = RedditPost {
                title: data["title"].as_str().unwrap_or("").to_string(),
                url: data["url"].as_str().unwrap_or("").to_string(),
                num_comments: data["num_comments"].as_i64().unwrap_or(0),
                ups: data["ups"].as_i64().unwrap_or(0),
                downs: data["downs"].as_i64().unwrap_or(0),
                score: data["score"].as_i64().unwrap_or(0),
            };
            dbg!(post);
        }
    }
    return Ok(());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filename = args.get(1).expect("Error getting the 1st arg");

    let urls = get_urls(filename);

    let _ = dbg!(call(None));
}
