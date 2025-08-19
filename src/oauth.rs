use axum::http::{HeaderMap, HeaderValue};
use reqwest::{Client, header::ACCEPT};
use serde::{Deserialize, Serialize};

pub struct OAuthClient {
    provider: String,
    client_id: String,
    client_secret: String,
}

impl OAuthClient {
    pub fn new<S: ToString>(provider: S, client_id: String, client_secret: String) -> Self {
        OAuthClient {
            provider: provider.to_string(),
            client_id,
            client_secret,
        }
    }

    pub fn authorize_url(&self) -> String {
        format!("{}/authorize?client_id={}", self.provider, self.client_id)
    }

    /// Tries to exchange the token from login request to a pair of access and refresh token
    pub async fn request_token(&self, code: &str) -> anyhow::Result<OAuthTokenResponse> {
        let mut headers = HeaderMap::new();
        headers.append(ACCEPT, HeaderValue::from_static("application/json"));

        let client = Client::builder().default_headers(headers).build()?;
        let body: [(&'static str, &str); 3] = [
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("code", code),
        ];

        let res: OAuthTokenResponse = client
            .post(format!("{}/access_token", self.provider))
            .form(&body)
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }
}

#[derive(Serialize, Deserialize)]
pub struct OAuthTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}
