mod api;
mod api_result_types;
mod api_types;
mod comments;
mod image_proxy;
mod search;
mod subreddit;
mod utils;
mod wiki;
mod user;

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
        .route("/r/:subreddit/comments/:file", get(comments::comments))
        .route("/r/:subreddit/search", get(search::search_handler))
        .route("/r/:subreddit/wiki", get(wiki::wiki_page))
        .route("/u/:username", get(user::user))
        .route("/i/:id", get(image_proxy::reddit_image_proxy))
        .with_state(reqwest::Client::new());

    let listener = std::net::TcpListener::bind("0.0.0.0:3000")?;
    tracing::info!("Listening on {}...", listener.local_addr()?);

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
