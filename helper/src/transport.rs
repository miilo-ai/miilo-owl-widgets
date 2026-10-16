use crate::{
    Result,
    config::{Endpoint, safe_url},
};
use reqwest::{blocking::Client, redirect::Policy};
use serde_json::Value;
use std::{io::Read, time::Duration};

pub trait Transport: Sync {
    fn request(&self, endpoint: &Endpoint, path: &str, body: Option<&Value>) -> Result<Value>;
}
pub struct Http {
    client: Client,
}
impl Http {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .redirect(Policy::none())
            .retry(reqwest::retry::never())
            .timeout(Duration::from_secs(8))
            .connect_timeout(Duration::from_secs(8))
            .build()
            .map_err(|_| "Cannot initialize HTTP")?;
        Ok(Self { client })
    }
}
impl Transport for Http {
    fn request(&self, endpoint: &Endpoint, path: &str, body: Option<&Value>) -> Result<Value> {
        let url = format!("{}{path}", safe_url(&endpoint.url)?);
        let mut request = match body {
            Some(b) => self.client.post(url).json(b),
            None => self.client.get(url),
        }
        .header("Accept", "application/json");
        let credential = endpoint.credential()?;
        if !credential.is_empty() {
            request = request.bearer_auth(&credential);
        }
        let response = request
            .send()
            .map_err(|_| "Endpoint unavailable; check network access")?;
        if !response.status().is_success() {
            return Err(format!(
                "HTTP {}; refresh or check access",
                response.status().as_u16()
            ));
        }
        let mut raw = Vec::new();
        response
            .take(2_000_001)
            .read_to_end(&mut raw)
            .map_err(|_| "Cannot read endpoint response")?;
        if raw.len() > 2_000_000 {
            return Err("Provider response exceeds the widget limit".into());
        }
        let value: Value =
            serde_json::from_slice(&raw).map_err(|_| "Endpoint returned invalid JSON")?;
        if !value.is_object() {
            return Err("Expected a JSON object".into());
        }
        Ok(value)
    }
}
