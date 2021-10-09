extern crate argparse;
// use std::vec;

use argparse::{ArgumentParser, Collect, Store};

fn main() {
    let mut cli_path = "/Applications/Dash.app".to_string();
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

    println!("{}", cli_path);
    println!("{:#?}", queries);
}
