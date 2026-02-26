use crate::{config::Config, endpoint::Endpoint, token::Token};
use reqwest::{StatusCode, blocking::Client};
use serde::Serialize;
use serde_json::Value;

pub struct AuthenticatedClient {
    client: Client,
    config: Config,
    token: Token,
}

impl AuthenticatedClient {
    pub fn new() -> anyhow::Result<Self> {
        let config = Config::new()?;
        let token = config.get_token()?;
        Ok(Self {
            client: Client::new(),
            config,
            token,
        })
    }
    pub fn access_token(&mut self) -> anyhow::Result<&str> {
        if self.token.is_expired() {
            self.token = self.config.get_token()?;
            Ok(&self.token.access_token)
        } else {
            Ok(&self.token.access_token)
        }
    }
    pub fn post(
        &mut self,
        endpoint: Endpoint,
        request_body: impl Serialize,
    ) -> anyhow::Result<Value> {
        let response = self
            .client
            .post(self.config.url.clone().join(&format!("api/{}", endpoint))?)
            .bearer_auth(self.access_token()?)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()?;
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => Ok(response.json()?),
            _ => Err(anyhow::anyhow!(
                "Post failed with status code: {} and response: {}",
                response.status(),
                response.json::<Value>()?
            )),
        }
    }
    pub fn delete(&mut self, endpoint: Endpoint, id: u32) -> anyhow::Result<()> {
        let response = self
            .client
            .delete(
                self.config
                    .url
                    .clone()
                    .join(&format!("api/{}/{}", endpoint, id))?,
            )
            .bearer_auth(self.access_token()?)
            .header("Content-Type", "application/json")
            .send()?;
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => Ok(()),
            _ => Err(anyhow::anyhow!(
                "Delete failed with status code: {} and response: {}",
                response.status(),
                response.json::<Value>()?
            )),
        }
    }
}
