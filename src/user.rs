use askama::Template;
use axum::{
    extract::{Path, Query, State},
    headers::UserAgent,
    http::{StatusCode, Uri},
    TypedHeader,
};
use reqwest::Client;
use serde::Deserialize;
use crate::api_result_types::RedditData;

use crate::{api_types::{UserSortingMode, UserFilterMode, SearchTimeOrdering}, api_result_types::ListingData};

#[derive(Template)]
#[template(path = "user.html")]
pub struct UserTemplate {
    username: String,
    data: ListingData,
    uri: Uri
}

pub async fn user(
    Path(username): Path<String>,
    Query(params): Query<UserParams>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    State(client): State<Client>,
    uri: Uri
) -> Result<UserTemplate, StatusCode> {
    let data = crate::api::user(
        &client,
        &username,
        params.sort,
        params.t,
        params.filter,
        params.after.as_deref(),
        user_agent.as_str(),
    )
    .await?;

    Ok(UserTemplate { username, data, uri })
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserParams {
    sort: Option<UserSortingMode>,
    filter: Option<UserFilterMode>,
    t: Option<SearchTimeOrdering>,
    after: Option<String>,
}