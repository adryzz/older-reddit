use askama::Template;
use axum::{
    extract::{Path, Query},
    headers::UserAgent,
    http::StatusCode,
    TypedHeader,
};
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
) -> Result<SubredditTemplate, StatusCode> {
    let mut client = reqwest::ClientBuilder::new()
        .user_agent(user_agent.as_str())
        .build()
        .unwrap();

    let sort = sorting.map(|v| v.0);

    let after = after.map(|v| v.0);

    let data = crate::api::subreddit(&mut client, &subreddit, sort, after.as_deref()).await?;

    Ok(SubredditTemplate { subreddit, data })
}
