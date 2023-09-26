use askama::Template;
use axum::{
    extract::{Path, Query, State},
    headers::UserAgent,
    http::StatusCode,
    TypedHeader,
};
use reqwest::Client;
use serde::Deserialize;

use crate::{
    api::CommentsQuery,
    api_result_types::{RedditData, ReplyList},
    api_types::CommentSortingMode,
};

#[derive(Template)]
#[template(path = "comments.html")]
pub struct CommentsTemplate {
    subreddit: String,
    data: CommentsQuery,
    gallery_index: usize
}

pub async fn comments(
    Path((subreddit, id)): Path<(String, String)>,
    Query(params): Query<CommentsParams>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    State(client): State<Client>,
) -> Result<CommentsTemplate, StatusCode> {

    let data = crate::api::comments(&client, &subreddit, &id, params.sorting, user_agent.as_str()).await?;
    dbg!(data.get_post_type());
    Ok(CommentsTemplate { subreddit, data, gallery_index: params.gallery_index.unwrap_or_default() })
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommentsParams {
    gallery_index: Option<usize>,
    sorting: Option<CommentSortingMode>
}
