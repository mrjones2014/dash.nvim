use regex::Regex;

/// Default value for the path to Dash.app
pub const DASH_APP_BASE_PATH: &str = "/Applications/Dash.app";

/// The path within Dash.app to the CLI
pub const DASH_APP_CLI_PATH: &str = "/Contents/Resources/dashAlfredWorkflow";

/// The URL protocol used to open `DashItem`s in Dash.app
pub const DASH_CALLBACK_PROTO: &str = "dash-workflow-callback://";

lazy_static! {
    /// The Regular Expression used to parse keywords from queries in the form `keyword:rest of query`
    pub static ref KEYWORD_PATTERN: Regex = Regex::new(r"^([a-zA-Z]+):.+").unwrap();
}
