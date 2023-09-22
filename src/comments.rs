use askama::Template;
use axum::{
    extract::{Path, Query, State},
    headers::UserAgent,
    http::StatusCode,
    TypedHeader,
};
use reqwest::Client;

use crate::{
    api::CommentsQuery,
    api_result_types::{T1Data, T3Data, ReplyList, RedditData},
    api_types::CommentSortingMode,
};

#[derive(Template)]
#[template(path = "comments.html")]
pub struct CommentsTemplate {
    subreddit: String,
    data: CommentsQuery,
}

pub async fn comments(
    Path((subreddit, id)): Path<(String, String)>,
    sorting: Option<Query<CommentSortingMode>>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    State(client): State<Client>,
) -> Result<CommentsTemplate, StatusCode> {
    let sort = sorting.map(|v| v.0);

    let data = crate::api::comments(&client, &subreddit, &id, sort, user_agent.as_str()).await?;
    dbg!(data.get_post_type());
    Ok(CommentsTemplate { subreddit, data })
}
