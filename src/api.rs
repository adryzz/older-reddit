use reqwest::{Client, StatusCode};

use crate::{
    api_result_types::{ApiData, RedditData, T1Data, T3Data},
    api_types::CommentSortingMode,
};

use crate::utils;

pub async fn comments(
    client: &mut Client,
    subreddit: &str,
    post_id: &str,
    sorting: Option<CommentSortingMode>,
) -> Result<(T3Data, Vec<T1Data>), StatusCode> {
    let sort = sorting.unwrap_or_default();

    let args = match sort {
        CommentSortingMode::Suggested => "",
        CommentSortingMode::Best => "?sort=confidence",
        CommentSortingMode::Controversial => "?sort=controversial",
        CommentSortingMode::Old => "?sort=old",
        CommentSortingMode::New => "?sort=new",
        CommentSortingMode::QAndA => "?sort=qa",
        CommentSortingMode::Top => "?sort=top",
    };

    let response = match client
        .get(format!(
            "{}/r/{}/comments/{}.json{}",
            utils::get_reddit_domain(),
            &subreddit,
            &post_id,
            args
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

    let res = match response.json::<ApiData>().await {
        Ok(j) => j,
        Err(e) => {
            tracing::error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let data = if let ApiData::Collection(d) = res {
        d
    } else {
        tracing::error!("Wrong schema 1");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    if data.len() != 2 {
        tracing::error!("Wrong schema 2");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let first_listing = if let RedditData::Listing(l) = &data[0] {
        l
    } else {
        tracing::error!("Wrong schema 3");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    if first_listing.children.len() != 1 {
        tracing::error!("Wrong schema 4");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let post = if let RedditData::T3(t3) = &first_listing.children[0] {
        t3.clone()
    } else {
        tracing::error!("Wrong schema 5");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let second_listing = if let RedditData::Listing(list) = &data[1] {
        list
    } else {
        tracing::error!("Wrong schema 6");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let all_t1 = second_listing
        .children
        .iter()
        .all(|child| matches!(child, RedditData::T1(_)));

    if !all_t1 {
        tracing::error!("Wrong schema 7");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let comments: Vec<T1Data> = second_listing
        .children
        .clone()
        .into_iter()
        .filter_map(|child| match child {
            RedditData::T1(t1_data) => Some(t1_data),
            _ => None,
        })
        .collect();

    return Ok((post, comments));
}
