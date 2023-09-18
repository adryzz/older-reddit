use reqwest::{Client, StatusCode};

use crate::{
    api_result_types::{ApiData, RedditData, T1Data, T3Data},
    api_types::{CommentSortingMode, SortingMode, TopSortingTime},
};

use crate::utils;

pub struct CommentsQuery {
    pub post: T3Data,
    pub comments: Vec<T1Data>,
    pub after: Option<String>,
    pub before: Option<String>,
}

pub async fn comments(
    client: &mut Client,
    subreddit: &str,
    post_id: &str,
    sorting: Option<CommentSortingMode>,
) -> Result<CommentsQuery, StatusCode> {
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

    return Ok(CommentsQuery {
        post,
        comments,
        after: second_listing.after.clone(),
        before: second_listing.before.clone(),
    });
}

pub struct SubredditQuery {
    pub posts: Vec<T3Data>,
    pub after: Option<String>,
    pub before: Option<String>,
}

// ?after=t3_16kksoi
pub async fn subreddit(
    client: &mut Client,
    subreddit: &str,
    sorting: Option<SortingMode>,
    after: Option<&str>,
) -> Result<SubredditQuery, StatusCode> {
    let sort = sorting.unwrap_or_default();

    let end_string = match sort {
        SortingMode::Default => ".json",
        SortingMode::Hot => "/hot.json",
        SortingMode::New => "/new.json",
        SortingMode::Rising => "/rising.json",
        SortingMode::Controversial => "/controversial.json",
        SortingMode::Top(t) => match t {
            TopSortingTime::PastHour => "/top.json?t=hour",
            TopSortingTime::Past24Hours => "/top.json?t=day",
            TopSortingTime::PastWeek => "/top.json?t=week",
            TopSortingTime::PastMonth => "/top.json?t=month",
            TopSortingTime::PastYear => "/top.json?t=year",
            TopSortingTime::AllTime => "/top.json?t=all",
        },
    };

    let url = if let Some(s) = after {
        if let SortingMode::Top(_) = sort {
            format!(
                "{}/r/{}{}&after={}",
                utils::get_reddit_domain(),
                subreddit,
                end_string,
                s
            )
        } else {
            format!(
                "{}/r/{}{}?after={}",
                utils::get_reddit_domain(),
                subreddit,
                end_string,
                s
            )
        }
    } else {
        format!("{}/r/{}{}", utils::get_reddit_domain(), subreddit, end_string)
    };

    let response = match client.get(url).send().await {
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

    let listing = if let ApiData::Single(RedditData::Listing(l)) = res {
        l
    } else {
        tracing::error!("Invalid schema 1");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let all_t3 = listing
        .children
        .iter()
        .all(|child| matches!(child, RedditData::T3(_)));

    if !all_t3 {
        tracing::error!("Wrong schema 2");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let posts: Vec<T3Data> = listing
        .children
        .into_iter()
        .filter_map(|child| match child {
            RedditData::T3(t3_data) => Some(t3_data),
            _ => None,
        })
        .collect();

    Ok(SubredditQuery {
        posts,
        after: listing.after,
        before: listing.before,
    })
}
