use roxmltree::Document;
use std::fmt::{Display, Formatter};

use crate::query_builder::query_builder;

/// Item returned from the Rust backend.
///
/// # Fields
///
/// - `value` -- the number value of the item, to be used when selected. Running a query, then opening the URL `dash-workflow-callback://[value]` will open the selected item in Dash.app
/// - `ordinal` -- a value to sort by, currently this is the same value as `display`
/// - `display` -- a display value
/// - `keyword` -- the keyword (if there was one) on the query that returned this result
/// - `query` -- the full query that returned this result
#[derive(Clone, Debug, PartialEq)]
pub struct DashItem {
    pub value: String,
    pub ordinal: String,
    pub display: String,
    pub keyword: String,
    pub query: String,
    pub is_fallback: bool,
}

#[derive(Debug, PartialEq)]
pub enum DashItemCreationError {
    XmlParsingError(roxmltree::Error),
    XmlMissingData(String),
}

impl Display for DashItemCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DashItemCreationError::XmlParsingError(e) => {
                write!(f, "Failed to parse XML string: {}", e)
            }
            DashItemCreationError::XmlMissingData(value) => {
                write!(f, "XML is missing data: {}", value)
            }
        }
    }
}

impl std::error::Error for DashItemCreationError {}

impl From<roxmltree::Error> for DashItemCreationError {
    fn from(e: roxmltree::Error) -> Self {
        DashItemCreationError::XmlParsingError(e)
    }
}

impl DashItem {
    /// Create a Vec of DashItems from an XML string
    /// that contains multiple XML representations
    /// of items.
    ///
    /// # Example
    ///
    /// ```
    /// let xml = dash_app_connector::get_xml(&cli_path, &query)?;
    /// let dash_items: Vec<DashItem> = DashItem::try_from_xml(xml)?;
    /// ```
    pub fn try_from_xml(xml: String, query: &str) -> Result<Vec<Self>, DashItemCreationError> {
        let xml_tree = Document::parse(&xml)?;
        let items_node = xml_tree
            .descendants()
            .find(|node| node.tag_name().name() == "items")
            .ok_or(DashItemCreationError::XmlMissingData(String::from(
                "<items> node",
            )))?;
        let item_keyword = query_builder::parse_keyword_or_default(query);

        let mut dash_items: Vec<DashItem> = Vec::new();
        for node in items_node.children().into_iter() {
            let relevant_tags = node.children().filter(|child| {
                let tag_name = child.tag_name().name();
                return tag_name == "text" || tag_name == "title" || tag_name == "subtitle";
            });

            let item_value = node
                .attribute("arg")
                .ok_or(DashItemCreationError::XmlMissingData(String::from(
                    r#"attribute "arg""#,
                )))?;

            let mut title_option: Option<&str> = None;
            let mut subtitle_option: Option<&str> = None;

            relevant_tags.for_each(|child| match child.tag_name().name() {
                "title" => title_option = Some(child.text().unwrap()),
                "subtitle" => {
                    if child.attributes().len() == 0 {
                        subtitle_option = Some(child.text().unwrap())
                    }
                }
                _ => {}
            });

            let title = title_option.ok_or(DashItemCreationError::XmlMissingData(String::from(
                "<title> node",
            )))?;
            let subtitle = subtitle_option.ok_or(DashItemCreationError::XmlMissingData(
                String::from("<subtitle> node"),
            ))?;

            let item_title = format!("{}: {}", title, subtitle);

            dash_items.push(DashItem {
                value: String::from(item_value),
                ordinal: String::from(&item_title),
                display: String::from(&item_title),
                keyword: String::from(&item_keyword),
                query: String::from(query),
                is_fallback: false,
            });
        }

        Ok(dash_items)
    }
}

mod tests {
    use super::*;

    fn minify(xml: &str) -> String {
        xml.split("\n")
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .join("")
    }

