mod arg_parser;
mod cli_runner;
mod constants;

extern crate argparse;
extern crate futures;
use cli_runner::TelescopeItem;
use futures::future::join_all;

#[tokio::main]
pub async fn main() {
    let args = arg_parser::parse_args();
    let mut results: Vec<TelescopeItem> = Vec::new();
    let mut futures = Vec::new();
    for query in &args.queries {
        futures.push(cli_runner::run_query(&args.cli_path, &query));
    }

    let all_futures = join_all(futures);
    let futures_results = all_futures.await;
    futures_results.iter().for_each(|result| {
        if result.len() > 0 {
            results.append(&mut result.to_owned());
        }
    });

    let json = match args.pretty_print {
        true => serde_json::to_string_pretty(&results).unwrap(),
        false => serde_json::to_string(&results).unwrap(),
    };

    println!("{}", json);
}
