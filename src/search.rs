use askama::Template;
use axum::{
    extract::{Path, Query, State},
    headers::UserAgent,
    TypedHeader, http::Uri,
};
use reqwest::{Client, StatusCode};
use serde::Deserialize;

use crate::{
    api::SubredditQuery,
    api_types::{SearchSortingMode, SearchTimeOrdering},
};

#[derive(Template)]
#[template(path = "search.html")]
pub struct SearchTemplate {
    subreddit: String,
    data: SubredditQuery,
    uri: Uri
}

//t=day
//sort=comments
//sort=new
//sort=relevance
pub async fn search_handler(
    Path(subreddit): Path<String>,
    Query(params): Query<SearchParams>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    State(client): State<Client>,
    uri: Uri
) -> Result<SearchTemplate, StatusCode> {
    let data = crate::api::search(
        &client,
        &subreddit,
        &params.q,
        params.sort,
        params.t,
        params.after.as_deref(),
        params.include_over_18.unwrap_or_default(),
        params.only_current_subreddit.unwrap_or_default(),
        user_agent.as_str(),
    )
    .await?;

    Ok(SearchTemplate { subreddit, data, uri })
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchParams {
    q: String,
    sort: Option<SearchSortingMode>,
    t: Option<SearchTimeOrdering>,
    include_over_18: Option<bool>,
    only_current_subreddit: Option<bool>,
    after: Option<String>,
}
