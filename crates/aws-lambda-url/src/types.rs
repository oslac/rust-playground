use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::fmt::Display;

pub type Response = Result<Success, Failure>;

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Success {
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Failure {
    pub body: String,
}

impl Error for Failure {}

impl Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}
