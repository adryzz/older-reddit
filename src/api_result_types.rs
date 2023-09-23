use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct EditTimestamp(Option<u64>);

impl<'de> Deserialize<'de> for EditTimestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        match value {
            serde_json::Value::Number(num) => {
                dbg!(&num);
                if let Some(num) = num.as_u64() {
                    Ok(EditTimestamp(Some(num)))
                } else {
                    Err(serde::de::Error::custom("invalid number format"))
                }
            }
            serde_json::Value::Bool(false) => Ok(EditTimestamp(None)),
            _ => Err(serde::de::Error::custom("invalid value for EditTimestamp")),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ApiData {
    Single(RedditData),
    Collection(Vec<RedditData>),
}

impl<'de> Deserialize<'de> for ApiData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        match &value {
            serde_json::Value::Array(_) => {
                let reddit_data: Vec<RedditData> = serde_json::from_value(value)
                    .map_err(|e| serde::de::Error::custom(e.to_string()))?;
                Ok(ApiData::Collection(reddit_data))
            }
            serde_json::Value::Object(_) => {
                let reddit_data: RedditData = serde_json::from_value(value)
                    .map_err(|e| serde::de::Error::custom(e.to_string()))?;
                Ok(ApiData::Single(reddit_data))
            }
            _ => Err(serde::de::Error::custom(
                "invalid JSON structure for ApiData",
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RedditData {
    Listing(ListingData),
    /// Comment
    T1(T1Data),
    /// Wiki page
    WikiPage(WikiPageData),
    /// Post listing, Post
    T3(T3Data),
    /// Anything else, to be discarded
    Unknown(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListingData {
    pub dist: Option<i32>,
    pub after: Option<String>,
    pub children: Vec<RedditData>,
    pub before: Option<String>,
}

#[derive(Debug, Deserialize)]
struct InnerData {
    kind: String,
    data: serde_json::Value, // Using serde_json::Value to capture the inner object
}

impl<'de> Deserialize<'de> for RedditData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner_data: InnerData = Deserialize::deserialize(deserializer)?;

        match inner_data.kind.as_str() {
            "Listing" => {
                let listing_data: ListingData = serde_json::from_value(inner_data.data)
                    .map_err(|e| serde::de::Error::custom(e.to_string()))?;
                Ok(RedditData::Listing(listing_data))
            }
            "t1" => {
                let t1_data: T1Data = serde_json::from_value(inner_data.data)
                    .map_err(|e| serde::de::Error::custom(e.to_string()))?;
                Ok(RedditData::T1(t1_data))
            }
            "t3" => {
                let t3_data: T3Data = serde_json::from_value(inner_data.data)
                    .map_err(|e| serde::de::Error::custom(e.to_string()))?;
                Ok(RedditData::T3(t3_data))
            }
            "wikipage" => {
                let wiki_data: WikiPageData = serde_json::from_value(inner_data.data)
                    .map_err(|e| serde::de::Error::custom(e.to_string()))?;
                Ok(RedditData::WikiPage(wiki_data))
            }
            // Handle other variants as needed
            _ => Ok(RedditData::Unknown(inner_data.kind)),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WikiPageData {
    pub content_md: String,
    pub revision_date: u64
}

/// Comment
#[derive(Debug, Clone, Deserialize)]
pub struct T1Data {
    pub id: String,
    pub subreddit: String,
    pub body: String,
    pub score: i32,
    pub author: String,
    //pub edited: EditTimestamp,
    pub locked: bool,
    pub stickied: bool,
    //pub created_utc: u64,
    pub replies: ReplyList,
    pub author_flair_text: Option<String>,
    pub author_flair_background_color: Option<String>,
}

/// Post listing, post
#[derive(Debug, Clone, Deserialize)]
pub struct T3Data {
    pub id: String,
    pub subreddit: String,
    pub selftext: String,
    pub title: String,
    pub score: i32,
    pub author: String,
    //pub edited: EditTimestamp,
    pub locked: bool,
    pub stickied: bool,
    pub spoiler: bool,
    //pub created_utc: u64,
    pub thumbnail: Option<String>,
    pub upvote_ratio: f32,
    pub archived: bool,
    pub pinned: bool,
    pub over_18: bool,
    pub author_flair_text: Option<String>,
    pub author_flair_background_color: Option<String>,
    pub num_comments: u32,
    pub subreddit_subscribers: u32,
    pub is_video: bool,
    pub is_gallery: Option<bool>,
    pub is_reddit_media_domain: bool,
    pub link_flair_text: Option<String>,
    pub link_flair_background_color: Option<String>,
    pub url: Option<String>,
    // gallery_data
    // media_metadata
    // poll_data
}

#[derive(Debug, Clone)]
pub enum ReplyList {
    None,
    Replies(ListingData),
}

impl<'de> Deserialize<'de> for ReplyList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: InnerData = match Deserialize::deserialize(deserializer) {
            Ok(v) => v,
            Err(_) => return Ok(ReplyList::None),
        };

        Ok(ReplyList::Replies(
            serde_json::from_value(value.data).map_err(|e| serde::de::Error::custom(e))?,
        ))
    }
}
