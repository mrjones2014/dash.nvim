extern crate roxmltree;

use regex::Regex;
use roxmltree::Document;
use serde::{Deserialize, Serialize};
use std::result::Result;
use std::{process::Command, string::FromUtf8Error};

#[derive(Serialize, Deserialize)]
pub struct TelescopeItem {
    pub value: String,
    pub title: String,
    pub display: String,
    pub keyword: String,
}

impl Clone for TelescopeItem {
    fn clone(&self) -> Self {
        return TelescopeItem {
            value: self.value.to_string(),
            title: self.title.to_string(),
            display: self.display.to_string(),
            keyword: self.keyword.to_string(),
        };
    }
}

pub async fn run_query(cli_path: &String, query: &String) -> Vec<TelescopeItem> {
    let raw_output = Command::new(cli_path)
        .args(&[query])
        .output()
        .expect("Failed to execute Dash.app CLI");
    let output_result: Result<String, FromUtf8Error> = String::from_utf8(raw_output.stdout);
    assert_eq!(output_result.is_ok(), true);
    let output = output_result.unwrap();

    let mut telescope_items = Vec::new();

    let xml_result = Document::parse(&output);
    let doc;
    match xml_result {
        Err(_) => return telescope_items,
        Ok(value) => doc = value,
    }

    let items_element = doc.descendants().find(|n| n.tag_name().name() == "items");

    let keyword_pattern = Regex::new(r"^([a-zA-Z]+):.+").unwrap();

    items_element.unwrap().children().for_each(|item| {
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
        telescope_items.push(TelescopeItem {
            value: item_value.to_string(),
            title: title.to_string(),
            display: title.to_string(),
            keyword: keyword.to_string(),
        });
    });
    return telescope_items;
}