    #[test]
    fn parses_valid_xml_structure() {
        // Valid XML output from query "react:useState hook"
        let valid_xml_str = self::minify(
            r#"
            <?xml version="1.0"?>
            <output>
              <rerun>0.1</rerun>
              <is-licensed>1</is-licensed>
              <items>
                <item uid="dash-advanced://useState%20-%20Hooks%20API%20Reference/Section/file:///Users/mat/Library/Application%20Support/Dash/DocSets/React/React.docset/Contents/Resources/Documents/reactjs.org/docs/hooks-reference.html%23//dash_ref_776/Section/useState/0" arg="0" autocomplete="useState - Hooks API Reference">
                  <title>useState - Hooks API Reference</title>
                  <text type="copy">useState - Hooks API Reference</text>
                  <text type="largetype">useState - Hooks API Reference</text>
                  <subtitle mod="cmd">Open "useState - Hooks API Reference" in browser</subtitle>
                  <subtitle mod="alt">Copy "useState - Hooks API Reference" to clipboard</subtitle>
                  <subtitle>React - useState - Hooks API Reference</subtitle>
                  <icon>/Users/mat/Library/Application Support/Dash/Data/Alfred/6b89bf46e15350780f7af21afb6a5d57.png</icon>
                  <quicklookurl>http://127.0.0.1:58920/Dash/aefwrwzu/reactjs.org/docs/hooks-reference.html#//dash_ref_776/Section/useState/0</quicklookurl>
                </item>
                <item uid="dash-advanced://What%20does%20const%20%5Bthing,%20setThing%5D%20=%20useState()%20mean?%20-%20Hooks%20FAQ/Section/file:///Users/mat/Library/Application%20Support/Dash/DocSets/React/React.docset/Contents/Resources/Documents/reactjs.org/docs/hooks-faq.html%23//dash_ref_169/Section/What%2520does%2520const%2520%255Bthing%252C%2520setThing%255D%2520%253D%2520useState%2528%2529%2520mean%253F/0" arg="1" autocomplete="What does const [thing, setThing] = useState() mean? - Hooks FAQ">
                  <title>What does const [thing, setThing] = useState() mean? - Hooks FAQ</title>
                  <text type="copy">What does const [thing, setThing] = useState() mean? - Hooks FAQ</text>
                  <text type="largetype">What does const [thing, setThing] = useState() mean? - Hooks FAQ</text>
                  <subtitle mod="cmd">Open "What does const [thing, setThing] = useState() mean? - Hooks FAQ" in browser</subtitle>
                  <subtitle mod="alt">Copy "What does const [thing, setThing] = useState() mean? - Hooks FAQ" to clipboard</subtitle>
                  <subtitle>React - What does const [thing, setThing] = useState() mean? - Hooks FAQ</subtitle>
                  <icon>/Users/mat/Library/Application Support/Dash/Data/Alfred/6b89bf46e15350780f7af21afb6a5d57.png</icon>
                  <quicklookurl>http://127.0.0.1:58920/Dash/aefwrwzu/reactjs.org/docs/hooks-faq.html#//dash_ref_169/Section/What%20does%20const%20%5Bthing%2C%20setThing%5D%20%3D%20useState%28%29%20mean%3F/0</quicklookurl>
                </item>
              </items>
            </output>
            "#,
        );

        let items = DashItem::try_from_xml(valid_xml_str, "ignored");

        assert_eq!(true, items.is_ok());
        assert_eq!(2, items.unwrap().len());
    }

