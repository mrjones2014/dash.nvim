use std::{fmt::Display, process::Command, string::FromUtf8Error};

/// Describes errors that can occur when interacting with Dash.app.
/// `DashConnectorError::IoError` can occur if the app path is configured
/// incorrectly, etc. `DashConnectorError::CharsetError` can happen
/// if for some reason the Dash.app CLI returns a string that is not
/// UTF-8 encoded.
#[derive(Debug)]
pub enum DashConnectorError {
    IoError(std::io::Error),
    CharsetError(FromUtf8Error),
}

impl Display for DashConnectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DashConnectorError::IoError(e) => {
                write!(f, "I/O error running Dash.app CLI: {}", e)
            }
            DashConnectorError::CharsetError(e) => write!(
                f,
                "XML string returned from Dash.app CLI has incorrect encoding: {}",
                e
            ),
        }
    }
}

impl From<std::io::Error> for DashConnectorError {
    fn from(e: std::io::Error) -> Self {
        DashConnectorError::IoError(e)
    }
}

impl From<FromUtf8Error> for DashConnectorError {
    fn from(e: FromUtf8Error) -> Self {
        DashConnectorError::CharsetError(e)
    }
}

/// Executes the Dash.app CLI and returns the XML-formatted output
///
/// # Arguments
///
/// - `cli_path` - the path to Dash.app's CLI
/// - `query` - the query to run
pub fn get_xml(cli_path: &str, query: &str) -> Result<String, DashConnectorError> {
    let raw_output = Command::new(cli_path).args(&[query]).output()?;
    let str_output = String::from_utf8(raw_output.stdout)?;
    Ok(str_output)
}
