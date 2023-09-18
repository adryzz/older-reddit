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
                if let Some(num) = num.as_u64() {
                    Ok(EditTimestamp(Some(num)))
                } else {
                    Err(serde::de::Error::custom("Invalid number format"))
                }
            }
            serde_json::Value::Bool(false) => Ok(EditTimestamp(None)),
            _ => Err(serde::de::Error::custom("Invalid value for EditTimestamp")),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ApiData {
    Single(RedditData),
    Collection(Vec<RedditData>)
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
            _ => Err(serde::de::Error::custom("Invalid JSON structure for ApiData")),
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
    T3(T3Data)
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListingData {
    dist: i32,
    after: String,
    children: Vec<RedditData>,
    before: String
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
            // Handle other variants as needed
            _ => Err(serde::de::Error::custom("Unknown variant")),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WikiPageData {
}

/// Comment
#[derive(Debug, Clone, Deserialize)]
pub struct T1Data {
    id: String,
    subreddit: String,
    body: String,
    score: i32,
    author: String,
    edited: EditTimestamp,
    locked: bool,
    stickied: bool,
    created_utc: u64
}

/// Post listing, post
#[derive(Debug, Clone, Deserialize)]
pub struct T3Data {
    id: String,
    subreddit: String,
    selftext: String,
    title: String,
    score: i32,
    author: String,
    edited: EditTimestamp,
    locked: bool,
    stickied: bool,
    spoiler: bool,
    created_utc: u64,
    thumbnail: Option<String>,
    upvote_ratio: f32,
    archived: bool,
    pinned: bool,
    over_18: bool,
    author_flair_text: Option<String>,
    num_comments: u32,
    subreddit_subscribers: u32,
    is_video: bool
}