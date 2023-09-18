mod api;
mod api_result_types;
mod api_types;
mod comments;
mod subreddit;
mod utils;

use axum::{response::Redirect, routing::get, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting server...");

    match run().await {
        Ok(_) => tracing::info!("Program exited successfully."),
        Err(e) => tracing::error!("Error: {}", e),
    }
}

async fn run() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/r/all") }))
        .route("/r/:subreddit", get(subreddit::subreddit))
        .route("/r/:subreddit/comments/:id", get(comments::comments));

    let listener = std::net::TcpListener::bind("0.0.0.0:3000")?;
    tracing::info!("Listening on {}...", listener.local_addr()?);

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
