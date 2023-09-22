use askama::Template;
use axum::{
    extract::{Path, Query, State},
    headers::UserAgent,
    http::StatusCode,
    TypedHeader,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{api::SubredditQuery, api_result_types::T3Data, api_types::SortingMode, utils};

#[derive(Template)]
#[template(path = "subreddit.html")]
pub struct SubredditTemplate {
    subreddit: String,
    data: SubredditQuery,
}

pub async fn subreddit(
    Path(subreddit): Path<String>,
    sorting: Option<Query<SortingMode>>,
    after: Option<Query<String>>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    State(client): State<Client>,
) -> Result<SubredditTemplate, StatusCode> {
    let sort = sorting.map(|v| v.0);

    let after = after.map(|v| v.0);

    let data = crate::api::subreddit(
        &client,
        &subreddit,
        sort,
        after.as_deref(),
        user_agent.as_str(),
    )
    .await?;

    Ok(SubredditTemplate { subreddit, data })
}
