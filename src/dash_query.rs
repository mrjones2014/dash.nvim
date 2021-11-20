use crate::{
    dash_app_connector::{self, DashConnectorError},
    dash_item::{DashItem, DashItemCreationError},
    search_engine::SearchEngine,
};
use crossbeam::channel;
use futures::future;
use std::fmt::Display;
use tokio::runtime::Runtime;

/// Describes the set of errors that can occur
/// when running Dash queries.
///
/// `QueryError::DashConnectorError` indicates that something
/// went wrong when interacting with Dash.app.
///
/// `QueryError::ItemCreation` indicates that something
/// went wrong when creating `DashItem`s from the XML
/// string returned by the Dash.app CLI.
#[derive(Debug)]
pub enum QueryError {
    DashConnectorError(DashConnectorError),
    ItemCreation(DashItemCreationError),
}

impl Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::DashConnectorError(e) => {
                write!(f, "Error from dash_app_connector: {}", e)
            }
            QueryError::ItemCreation(e) => {
                write!(f, "Error creating DashItem struct from XML string: {}", e)
            }
        }
    }
}

impl std::error::Error for QueryError {}

impl From<DashConnectorError> for QueryError {
    fn from(e: DashConnectorError) -> Self {
        QueryError::DashConnectorError(e)
    }
}

impl From<DashItemCreationError> for QueryError {
    fn from(e: DashItemCreationError) -> Self {
        QueryError::ItemCreation(e)
    }
}

/// async wrapper around query function so we can run multiple in parallel
async fn run_query_async(cli_path: &str, query: &str) -> Result<Vec<DashItem>, QueryError> {
    self::run_query_sync(cli_path, query)
}

/// Returns a tuple of (results, errors)
async fn run_queries_async(
    cli_path: &str,
    queries: &[String],
    search_engine_fallback: &SearchEngine,
) -> (Vec<DashItem>, Vec<String>) {
    let mut results: Vec<DashItem> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    let futures: Vec<_> = queries
        .iter()
        .map(|query| run_query_async(cli_path, query))
        .collect();
    future::join_all(futures)
        .await
        .iter()
        .for_each(|future_result| match future_result {
            Ok(items) => items.iter().for_each(|item| results.push(item.to_owned())),
            Err(e) => errors.push(format!("{}", e)),
        });

    if results.is_empty() {
        let query = if !queries.is_empty() { &queries[0] } else { "" };
        results.push(search_engine_fallback.to_dash_item(query));
    }

    (results, errors)
}

/// Run a single query, with search engine fallback. This
/// method does not handle search engine fallback,
/// because it is not intended to be used directly. It is
/// called internally by `dash_query_binding::open_item`
/// and is expected to always be given a query which is
/// known to return at least one result.
///
/// # Arguments
///
/// - `cli_path` - the path to Dash.app's CLI to run the queries with
/// - `query` - the query to run
pub fn run_query_sync(cli_path: &str, query: &str) -> Result<Vec<DashItem>, QueryError> {
    let xml_result = dash_app_connector::get_xml(cli_path, query)?;
    Ok(DashItem::try_from_xml(xml_result, query)?)
}

/// Run a list of queries in parallel, with a search engine fallback
///
/// # Arguments
///
/// - `cli_path` - the path to Dash.app's CLI to run the queries with
/// - `queries` - the list of queries to run
/// - `search_engine_fallback` - the search engine that should be used when no results are found
pub fn run_queries_parallel(
    cli_path: String,
    queries: Vec<String>,
    search_engine_fallback: SearchEngine,
) -> (Vec<DashItem>, Vec<String>) {
    // if empty, just return empty results
    if queries.is_empty() {
        return (Vec::new(), Vec::new());
    }

    // if only 1 query, don't bother with the overhead of parallelization
    if queries.len() == 1 {
        let result = run_query_sync(&cli_path, &queries[0]);
        if result.is_ok() {
            let result_items = result.as_ref().unwrap();
            if result_items.is_empty() {
                let query = if !queries.is_empty() { &queries[0] } else { "" };
                return (vec![search_engine_fallback.to_dash_item(query)], Vec::new());
            }

            return (result_items.to_owned(), Vec::new());
        }

        return (Vec::new(), vec![format!("{}", result.unwrap_err())]);
    }

    let (tx, rx) = channel::bounded(1);
    let runtime = Runtime::new().unwrap();
    let handle = runtime.handle();
    handle.spawn(async move {
        let result_table = &run_queries_async(&cli_path, &queries, &search_engine_fallback).await;
        let _ = tx.send(result_table.clone());
    });

    rx.recv().unwrap()
}
