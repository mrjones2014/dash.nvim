mod cli_runner;
mod constants;

extern crate argparse;
extern crate futures;
use argparse::{ArgumentParser, Collect, Store};
use cli_runner::TelescopeItem;
use futures::future::join_all;

#[tokio::main]
pub async fn main() {
    let mut cli_path = constants::DASH_APP_BASE_PATH.to_owned();
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

    cli_path.push_str(constants::DASH_APP_CLI_PATH);

    let mut results: Vec<TelescopeItem> = Vec::new();
    let mut futures = Vec::new();
    for query in &queries {
        futures.push(cli_runner::run_query(&cli_path, &query));
    }

    let all_futures = join_all(futures);
    let futures_results = all_futures.await;
    futures_results.iter().for_each(|result| {
        if result.len() > 0 {
            results.append(&mut result.to_owned());
        }
    });

    print!("{}", serde_json::to_string(&results).unwrap());
}
