use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("Can't Write The Config")]
    ConfigWriteError,
    #[error("Can't Read The Config")]
    ConfigReadError,
    #[error("Invalid Syntax Can't Parse The Config")]
    ConfigParseError,
    #[error("Cant Retrieve Process Data")]
    ProcessError,
}
