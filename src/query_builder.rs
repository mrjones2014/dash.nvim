use crate::constants;

/// Given search text and the list of configured keywords for current buffer type,
/// return the list of queries that should be run via the Dash.app CLI, with keywords
/// prepended to the search text
///
/// # Arguments
///
/// - `search_text` - the search text typed by the user, to be used as the query
/// - `configured_keywords` - the configured keywords for the current buffer type
pub fn build_queries(search_text: String, configured_keywords: &Vec<String>) -> Vec<String> {
    if configured_keywords.len() == 0 {
        return vec![search_text];
    }

    configured_keywords
        .into_iter()
        .map(|keyword| format!("{}:{}", keyword, search_text))
        .collect()
}

/// Given a query in the form `keyword:rest of query`, parse the keyword out of it.
/// If no keyword is present, returns an empty string.
pub fn parse_keyword_or_default(query: &str) -> String {
    if !constants::KEYWORD_PATTERN.is_match(query) {
        return String::from("");
    }

    let captures = constants::KEYWORD_PATTERN.captures(query);
    if captures.is_none() {
        return String::from("");
    }

    let first_capture = captures.unwrap().get(1);
    if first_capture.is_none() {
        return String::from("");
    }

    String::from(first_capture.unwrap().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_configured_keywords_is_empty_then_returns_search_text() {
        let queries = build_queries(String::from("search text"), &Vec::new());

        assert_eq!(1, queries.len());
        assert_eq!("search text", queries[0]);
    }

    #[test]
    fn when_configured_keywords_is_not_empty_then_returns_search_text_prefixed_with_each_query() {
        let keywords = vec![String::from("typescript"), String::from("javascript")];
        let queries = build_queries(String::from("search text"), &keywords);

        assert_eq!(2, queries.len());
        assert_eq!("typescript:search text", queries[0]);
        assert_eq!("javascript:search text", queries[1]);
    }
}
