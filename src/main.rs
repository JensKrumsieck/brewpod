use std::{collections::HashMap, sync::Arc};

use axum::{Router, extract::Query, response::Redirect, routing::get};
use brewpod::oauth::OAuthClient;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let client_id = dotenvy::var("CLIENT_ID")?;
    let client_secret = dotenvy::var("CLIENT_SECRET")?;
    let client = Arc::new(OAuthClient::new(
        "https://github.com//login/oauth",
        client_id,
        client_secret,
    ));

    let listener = TcpListener::bind("localhost:9292").await?;
    info!("listening on http://{}", listener.local_addr()?);

    let app = Router::new()
        .route("/", get(|| async { "Oppa" }))
        .route(
            "/login-github",
            get({
                let client = Arc::clone(&client);
                || async move { Redirect::temporary(&client.authorize_url()) }
            }),
        )
        .route(
            "/login",
            get({
                let client = Arc::clone(&client);
                |Query(params): Query<HashMap<String, String>>| async move {
                    let code = params.get("code").cloned().unwrap_or_default();
                    let res = client.request_token(&code).await.unwrap();
                    format!("{}, {}", res.access_token, res.refresh_token)
                }
            }),
        );

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
