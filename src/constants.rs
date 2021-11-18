pub mod constants {
    use regex::Regex;

    pub const DASH_APP_BASE_PATH: &str = "/Applications/Dash.app";
    pub const DASH_APP_CLI_PATH: &str = "/Contents/Resources/dashAlfredWorkflow";

    lazy_static! {
        pub static ref KEYWORD_PATTERN: Regex = Regex::new(r"^([a-zA-Z]+):.+").unwrap();
    }
}
