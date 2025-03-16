use crate::entry::Bookmark;
use scraper::{Html, Selector};
use std::fs;

pub fn parse_bookmarks(html_file: &str) -> Vec<Bookmark> {
    let html = fs::read_to_string(html_file).expect("Failed to read file");
    let document = Html::parse_document(&html);

    let a_selector = Selector::parse("dt > a").unwrap();
    let mut bookmarks = Vec::new();

    for element in document.select(&a_selector) {
        let url = element.value().attr("href").unwrap_or("").to_string();
        let name = element.text().collect::<Vec<_>>().join(" ");
        /*
        let _timestamp = element
            .value()
            .attr("add_date")
            .and_then(|s| s.parse::<u64>().ok());
        */
        let id = 0; 
        let description = String::from("Imported");

        bookmarks.push(Bookmark { id, name, url, description });
    }

    bookmarks
}
