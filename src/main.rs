mod cli_runner;

extern crate argparse;
use argparse::{ArgumentParser, Collect, Store};

fn main() {
    let mut cli_path = "/Applications/Dash.app".to_string().to_owned();
    let mut queries: Vec<String> = [].to_vec();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Run queries in Dash.app");
        ap.refer(&mut cli_path).add_option(
            &["-c", "--cli-path"],
            Store,
            "The path to the Dash.app CLI.",
        );
        ap.refer(&mut queries)
            .add_argument("queries", Collect, "Queries to run");
        ap.parse_args_or_exit();
    }

    cli_path.push_str("/Contents/Resources/dashAlfredWorkflow");
    for query in &queries {
        cli_runner::run_query(&cli_path.to_string(), &query.to_string());
    }
}
