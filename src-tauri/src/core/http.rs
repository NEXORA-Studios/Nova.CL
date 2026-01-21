// src/core/http.rs
pub struct HttpService {
    client: reqwest::Client,
}

impl HttpService {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            client: reqwest::Client::new(),
        })
    }

    pub async fn start(self) -> anyhow::Result<Self> {
        Ok(self)
    }

    pub async fn get(&self, url: &str) -> anyhow::Result<String> {
        Ok(self.client.get(url).send().await?.text().await?)
    }
}
