use std::fs::File;
use std::io::{self, BufRead};

fn get_urls(filename: &str) -> Vec<String> {
    let file = File::open(&filename).expect("opening file");
    let lines = io::BufReader::new(file).lines();
    lines
        .map_while(Result::ok)
        .map(|x| x.trim().trim_end_matches("/").to_string()) // cleanup
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filename = args.get(1).expect("Error getting the 1st arg");

    let urls = get_urls(filename);

    dbg!(urls);
}
