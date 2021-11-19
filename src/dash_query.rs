pub mod dash_query {
    use std::fmt::Display;

    use crossbeam::channel;
    use futures::future;
    use tokio::runtime::Runtime;

    use crate::{
        dash_app_connector::dash_app_connector::{self, DashConnectorError},
        dash_item::{DashItem, DashItemCreationError},
        search_engine::SearchEngine,
    };

    #[derive(Debug)]
    enum QueryError {
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

    async fn run_query_async(cli_path: &str, query: &str) -> Result<Vec<DashItem>, QueryError> {
        let xml_result = dash_app_connector::get_xml(cli_path, &query)?;
        Ok(DashItem::try_from_xml(xml_result, &query)?)
    }

    async fn run_queries_async(
        cli_path: &str,
        queries: &Vec<String>,
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

        if results.len() == 0 {
            let query = if queries.len() > 0 { &queries[0] } else { "" };
            results.push(search_engine_fallback.to_dash_item(&query));
        }

        (results, errors)
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
        let (tx, rx) = channel::bounded(1);
        let runtime = Runtime::new().unwrap();
        let handle = runtime.handle();
        handle.spawn(async move {
            let result_table =
                &run_queries_async(&cli_path, &queries, &search_engine_fallback).await;
            let _ = tx.send(result_table.clone());
        });

        return rx.recv().unwrap().to_owned();
    }
}
