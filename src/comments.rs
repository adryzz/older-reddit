use askama::Template;
use axum::{
    extract::{Path, Query},
    headers::UserAgent,
    http::StatusCode,
    TypedHeader,
};

use crate::{
    api::CommentsQuery,
    api_result_types::{T1Data, T3Data},
    api_types::CommentSortingMode,
};

#[derive(Template)]
#[template(path = "comments.html")]
pub struct SubredditTemplate {
    subreddit: String,
    data: CommentsQuery,
}

pub async fn comments(
    Path((subreddit, id)): Path<(String, String)>,
    sorting: Option<Query<CommentSortingMode>>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> Result<SubredditTemplate, StatusCode> {
    let mut client = reqwest::ClientBuilder::new()
        .user_agent(user_agent.as_str())
        .build()
        .unwrap();

    let sort = sorting.map(|v| v.0);

    let data = crate::api::comments(&mut client, &subreddit, &id, sort).await?;

    Ok(SubredditTemplate { subreddit, data })
}
