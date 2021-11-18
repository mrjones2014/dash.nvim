use std::{fmt::Display, str::FromStr};

use crate::{dash_item::DashItem, query_builder::query_builder};

pub enum SearchEngine {
    DDG,
    STARTPAGE,
    GOOGLE,
}

impl SearchEngine {
    pub fn to_dash_item(&self, query: &str) -> DashItem {
        let keyword = query_builder::parse_keyword_or_default(&query);
        let search_engine_query_str = if keyword.len() > 0 {
            format!("{} {}", keyword, query)
        } else {
            String::from(query)
        };
        let url = match self {
            SearchEngine::DDG => format!("https://duckduckgo.com/?q={}", search_engine_query_str),
            SearchEngine::STARTPAGE => format!(
                "https://startpage.com/sp/search?query={}",
                search_engine_query_str
            ),
            SearchEngine::GOOGLE => format!(
                "https://www.google.com/search?q={}",
                search_engine_query_str
            ),
        };
        let title = format!("Search {} for: {}", format!("{}", self), query);

        DashItem {
            value: url,
            ordinal: String::from(&title),
            display: String::from(&title),
            keyword: String::from(""),
            query: String::from(query),
            is_fallback: true,
        }
    }
}

impl FromStr for SearchEngine {
    type Err = ();

    fn from_str(input: &str) -> Result<SearchEngine, Self::Err> {
        match input.to_lowercase().as_str() {
            "duckduckgo" => Ok(SearchEngine::DDG),
            "ddg" => Ok(SearchEngine::DDG),
            "startpage" => Ok(SearchEngine::STARTPAGE),
            "google" => Ok(SearchEngine::GOOGLE),
            _ => Ok(SearchEngine::DDG),
        }
    }
}

impl Display for SearchEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchEngine::DDG => write!(f, "DuckDuckGo"),
            SearchEngine::STARTPAGE => write!(f, "StartPage"),
            SearchEngine::GOOGLE => write!(f, "Google"),
        }
    }
}