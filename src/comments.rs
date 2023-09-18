use askama::Template;
use axum::{extract::Path, headers::UserAgent, http::StatusCode, TypedHeader};
use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Template)]
#[template(path = "comments.html")]
pub struct SubredditTemplate {
    subreddit: String,
    data: PostResults,
}

pub async fn comments(
    Path((subreddit, id)): Path<(String, String)>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> Result<SubredditTemplate, StatusCode> {
    let client = reqwest::ClientBuilder::new()
        .user_agent(user_agent.as_str())
        .build()
        .unwrap();

    let response = match client
        .get(format!(
            "{}/r/{}/comments/{}.json",
            utils::get_reddit_domain(),
            &subreddit,
            &id
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

    let res = match response.json::<Vec<PostResponse>>().await {
        Ok(j) => j,
        Err(e) => {
            tracing::error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(SubredditTemplate {
        subreddit,
        data: res.try_into()?,
    })
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct PostResponse {
    data: PostData
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct PostData {
    children: Vec<PostDataChildren>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct PostDataChildren {
    data: PostDataChildrenInner
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct PostDataChildrenInner {
    selftext: Option<String>,
    body: Option<String>,
    title: Option<String>,
    //edited: bool,
    //created_utc: u64,
    score: i32,
    stickied: bool,
    author: String,
    locked: bool,
    id: String,
    spoiler: Option<bool>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct PostResults {
    post: RedditPost,
    comments: Vec<RedditComments>
}

impl TryFrom<Vec<PostResponse>> for PostResults {
    type Error = StatusCode;

    fn try_from(value: Vec<PostResponse>) -> Result<Self, StatusCode> {
        let stuff = &value[0].data.children[0].data;

        let post = RedditPost {
            id: stuff.id.clone(),
            selftext: stuff.selftext.clone().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
            title: stuff.title.clone().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
            score: stuff.score,
            author: stuff.author.clone(),
            //edited: stuff.edited,
            locked: stuff.locked,
            stickied: stuff.stickied,
            spoiler: stuff.spoiler.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        };

        let children = &value[1].data.children;
        let mut comments: Vec<RedditComments> = Vec::new();

        for c in children {
            let comment = RedditComments {
                id: c.data.id.clone(),
                body: c.data.body.clone().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
                score: c.data.score,
                author: c.data.author.clone(),
                //edited: c.data.edited,
                locked: c.data.locked,
                stickied: c.data.stickied
            };

            comments.push(comment);
        }

        Ok(Self {
            post,
            comments
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RedditPost {
    id: String,
    selftext: String,
    title: String,
    score: i32,
    author: String,
    //edited: bool,
    locked: bool,
    stickied: bool,
    spoiler: bool
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RedditComments {
    id: String,
    body: String,
    score: i32,
    author: String,
    //edited: bool,
    locked: bool,
    stickied: bool
}
