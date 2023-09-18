use askama::Template;
use axum::{extract::Path, headers::UserAgent, http::StatusCode, TypedHeader};
use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Template)]
#[template(path = "subreddit.html")]
pub struct SubredditTemplate {
    subreddit: String,
    data: SubredditResults,
}

pub async fn subreddit(
    Path(subreddit): Path<String>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> Result<SubredditTemplate, StatusCode> {
    let client = reqwest::ClientBuilder::new()
        .user_agent(user_agent.as_str())
        .build()
        .unwrap();

    let response = match client
        .get(format!(
            "{}/r/{}.json",
            utils::get_reddit_domain(),
            &subreddit
        ))
        .send()
        .await
    {
        Ok(r) => {
            if r.status() == StatusCode::OK {
                r
            } else {
                return Err(r.status());
            }
        }
        Err(e) => {
            tracing::error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let res = match response.json::<SubredditResponse>().await {
        Ok(j) => j,
        Err(e) => {
            tracing::error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(SubredditTemplate {
        subreddit,
        data: res.data,
    })
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SubredditResponse {
    kind: String,
    data: SubredditResults,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SubredditResults {
    after: String,
    dist: i32,
    children: Vec<SubredditPost>,
    before: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SubredditPost {
    kind: String,
    data: SubredditPostData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SubredditPostData {
    subreddit: String,
    id: String,
    title: String,
    score: i32,
    thumbnail: Option<String>,
    //edited: bool,
    upvote_ratio: f32,
    archived: bool,
    pinned: bool,
    over_18: bool,
    spoiler: bool,
    author_flair_text: Option<String>,
    author: String,
    num_comments: u32,
    //created_utc: u64,
    selftext_html: Option<String>,
}
