use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;

fn extract_urls(document_content: &str) -> Vec<String> {
    let mut result = vec![];

    let html = Html::parse_document(document_content);

    let selector_h3 = Selector::parse("h3").unwrap();
    let selector_a = Selector::parse("a").unwrap();

    for elem in html.select(&selector_h3) {
        result.push(format!("##{}", elem.text().collect::<Vec<_>>().join("")).to_string());

        let ul = elem
            .next_siblings()
            .filter_map(ElementRef::wrap)
            .find(|&x| x.value().name() == "ul");

        if let Some(node) = ul {
            for elem in node.select(&selector_a) {
                if let Some(attr) = elem.value().attr("href") {
                    result.push(attr.to_string());
                }
            }
        }
    }
    result
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filename = args.get(1).expect("Error getting the 1st arg");
    let file = std::fs::read_to_string(filename);
    for line in extract_urls(&file.unwrap()) {
        println!("{}", line);
    }
}
