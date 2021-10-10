extern crate roxmltree;

use regex::Regex;
use roxmltree::Document;
use serde::Serialize;
use std::result::Result;
use std::{process::Command, string::FromUtf8Error};

#[derive(Serialize)]
pub struct TelescopeItem {
    pub value: String,
    pub ordinal: String,
    pub display: String,
    pub keyword: String,
    pub query: String,
}

impl Clone for TelescopeItem {
    fn clone(&self) -> Self {
        return TelescopeItem {
            value: self.value.to_string(),
            ordinal: self.ordinal.to_string(),
            display: self.display.to_string(),
            keyword: self.keyword.to_string(),
            query: self.query.to_string(),
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
        let relevant_tags = item.children().filter(|child| {
            let tag_name = child.tag_name().name();
            return tag_name == "text" || tag_name == "title" || tag_name == "subtitle";
        });
        let item_value: String = item.attribute("arg").unwrap().to_string();
        let mut title: String = "".to_string();
        let mut subtitle: String = "".to_string();
        relevant_tags.for_each(|child| match child.tag_name().name() {
            "title" => title = child.text().unwrap().to_string(),
            "subtitle" => subtitle = child.text().unwrap().to_string(),
            _ => {}
        });

        assert_ne!(item_value, "");
        assert_ne!(title, "");
        assert_ne!(subtitle, "");

        title = format!("{}: {}", title, subtitle);

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
            ordinal: title.to_string(),
            display: title.to_string(),
            keyword: keyword.to_string(),
            query: query.to_string(),
        });
    });
    return telescope_items;
}
