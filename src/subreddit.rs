use askama::Template;
use axum::{
    extract::{Path, Query, State},
    headers::UserAgent,
    http::{StatusCode, Uri},
    TypedHeader,
};
use reqwest::Client;
use serde::Deserialize;

use crate::{
    api::SubredditQuery,
    api_types::{SortingMode, TopSortingTime},
};

#[derive(Template)]
#[template(path = "subreddit.html")]
pub struct SubredditTemplate {
    subreddit: String,
    data: SubredditQuery,
    uri: Uri
}

pub async fn subreddit(
    Path(subreddit): Path<String>,
    Query(params): Query<SubredditParams>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    State(client): State<Client>,
    uri: Uri
) -> Result<SubredditTemplate, StatusCode> {
    let data = crate::api::subreddit(
        &client,
        &subreddit,
        params.sort,
        params.t,
        params.after.as_deref(),
        user_agent.as_str(),
    )
    .await?;

    Ok(SubredditTemplate { subreddit, data, uri })
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubredditParams {
    sort: Option<SortingMode>,
    t: Option<TopSortingTime>,
    after: Option<String>,
}
