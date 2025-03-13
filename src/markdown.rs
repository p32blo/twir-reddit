use html_escape::decode_html_entities;
use scraper::{Html, Selector};
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufRead, Write};

use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

struct RedditResponse {
    after: Option<String>,
    posts: Vec<RedditPost>,
}

#[derive(Debug, Clone)]
struct RedditPost {
    title: String,
    url: String,
    permalink: String,
    num_comments: i64,
    _ups: i64,
    _downs: i64,
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
    result
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
            title: data["title"].as_str().unwrap_or_default().to_string(),
            url: data["url"].as_str().unwrap_or_default().to_string(),
            permalink: format!(
                "http://www.reddit.com{permalink}",
                permalink = data["permalink"].as_str().unwrap_or_default(),
            ),
            num_comments: data["num_comments"].as_i64().unwrap_or_default(),
            _ups: data["ups"].as_i64().unwrap_or_default(),
            _downs: data["downs"].as_i64().unwrap_or_default(),
            score: data["score"].as_i64().unwrap_or_default(),
            links,
        }
    }
}

fn get_urls(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("opening file");
    let lines = io::BufReader::new(file).lines();
    lines
        .map_while(Result::ok)
        .map(|x| x.trim().trim_end_matches('/').to_string()) // cleanup
        .collect()
}

fn call(after: &Option<String>) -> Result<RedditResponse, ureq::Error> {
    let url = "http://www.reddit.com/r/rust/new.json";

    let mut req = ureq::get(url)
        .header("User-Agent", "Rust TWIR/0.0.1")
        .query("sort", "new");

    if let Some(param) = after {
        req = req.query("after", param)
    }

    let mut resp = req.call().expect("error getting response");
    let v: Value = resp.body_mut().read_json()?;

    let after = v["data"]["after"].as_str().map(|x| x.to_string());

    let mut posts: Vec<RedditPost> = vec![];
    if let Some(children) = v["data"]["children"].as_array() {
        posts = children.iter().map(RedditPost::from).collect()
    }

    Ok(RedditResponse { posts, after })
}

fn process(
    map: &mut HashMap<String, RedditPost>,
    posts: &[RedditPost],
    urls: &HashSet<String>,
) -> Option<String> {
    // return if a TWIR Post was found
    let mut res = None;

    for post in posts {
        if post.url.contains("this-week-in-rust.org") {
            res = Some(post.url.to_string());
        }

        let links = std::iter::once(&post.url)
            .chain(&post.links)
            .map(|x| x.trim_end_matches('/'));

        for link in links {
            if let Some(url) = urls.get(link) {
                map.insert(url.to_string(), post.clone());
            }
        }
    }

    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let filename = args.get(1).expect("Error getting the 1st arg");

    let ordered_urls = get_urls(filename);
    let urls: HashSet<String> = HashSet::from_iter(
        ordered_urls
            .iter()
            .filter(|&x| !x.starts_with("##"))
            .cloned(),
    );

    let mut map = HashMap::new();

    let mut token: Option<String> = None;

    let mut twir_post_count = 0;

    // End iterating when we have passed more than TWIR Posts
    // (This makes it so two weeks of posts are included)
    while twir_post_count < 3 {
        // Call API and update token for next page
        let RedditResponse { posts, after } = call(&token)?;
        token = after;

        // Count number of TWIR Posts
        if let Some(_) = process(&mut map, &posts, &urls) {
            twir_post_count += 1;
        }

        // Print progress
        print!(" ");
        io::stdout().flush().expect("Unable to flush stdout");
    }

    print_header();
    print_result(&ordered_urls, &map);
    print_footer();

    Ok(())
}

fn print_header() {
    println!(
        r#"
# TWIR @ Reddit

Hey everyone, here you can follow the r/rust comment threads of articles featured in TWIR (This Week in Rust).
I've always found it helpful to search for additional insights in the comment section here
and I hope you can find it helpful too.
Enjoy !
"#,
    );
}

fn print_footer() {
    println!(
        r#"
--

If you are curious how this comment is generated you can now check https://github.com/p32blo/twir-reddit

Also, I'm looking for a Rust job opportunity!
If you know anyone interested in a Remote Developer in Europe, you can contact me at p32blo@gmail.com.
Thank you!
"#,
    );
}

fn print_result(urls: &[String], map: &HashMap<String, RedditPost>) {
    let mut section = None;
    for url in urls {
        if url.starts_with("##") {
            section = Some(url);
        } else if let Some(post) = map.get(url) {
            if let Some(url) = section.take() {
                println!("\n{}", url);
            }
            print_post(post);
        }
    }
}

fn print_post(post: &RedditPost) {
    println!(
        "- [{title}]({url}) `↑{score} | {num_comments} comment{plural}`",
        title = post.title,
        url = post.permalink,
        score = post.score,
        num_comments = post.num_comments,
        plural = if post.num_comments > 1 { "s" } else { "" },
    );
}
