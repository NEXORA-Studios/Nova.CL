use crate::http::types::{HttpError, HttpRequest, HttpResponse};
use reqwest::{Client as ReqwestClient, Response};
use std::collections::HashMap;

pub struct HttpClient {
    client: ReqwestClient,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = ReqwestClient::new();
        HttpClient { client }
    }

    async fn process_response(response: Response) -> Result<HttpResponse, HttpError> {
        let status = response.status().as_u16();

        let mut headers = HashMap::new();
        for (key, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(key.to_string(), value_str.to_string());
            }
        }

        let bytes = response.bytes().await?;
        let body = match serde_json::from_slice(&bytes) {
            Ok(json) => Some(json),
            Err(_) => {
                let text = String::from_utf8_lossy(&bytes).to_string();
                Some(serde_json::Value::String(text))
            }
        };

        Ok(HttpResponse {
            status,
            headers,
            body,
            text: None,
        })
    }

    pub async fn request(&self, req: &HttpRequest) -> Result<HttpResponse, HttpError> {
        let mut request_builder = match req.method.to_uppercase().as_str() {
            "GET" => self.client.get(&req.url),
            "POST" => self.client.post(&req.url),
            "PUT" => self.client.put(&req.url),
            "DELETE" => self.client.delete(&req.url),
            "PATCH" => self.client.patch(&req.url),
            "HEAD" => self.client.head(&req.url),
            "OPTIONS" => self.client.request(reqwest::Method::OPTIONS, &req.url),
            _ => {
                return Err(HttpError {
                    message: format!("Unsupported HTTP method: {}", req.method),
                    code: None,
                })
            }
        };

        if let Some(headers) = &req.headers {
            for (key, value) in headers {
                request_builder = request_builder.header(key, value);
            }
        }

        if let Some(body) = &req.body {
            request_builder = request_builder.json(body);
        }

        let response = request_builder.send().await?;
        Self::process_response(response).await
    }

    pub async fn get(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
    ) -> Result<HttpResponse, HttpError> {
        self.request(&HttpRequest {
            method: "GET".to_string(),
            url: url.to_string(),
            headers,
            body: None,
        })
        .await
    }

    pub async fn post(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<serde_json::Value>,
    ) -> Result<HttpResponse, HttpError> {
        self.request(&HttpRequest {
            method: "POST".to_string(),
            url: url.to_string(),
            headers,
            body,
        })
        .await
    }

    pub async fn put(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<serde_json::Value>,
    ) -> Result<HttpResponse, HttpError> {
        self.request(&HttpRequest {
            method: "PUT".to_string(),
            url: url.to_string(),
            headers,
            body,
        })
        .await
    }

    pub async fn delete(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
    ) -> Result<HttpResponse, HttpError> {
        self.request(&HttpRequest {
            method: "DELETE".to_string(),
            url: url.to_string(),
            headers,
            body: None,
        })
        .await
    }

    pub async fn patch(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<serde_json::Value>,
    ) -> Result<HttpResponse, HttpError> {
        self.request(&HttpRequest {
            method: "PATCH".to_string(),
            url: url.to_string(),
            headers,
            body,
        })
        .await
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
