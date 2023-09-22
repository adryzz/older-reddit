use axum::{
    extract::{Path, State},
    headers::UserAgent,
    http::{Response, StatusCode},
    TypedHeader,
};
use bytes::Bytes;
use reqwest::{header::USER_AGENT, Client};

#[axum::debug_handler]
pub async fn reddit_image_proxy(
    Path(file): Path<String>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    State(client): State<Client>,
) -> Result<Response<axum::body::Full<Bytes>>, StatusCode> {
    let url = format!("https://i.redd.it/{}", file);
    match client
        .get(&url)
        .header(USER_AGENT, user_agent.as_str())
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                let image_bytes = response.bytes().await.unwrap();
                Ok(Response::new(axum::body::Full::new(image_bytes)))
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
