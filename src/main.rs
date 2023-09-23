extern crate scraper;
use scraper::{ Html, Selector };

fn main() {
    let url = "https://tpl.ca";
    let response = reqwest::blocking::get(url);
    let html_content = response.unwrap().text().unwrap();
    let document = Html::parse_document(&html_content);

    let form_selector = Selector::parse("form#searchForm").unwrap();
    let form = document.select(&form_selector).next().unwrap();
    let span_selector = Selector::parse("span").unwrap();
    let span = form.select(&span_selector).next().unwrap();
    let input_selector = Selector::parse("input").unwrap();
    let input = span.select(&input_selector).next().unwrap();
    let input_placeholder = input.value().attr("placeholder").unwrap();
    println!("Search field placeholder: {:?}", input_placeholder);
}
