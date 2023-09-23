use askama::Template;
use axum::{
    extract::{Path, Query, State},
    headers::UserAgent,
    http::StatusCode,
    TypedHeader,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    api::SubredditQuery,
    api_result_types::{T3Data, WikiPageData},
    api_types::{SortingMode, TopSortingTime},
    utils,
};

#[derive(Template)]
#[template(path = "wiki.html")]
pub struct WikiTemplate {
    subreddit: String,
    data: WikiPageData,
}

pub async fn wiki_page(
    Path(subreddit): Path<String>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    State(client): State<Client>,
) -> Result<WikiTemplate, StatusCode> {
    let data = crate::api::wiki(
        &client,
        &subreddit,
        None,
        user_agent.as_str(),
    )
    .await?;

    Ok(WikiTemplate { subreddit, data })
}