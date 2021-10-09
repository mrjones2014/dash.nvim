extern crate roxmltree;

use regex::Regex;
use roxmltree::Document;
use std::result::Result;
use std::{process::Command, string::FromUtf8Error};

pub async fn run_query(cli_path: &String, query: &String) -> String {
    let raw_output = Command::new(cli_path)
        .args(&[query])
        .output()
        .expect("Failed to execute Dash.app CLI");
    let output_result: Result<String, FromUtf8Error> = String::from_utf8(raw_output.stdout);
    assert_eq!(output_result.is_ok(), true);
    let output = output_result.unwrap();
    let doc = Document::parse(&output).unwrap();
    let items_element = doc.descendants().find(|n| n.tag_name().name() == "items");

    let keyword_pattern = Regex::new(r"^([a-zA-Z]+):.+").unwrap();

    let mut json_items = [].to_vec();

    &items_element.unwrap().children().for_each(|item| {
        let item_value = item
            .children()
            .find(|child| {
                child.tag_name().name() == "text" && child.attribute("type") == Some("copy")
            })
            .unwrap()
            .first_child()
            .unwrap()
            .text()
            .unwrap();
        let mut title = item
            .children()
            .find(|child| child.tag_name().name() == "title")
            .unwrap()
            .text()
            .unwrap()
            .to_owned();
        let subtitle = item
            .children()
            .filter(|child| child.tag_name().name() == "subtitle")
            .last()
            .unwrap()
            .text()
            .unwrap();
        title.push_str(": ");
        title.push_str(subtitle);

        let mut keyword = "";
        if keyword_pattern.is_match(&query) {
            keyword = &keyword_pattern
                .captures(&query)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();
        }
        let json_blob = format!(
            "{{ \"value\": {:?}, \"display\": {:?}, \"ordinal\": {:?}, \"keyword\": {:?} }}",
            item_value, title, title, keyword
        );
        json_items.push(json_blob);
    });
    return json_items.join(",");
}
