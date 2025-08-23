use axum::{
    Json, Router,
    extract::Query,
    response::Redirect,
    routing::{get, post},
};
use brewpod::{oauth::OAuthClient, webhook::WebhookEvent};
use std::io::Write;
use std::{collections::HashMap, fs::OpenOptions, sync::Arc};
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
                    let res = client.request_access_token(&code).await.unwrap();
                    format!("{}, {}", res.access_token, res.refresh_token)
                }
            }),
        )
        .route(
            "/webhook",
            post(|Json(payload): Json<WebhookEvent>| async {
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("data.log")
                    .unwrap();
                match payload {
                    WebhookEvent::Push(webhook_push) => write!(
                        file,
                        "{} pushed to {}",
                        webhook_push.pusher.name, webhook_push.repository.full_name
                    )
                    .unwrap(),
                }

                "success"
            }),
        );

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
