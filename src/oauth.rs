use axum::http::{HeaderMap, HeaderValue};
use reqwest::{Client, header::ACCEPT};
use serde::{Deserialize, Serialize};

pub struct OAuthClient {
    client_id: String,
    client_secret: String,
}

impl OAuthClient {
    pub fn new(client_id: String, client_secret: String) -> Self {
        OAuthClient {
            client_id,
            client_secret,
        }
    }

    pub fn url(&self, domain: &str) -> String {
        format!(
            "https://{domain}/login/oauth/authorize?client_id={}",
            self.client_id
        )
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
            .post("https://github.com/login/oauth/access_token")
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
