use crate::constants;
use argparse::{ArgumentParser, Collect, Store, StoreTrue};

pub struct DashArgs {
    pub cli_path: String,
    pub queries: Vec<String>,
    pub pretty_print: bool,
}

pub fn parse_args() -> DashArgs {
    let mut cli_path = constants::DASH_APP_BASE_PATH.to_owned();
    let mut queries: Vec<String> = [].to_vec();
    let mut pretty_print: bool = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Run queries in Dash.app");
        ap.refer(&mut cli_path).add_option(
            &["-c", "--cli-path"],
            Store,
            "The path to the Dash.app CLI.",
        );
        ap.refer(&mut pretty_print).add_option(
            &["--pretty-print"],
            StoreTrue,
            "Pretty-print the JSON output",
        );
        ap.refer(&mut queries)
            .add_argument("queries", Collect, "Queries to run");
        ap.parse_args_or_exit();
    }

    cli_path.push_str(constants::DASH_APP_CLI_PATH);

    return DashArgs {
        cli_path,
        queries,
        pretty_print,
    };
}