    #[test]
    fn parses_keyword_from_query_when_present() {
        // Valid XML output from query "react:useState hook"
        let valid_xml_str = self::minify(
            r#"
            <?xml version="1.0"?>
            <output>
              <rerun>0.1</rerun>
              <is-licensed>1</is-licensed>
              <items>
                <item uid="dash-advanced://useState%20-%20Hooks%20API%20Reference/Section/file:///Users/mat/Library/Application%20Support/Dash/DocSets/React/React.docset/Contents/Resources/Documents/reactjs.org/docs/hooks-reference.html%23//dash_ref_776/Section/useState/0" arg="0" autocomplete="useState - Hooks API Reference">
                  <title>useState - Hooks API Reference</title>
                  <text type="copy">useState - Hooks API Reference</text>
                  <text type="largetype">useState - Hooks API Reference</text>
                  <subtitle mod="cmd">Open "useState - Hooks API Reference" in browser</subtitle>
                  <subtitle mod="alt">Copy "useState - Hooks API Reference" to clipboard</subtitle>
                  <subtitle>React - useState - Hooks API Reference</subtitle>
                  <icon>/Users/mat/Library/Application Support/Dash/Data/Alfred/6b89bf46e15350780f7af21afb6a5d57.png</icon>
                  <quicklookurl>http://127.0.0.1:58920/Dash/aefwrwzu/reactjs.org/docs/hooks-reference.html#//dash_ref_776/Section/useState/0</quicklookurl>
                </item>
                <item uid="dash-advanced://What%20does%20const%20%5Bthing,%20setThing%5D%20=%20useState()%20mean?%20-%20Hooks%20FAQ/Section/file:///Users/mat/Library/Application%20Support/Dash/DocSets/React/React.docset/Contents/Resources/Documents/reactjs.org/docs/hooks-faq.html%23//dash_ref_169/Section/What%2520does%2520const%2520%255Bthing%252C%2520setThing%255D%2520%253D%2520useState%2528%2529%2520mean%253F/0" arg="1" autocomplete="What does const [thing, setThing] = useState() mean? - Hooks FAQ">
                  <title>What does const [thing, setThing] = useState() mean? - Hooks FAQ</title>
                  <text type="copy">What does const [thing, setThing] = useState() mean? - Hooks FAQ</text>
                  <text type="largetype">What does const [thing, setThing] = useState() mean? - Hooks FAQ</text>
                  <subtitle mod="cmd">Open "What does const [thing, setThing] = useState() mean? - Hooks FAQ" in browser</subtitle>
                  <subtitle mod="alt">Copy "What does const [thing, setThing] = useState() mean? - Hooks FAQ" to clipboard</subtitle>
                  <subtitle>React - What does const [thing, setThing] = useState() mean? - Hooks FAQ</subtitle>
                  <icon>/Users/mat/Library/Application Support/Dash/Data/Alfred/6b89bf46e15350780f7af21afb6a5d57.png</icon>
                  <quicklookurl>http://127.0.0.1:58920/Dash/aefwrwzu/reactjs.org/docs/hooks-faq.html#//dash_ref_169/Section/What%20does%20const%20%5Bthing%2C%20setThing%5D%20%3D%20useState%28%29%20mean%3F/0</quicklookurl>
                </item>
              </items>
            </output>
            "#,
        );

        let items_result = DashItem::try_from_xml(valid_xml_str, "react:useState hook");

        assert_eq!(true, items_result.is_ok());
        let items = items_result.unwrap();
        assert_eq!(2, items.len());
        assert_eq!("react", items[0].keyword);
        assert_eq!("react:useState hook", items[0].query);
    }

    #[test]
    fn fails_when_invalid_xml() {
        let items_result = DashItem::try_from_xml(self::minify("invalid xml string"), "ignored");

        let mut is_xml_parse_error = false;
        match items_result.unwrap_err() {
            DashItemCreationError::XmlParsingError(_) => is_xml_parse_error = true,
            _ => {}
        }

        assert!(is_xml_parse_error);
    }

    #[test]
    fn fails_when_item_has_no_arg_attr() {
        let xml_str = self::minify(
            r#"<?xml version="1.0"?>
            <output>
              <rerun>0.1</rerun>
              <is-licensed>1</is-licensed>
              <items>
                <item uid="dash-advanced://useState%20-%20Hooks%20API%20Reference/Section/file:///Users/mat/Library/Application%20Support/Dash/DocSets/React/React.docset/Contents/Resources/Documents/reactjs.org/docs/hooks-reference.html%23//dash_ref_776/Section/useState/0" autocomplete="useState - Hooks API Reference">
                  <title>useState - Hooks API Reference</title>
                  <text type="copy">useState - Hooks API Reference</text>
                  <text type="largetype">useState - Hooks API Reference</text>
                  <subtitle mod="cmd">Open "useState - Hooks API Reference" in browser</subtitle>
                  <subtitle mod="alt">Copy "useState - Hooks API Reference" to clipboard</subtitle>
                  <subtitle>React - useState - Hooks API Reference</subtitle>
                  <icon>/Users/mat/Library/Application Support/Dash/Data/Alfred/6b89bf46e15350780f7af21afb6a5d57.png</icon>
                  <quicklookurl>http://127.0.0.1:58920/Dash/aefwrwzu/reactjs.org/docs/hooks-reference.html#//dash_ref_776/Section/useState/0</quicklookurl>
                </item>
              </items>
            </output>
            "#,
        );

        let items_result = DashItem::try_from_xml(xml_str, "ignored");

        assert_eq!(
            Err(DashItemCreationError::XmlMissingData(String::from(
                r#"attribute "arg""#
            ))),
            items_result
        );
    }

