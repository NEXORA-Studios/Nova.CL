// src/app/state.rs
use std::marker::PhantomData;

pub struct Init;
pub struct Running;
pub struct Shutdown;

use crate::core::http::HttpService;

pub struct AppState<S> {
    pub http: HttpService,
    _state: PhantomData<S>,
}

impl AppState<Init> {
    pub async fn init() -> anyhow::Result<Self> {
        let http = HttpService::new()?;

        Ok(Self {
            http,
            _state: PhantomData,
        })
    }

    pub async fn start(self) -> anyhow::Result<AppState<Running>> {
        Ok(AppState {
            http: self.http.start().await?,
            _state: PhantomData,
        })
    }
}
