use axum::{extract::Path, headers::UserAgent, http::{Response, StatusCode}, TypedHeader};
use bytes::Bytes;

#[axum::debug_handler]
pub async fn reddit_image_proxy(
    Path(file): Path<String>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> Result<Response<axum::body::Full<Bytes>>, StatusCode> {
    let client = reqwest::ClientBuilder::new()
        .user_agent(user_agent.as_str())
        .build()
        .unwrap();

    let url = format!("https://i.redd.it/{}", file);
    match client.get(&url).send().await {
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