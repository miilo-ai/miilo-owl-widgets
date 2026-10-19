use crate::{
    Result,
    application::ports::Transport,
    config::{Endpoint, safe_url},
};
use reqwest::{blocking::Client, redirect::Policy};
use serde_json::Value;
use std::{io::Read, time::Duration};

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
    fn snapshots(&self, endpoints: &[Endpoint]) -> Vec<Result<Value>> {
        endpoints
            .chunks(4)
            .flat_map(|batch| {
                std::thread::scope(|scope| {
                    batch
                        .iter()
                        .map(|endpoint| {
                            scope.spawn(move || self.request(endpoint, "/v1/snapshot", None))
                        })
                        .collect::<Vec<_>>()
                        .into_iter()
                        .map(|h| {
                            h.join()
                                .unwrap_or_else(|_| Err("Endpoint worker failed".into()))
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect()
    }

    fn request(&self, endpoint: &Endpoint, path: &str, body: Option<&Value>) -> Result<Value> {
        if endpoint.local_snapshot_file.is_some() {
            return crate::local_snapshot::request(endpoint, path, body);
        }
        let paired = if let Some(peer) = &endpoint.paired_host {
            if path != "/v1/snapshot"
                || body.is_some()
                || endpoint.url != peer.url
                || endpoint.allow_actions
            {
                return Err("Paired computers permit read-only observation".into());
            }
            Some(miilo_pairing::adapters::client::pinned(
                &peer.certificate_pem,
                &peer.fingerprint,
            )?)
        } else {
            None
        };
        let url = format!("{}{path}", safe_url(&endpoint.url)?);
        let mut request = match body {
            Some(b) => self.client.post(url).json(b),
            None => paired.as_ref().unwrap_or(&self.client).get(url),
        }
        .header("Accept", "application/json")
        // Read-only generation can take longer than observation or action requests.
        .timeout(Duration::from_secs(if path == "/v1/explanations" {
            20
        } else {
            8
        }));
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
