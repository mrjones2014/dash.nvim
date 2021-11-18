pub mod dash_app_connector {
    use std::{process::Command, string::FromUtf8Error};

    pub enum DashConnectorError {
        IoError(std::io::Error),
        CharsetError(FromUtf8Error),
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

    pub async fn get_xml(cli_path: &str, query: &str) -> Result<String, DashConnectorError> {
        let raw_output = Command::new(cli_path).args(&[query]).output()?;
        let str_output = String::from_utf8(raw_output.stdout)?;
        Ok(str_output)
    }
}
