use std::{fmt::Display, str::FromStr};

use crate::{dash_item::DashItem, query_builder};

/// Defines supported search engine fallbacks.
/// Currently supported are:
///
/// - DuckDuckGo
/// - StartPage
/// - Google
///
/// Falls back to DuckDuckGo if no or an invalid value is configured.
#[derive(Debug, PartialEq)]
pub enum SearchEngine {
    DDG,
    STARTPAGE,
    GOOGLE,
}

impl SearchEngine {
    /// Convert a `SearchEngine` to a `DashItem`,
    /// using the provided query in the search engine URL,
    /// which will be the set as the `value` field of the `DashItem`
    pub fn to_dash_item(&self, query: &str) -> DashItem {
        let keyword = query_builder::parse_keyword_or_default(query);
        let search_engine_query_str = if !keyword.is_empty() {
            let str_to_replace = format!("{}:", keyword);
            let query_without_keyword = query.replace(&str_to_replace, "");
            format!("{} {}", keyword, query_without_keyword)
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
        let title = format!(
            "Search {} for: {}",
            format!("{}", self),
            search_engine_query_str
        );

        DashItem {
            value: url,
            ordinal: String::from(&title),
            display: String::from(&title),
            keyword,
            query: String::from(query),
            is_fallback: true,
        }
    }
}

impl FromStr for SearchEngine {
    type Err = ();

    /// Convert a string to SearchEngine enum. Will not panic,
    /// instead will default to SearchEngine::DDG
    fn from_str(input: &str) -> Result<SearchEngine, Self::Err> {
        match input.to_lowercase().as_str() {
            "startpage" => Ok(SearchEngine::STARTPAGE),
            "google" => Ok(SearchEngine::GOOGLE),
            // DDG by default
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn from_str_when_no_match_defaults_to_ddg() {
        let search_engine = "no match".parse::<SearchEngine>().unwrap();
        assert_eq!(SearchEngine::DDG, search_engine);
    }

    #[test]
    fn from_str_is_not_case_sensitive() {
        // pairs of (input, expected_result)
        let test_data = HashMap::from([
            ("google", SearchEngine::GOOGLE),
            ("GOOGLE", SearchEngine::GOOGLE),
            ("gOoGLe", SearchEngine::GOOGLE),
            ("startpage", SearchEngine::STARTPAGE),
            ("STARTPAGE", SearchEngine::STARTPAGE),
            ("StArTpAgE", SearchEngine::STARTPAGE),
            ("ddg", SearchEngine::DDG),
            ("duckduckgo", SearchEngine::DDG),
            ("DDG", SearchEngine::DDG),
            ("DUCKDUCKGO", SearchEngine::DDG),
            ("DdG", SearchEngine::DDG),
            ("DuCkDuCkGo", SearchEngine::DDG),
        ]);

        for (input, expected_result) in test_data.iter() {
            assert_eq!(expected_result, &input.parse::<SearchEngine>().unwrap());
        }
    }

    #[test]
    fn to_dash_item_replaces_keyword_colon_with_keyword_space() {
        let query = "rust:match arms";
        let dash_item = SearchEngine::DDG.to_dash_item(query);
        assert!(dash_item.value.ends_with("=rust match arms"));
    }

    #[test]
    fn to_dash_item_appends_query_unchanged_when_no_keyword() {
        let query = "match arms";
        let dash_item = SearchEngine::DDG.to_dash_item(query);
        assert!(dash_item.value.ends_with("=match arms"));
    }

    #[test]
    fn to_dash_item_sets_is_fallback_true() {
        assert!(SearchEngine::DDG.to_dash_item("test").is_fallback);
    }
}
