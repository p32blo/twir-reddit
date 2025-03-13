use html_escape::decode_html_entities;
use scraper::{Html, Selector};
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufRead};
use ureq;

use std::collections::HashSet;
use std::error::Error;
use ureq::http::HeaderValue;

struct RedditResponse {
    after: String,
    posts: Vec<RedditPost>,
}

#[derive(Debug, Clone)]
struct RedditPost {
    title: String,
    url: String,
    permalink: String,
    num_comments: i64,
    ups: i64,
    downs: i64,
    score: i64,
    links: Vec<String>,
}

fn extract_urls(html: &str) -> Vec<String> {
    let mut result = vec![];

    let html = Html::parse_document(html);

    let selector_a = Selector::parse("a").unwrap();

    for elem in html.select(&selector_a) {
        if let Some(attr) = elem.value().attr("href") {
            result.push(attr.to_string());
        }
    }
    return result;
}

impl From<&Value> for RedditPost {
    fn from(item: &Value) -> Self {
        let data = &item["data"];

        let mut links = vec![];

        if let Some(html) = data["selftext_html"].as_str() {
            // Unescape the HTML
            let html = &decode_html_entities(html);

            // Extract links in the unescaped HTML
            links = extract_urls(html);
        }

        RedditPost {
            title: data["title"].as_str().unwrap_or("").to_string(),
            url: data["url"].as_str().unwrap_or("").to_string(),
            permalink: data["permalink"].as_str().unwrap_or("").to_string(),
            num_comments: data["num_comments"].as_i64().unwrap_or(0),
            ups: data["ups"].as_i64().unwrap_or(0),
            downs: data["downs"].as_i64().unwrap_or(0),
            score: data["score"].as_i64().unwrap_or(0),
            links,
        }
    }
}

fn get_urls(filename: &str) -> Vec<String> {
    let file = File::open(&filename).expect("opening file");
    let lines = io::BufReader::new(file).lines();
    lines
        .map_while(Result::ok)
        .map(|x| x.trim().trim_end_matches("/").to_string()) // cleanup
        .collect()
}

fn call(after: Option<&str>) -> Result<RedditResponse, ureq::Error> {
    dbg!(&after);
    let url = "http://www.reddit.com/r/rust/new.json";

    let mut req = ureq::get(url).header("sort", "new");

    if let Some(param) = after {
        let headers = req.headers_mut().unwrap();
        headers.insert("after", HeaderValue::from_str(param).unwrap());
        headers.insert("User-Agent", HeaderValue::from_static("Rust TWIR/0.0.1"));
    }

    let mut resp = ureq::get(url).call().expect("error getting response");
    let v: Value = resp.body_mut().read_json()?;

    let after = v["data"]["after"].as_str().unwrap_or_default().to_string();

    let mut posts: Vec<RedditPost> = vec![];
    if let Some(children) = v["data"]["children"].as_array() {
        posts = children.iter().map(RedditPost::from).collect()
    }

    Ok(RedditResponse { posts, after })
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let filename = args.get(1).expect("Error getting the 1st arg");

    let urls: HashSet<String> = HashSet::from_iter(
        get_urls(filename)
            .into_iter()
            .filter(|x| !x.starts_with("##")),
    );
    let res = call(None)?;

    for post in res.posts {
        let orig_post = post.clone();

        let links: Vec<String> = std::iter::once(post.url)
            .chain(post.links.into_iter())
            .map(|x| x.trim_end_matches("/").to_string())
            .collect();

        for link in &links {
            //dbg!(link);
            if urls.contains(link) {
                dbg!(&orig_post.title);
            }
        }
    }
    let mut token = res.after;
    for _ in 0..5 {
        let resp = call(Some(&token))?;

        token = resp.after;
    }
    Ok(())
}
