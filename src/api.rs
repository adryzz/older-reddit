use reqwest::{header::USER_AGENT, Client, StatusCode};
use tracing_subscriber::util;

use crate::{
    api_result_types::{ApiData, RedditData, T1Data, T3Data},
    api_types::{CommentSortingMode, SortingMode, TopSortingTime, SearchSortingMode, SearchTimeOrdering},
};

use crate::utils;

pub struct CommentsQuery {
    pub post: T3Data,
    pub comments: Vec<T1Data>,
    pub after: Option<String>,
    pub before: Option<String>,
}

impl CommentsQuery {
    pub fn get_post_type(&self) -> PostType {
        if self.post.is_video {
            return PostType::Video;
        }

        if self.post.is_gallery.is_some() {
            return PostType::Gallery;
        }

        if self.post.is_reddit_media_domain {
            return PostType::Image;
        }

        if self.post.url.is_some() && self.post.selftext.len() == 0 {
            return PostType::Link;
        }

        return PostType::Text;
    }

    pub fn get_url(&self) -> Option<String> {
        if let Some(u) = self.post.url.clone() {
            if self.post.is_reddit_media_domain {
                return Some(u.replace("https://i.redd.it", "/i"));
            }
            return Some(u)
        }

        return None;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PostType {
    Text,
    Link,
    Gallery,
    Video,
    Image,
    Poll,
}

pub async fn comments(
    client: &Client,
    subreddit: &str,
    post_id: &str,
    sorting: Option<CommentSortingMode>,
    user_agent: &str,
) -> Result<CommentsQuery, StatusCode> {
    let sort = sorting.unwrap_or_default();

    let mut base = utils::get_reddit_domain();
    base.add_route("r");
    base.add_route(subreddit);
    base.add_route("comments");
    base.add_route(&format!("{}.json", post_id));

    match sort {
        CommentSortingMode::Suggested => &mut base,
        CommentSortingMode::Best => base.add_param("sort", "confidence"),
        CommentSortingMode::Controversial => base.add_param("sort", "controversial"),
        CommentSortingMode::Old => base.add_param("sort", "old"),
        CommentSortingMode::New => base.add_param("sort", "new"),
        CommentSortingMode::QAndA => base.add_param("sort", "qa"),
        CommentSortingMode::Top => base.add_param("sort", "top"),
    };

    let url = base.build();

    let response = match client.get(&url).header(USER_AGENT, user_agent).send().await {
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
        //return Err(StatusCode::INTERNAL_SERVER_ERROR);
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

impl T3Data {
    pub fn get_author_flair(&self) -> Option<(&str, &str)> {
        match &self.author_flair_text {
            Some(t) => {
                match &self.author_flair_background_color {
                    Some(c) => Some((t, c)),
                    None => Some((t, "#000000"))
                }
            },
            None => None
        }
    }

    pub fn get_link_flair(&self) -> Option<(&str, &str)> {
        match &self.link_flair_text {
            Some(t) => {
                match &self.link_flair_background_color {
                    Some(c) => Some((t, c)),
                    None => Some((t, "#000000"))
                }
            },
            None => None
        }
    }
}

impl T1Data {
    pub fn get_author_flair(&self) -> Option<(&str, &str)> {
        match &self.author_flair_text {
            Some(t) => {
                match &self.author_flair_background_color {
                    Some(c) => Some((t, c)),
                    None => Some((t, "#000000"))
                }
            },
            None => None
        }
    }
}

// ?after=t3_16kksoi
pub async fn subreddit(
    client: &Client,
    subreddit: &str,
    sorting: Option<SortingMode>,
    after: Option<&str>,
    user_agent: &str,
) -> Result<SubredditQuery, StatusCode> {
    let sort = sorting.unwrap_or_default();

    let mut base = utils::get_reddit_domain();
    base.add_route("r");
    base.add_route(subreddit);

    match sort {
        SortingMode::Default => base.add_route(".json"),
        SortingMode::Hot => base.add_route("hot.json"),
        SortingMode::New => base.add_route("new.json"),
        SortingMode::Rising => base.add_route("rising.json"),
        SortingMode::Controversial => base.add_route("controversial.json"),
        SortingMode::Top(t) => {
            base.add_route("top.json");
            match t {
                TopSortingTime::PastHour => base.add_param("t", "hour"),
                TopSortingTime::Past24Hours => base.add_param("t", "day"),
                TopSortingTime::PastWeek => base.add_param("t", "week"),
                TopSortingTime::PastMonth => base.add_param("t", "month"),
                TopSortingTime::PastYear => base.add_param("t", "year"),
                TopSortingTime::AllTime => base.add_param("t", "all"),
            }
        }
    };

    if let Some(s) = after {
        base.add_param("after", s);
    }

    let url = base.build();

    let response = match client.get(url).header(USER_AGENT, user_agent).send().await {
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
        //return Err(StatusCode::INTERNAL_SERVER_ERROR);
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


// ?after=t3_16kksoi
pub async fn search(
    client: &Client,
    subreddit: &str,
    query: &str,
    sorting: Option<SearchSortingMode>,
    time_ordering: Option<SearchTimeOrdering>,
    after: Option<&str>,
    include_over_18: bool,
    only_current_subreddit: bool,
    user_agent: &str,
) -> Result<SubredditQuery, StatusCode> {
    let sort = sorting.unwrap_or_default();
    let order = time_ordering.unwrap_or_default();

    let mut base = utils::get_reddit_domain();
    base.add_route("r");
    base.add_route(subreddit);
    base.add_route("search.json");

    base.add_param("q", query);

    match sort {
        SearchSortingMode::Relevance => base.add_param("sort", "relevance"),
        SearchSortingMode::New => base.add_param("sort", "new"),
        SearchSortingMode::Comments => base.add_param("sort", "comments"),
    };

    match order {
        SearchTimeOrdering::PastHour => base.add_param("t", "hour"),
        SearchTimeOrdering::Past24Hours => base.add_param("t", "day"),
        SearchTimeOrdering::PastWeek => base.add_param("t", "week"),
        SearchTimeOrdering::PastMonth => base.add_param("t", "month"),
        SearchTimeOrdering::PastYear => base.add_param("t", "year"),
        SearchTimeOrdering::AllTime => base.add_param("t", "all"),
    };

    if only_current_subreddit {
        base.add_param("restrict_sr", "on");
    }

    if include_over_18 {
        base.add_param("include_over_18", "on");
    }

    if let Some(s) = after {
        base.add_param("after", s);
    }

    let url = base.build();

    let response = match client.get(url).header(USER_AGENT, user_agent).send().await {
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
        //return Err(StatusCode::INTERNAL_SERVER_ERROR);
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