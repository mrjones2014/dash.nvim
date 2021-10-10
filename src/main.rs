mod cli_runner;

extern crate argparse;
extern crate futures;
use argparse::{ArgumentParser, Collect, Store};
use futures::future::join_all;

#[tokio::main]
pub async fn main() {
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

    println!("[");
    let mut results: Vec<String> = Vec::new();
    let mut futures = Vec::new();
    for query in &queries {
        futures.push(cli_runner::run_query(&cli_path, &query));
    }

    let all_futures = join_all(futures);
    let futures_results = all_futures.await;
    futures_results.iter().for_each(|result| {
        let json = result.to_string();
        if json.len() > 50 {
            // min length of the JSON with all empty values
            results.push(json)
        }
    });

    println!("{}", &results.join(","));
    println!("]");
}