    #[test]
    fn fails_when_item_has_no_title_node() {
        let xml_str = self::minify(
            r#"<?xml version="1.0"?>
            <output>
              <rerun>0.1</rerun>
              <is-licensed>1</is-licensed>
              <items>
                <item uid="dash-advanced://useState%20-%20Hooks%20API%20Reference/Section/file:///Users/mat/Library/Application%20Support/Dash/DocSets/React/React.docset/Contents/Resources/Documents/reactjs.org/docs/hooks-reference.html%23//dash_ref_776/Section/useState/0" arg="0" autocomplete="useState - Hooks API Reference">
                  <text type="copy">useState - Hooks API Reference</text>
                  <text type="largetype">useState - Hooks API Reference</text>
                  <subtitle mod="cmd">Open "useState - Hooks API Reference" in browser</subtitle>
                  <subtitle mod="alt">Copy "useState - Hooks API Reference" to clipboard</subtitle>
                  <subtitle>React - useState - Hooks API Reference</subtitle>
                  <icon>/Users/mat/Library/Application Support/Dash/Data/Alfred/6b89bf46e15350780f7af21afb6a5d57.png</icon>
                  <quicklookurl>http://127.0.0.1:58920/Dash/aefwrwzu/reactjs.org/docs/hooks-reference.html#//dash_ref_776/Section/useState/0</quicklookurl>
                </item>
              </items>
            </output>
            "#,
        );

        let items_result = DashItem::try_from_xml(xml_str, "ignored");
        assert_eq!(
            Err(DashItemCreationError::XmlMissingData(String::from(
                "<title> node"
            ))),
            items_result
        );
    }

    #[test]
    fn fails_when_item_has_no_subtitle_node_without_attrs() {
        let xml_str = self::minify(
            r#"
            <?xml version="1.0"?>
            <output>
              <rerun>0.1</rerun>
              <is-licensed>1</is-licensed>
              <items>
                <item uid="dash-advanced://useState%20-%20Hooks%20API%20Reference/Section/file:///Users/mat/Library/Application%20Support/Dash/DocSets/React/React.docset/Contents/Resources/Documents/reactjs.org/docs/hooks-reference.html%23//dash_ref_776/Section/useState/0" arg="0" autocomplete="useState - Hooks API Reference">
                  <title>useState - Hooks API Reference</title>
                  <text type="copy">useState - Hooks API Reference</text>
                  <text type="largetype">useState - Hooks API Reference</text>
                  <subtitle mod="cmd">Open "useState - Hooks API Reference" in browser</subtitle>
                  <subtitle mod="alt">Copy "useState - Hooks API Reference" to clipboard</subtitle>
                  <icon>/Users/mat/Library/Application Support/Dash/Data/Alfred/6b89bf46e15350780f7af21afb6a5d57.png</icon>
                  <quicklookurl>http://127.0.0.1:58920/Dash/aefwrwzu/reactjs.org/docs/hooks-reference.html#//dash_ref_776/Section/useState/0</quicklookurl>
                </item>
              </items>
            </output>
            "#,
        );

        let items_result = DashItem::try_from_xml(xml_str, "ignored");
        assert_eq!(
            Err(DashItemCreationError::XmlMissingData(String::from(
                "<subtitle> node"
            ))),
            items_result
        );
    }
}
