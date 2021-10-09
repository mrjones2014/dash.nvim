mod cli_runner;

extern crate argparse;
use argparse::{ArgumentParser, Collect, Store};
use std::thread;

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

    let mut threads = Vec::new();

    println!("[");
    let mut results = [].to_vec();
    for query in &queries {
        threads.push(thread::spawn(|| {
            results.push(cli_runner::run_query(
                &cli_path.to_string(),
                &query.to_string(),
            ));
        }))
    }

    for thread in &threads {
        thread.join();
    }
    println!("{}", results.join(","));
    println!("]");
}
