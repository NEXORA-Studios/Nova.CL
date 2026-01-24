use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<serde_json::Value>,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpError {
    pub message: String,
    pub code: Option<u16>,
}

impl From<reqwest::Error> for HttpError {
    fn from(error: reqwest::Error) -> Self {
        HttpError {
            message: error.to_string(),
            code: error.status().map(|s| s.as_u16()),
        }
    }
}
