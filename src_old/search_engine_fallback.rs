use crate::constants::KEYWORD_PATTERN;
use regex::Regex;
use std::str::FromStr;

pub enum SearchEngine {
    DDG,
    STARTPAGE,
    GOOGLE,
}

impl FromStr for SearchEngine {
    type Err = ();

    fn from_str(input: &str) -> Result<SearchEngine, Self::Err> {
        return match input.to_lowercase().as_str() {
            "duckduckgo" => Ok(SearchEngine::DDG),
            "ddg" => Ok(SearchEngine::DDG),
            "startpage" => Ok(SearchEngine::STARTPAGE),
            "google" => Ok(SearchEngine::GOOGLE),
            _ => Ok(SearchEngine::DDG),
        };
    }
}

fn format_search_engine_query(query: &str) -> String {
    let keyword_pattern = Regex::new(KEYWORD_PATTERN).unwrap();
    let mut formatted_query = query.to_string();
    if keyword_pattern.is_match(&formatted_query) {
        let keyword = &keyword_pattern
            .captures(&formatted_query)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        // replace keyword:query syntax with regular "keyword query"
        // to put into a search engine
        formatted_query = formatted_query.replace(
            &format!("{}:", &keyword.to_string()),
            &format!("{} ", &keyword.to_string()),
        );
    }

    return formatted_query;
}

pub fn get_search_engine_url(search_engine: &SearchEngine, query: &str) -> String {
    return match search_engine {
        SearchEngine::DDG => format!(
            "https://duckduckgo.com/?q={}",
            format_search_engine_query(query)
        ),
        SearchEngine::STARTPAGE => format!(
            "https://startpage.com/sp/search?query={}",
            format_search_engine_query(query)
        ),
        SearchEngine::GOOGLE => format!(
            "https://www.google.com/search?q={}",
            format_search_engine_query(query)
        ),
    };
}
