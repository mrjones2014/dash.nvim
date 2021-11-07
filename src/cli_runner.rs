extern crate roxmltree;

use regex::Regex;
use roxmltree::Document;
use std::result::Result;
use std::{process::Command, string::FromUtf8Error};

use crate::constants::KEYWORD_PATTERN;

pub struct DashItem {
    pub value: String,
    pub ordinal: String,
    pub display: String,
    pub keyword: String,
    pub query: String,
    pub preview_url: String,
}

impl Clone for DashItem {
    fn clone(&self) -> Self {
        return DashItem {
            value: self.value.to_string(),
            ordinal: self.ordinal.to_string(),
            display: self.display.to_string(),
            keyword: self.keyword.to_string(),
            query: self.query.to_string(),
            preview_url: self.preview_url.to_string(),
        };
    }
}

fn remove_rsquo_entities(input: &str) -> String {
    return input.replace("&rsquo;", "'");
}

pub async fn run_query(cli_path: &str, query: &str) -> Vec<DashItem> {
    let raw_output = Command::new(cli_path)
        .args(&[query])
        .output()
        .expect("Failed to execute Dash.app CLI");
    let output_result: Result<String, FromUtf8Error> = String::from_utf8(raw_output.stdout);

    if !output_result.is_ok() {
        return Vec::new();
    }

    let output = remove_rsquo_entities(&output_result.unwrap());
    let mut dash_items = Vec::new();

    let xml_result = Document::parse(&output);
    let doc;
    match xml_result {
        Err(_) => return dash_items,
        Ok(value) => doc = value,
    }

    let items_element = doc.descendants().find(|n| n.tag_name().name() == "items");

    let keyword_pattern = Regex::new(KEYWORD_PATTERN).unwrap();

    for item in items_element.unwrap().children() {
        let relevant_tags = item.children().filter(|child| {
            let tag_name = child.tag_name().name();
            return tag_name == "text"
                || tag_name == "title"
                || tag_name == "subtitle"
                || tag_name == "quicklookurl";
        });
        let item_value: String = item.attribute("arg").unwrap().to_string();
        let mut title: String = "".to_string();
        let mut subtitle: String = "".to_string();
        let mut preview_url: String = "".to_string();
        relevant_tags.for_each(|child| match child.tag_name().name() {
            "title" => title = child.text().unwrap().to_string(),
            "subtitle" => subtitle = child.text().unwrap().to_string(),
            "quicklookurl" => preview_url = child.text().unwrap().to_string(),
            _ => {}
        });

        if item_value == "" || title == "" || subtitle == "" {
            continue;
        }

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
        dash_items.push(DashItem {
            value: item_value.to_string(),
            ordinal: title.to_string(),
            display: title.to_string(),
            keyword: keyword.to_string(),
            query: query.to_string(),
            preview_url: preview_url.to_string(),
        });
    }
    return dash_items;
}
