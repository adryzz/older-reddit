use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize)]
pub enum SortingMode {
    #[default]
    #[serde(rename = "suggested")]
    Default,
    #[serde(rename = "hot")]
    Hot,
    #[serde(rename = "new")]
    New,
    #[serde(rename = "rising")]
    Rising,
    #[serde(rename = "controversial")]
    Controversial,
    #[serde(rename = "top")]
    Top,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize)]
pub enum TopSortingTime {
    #[serde(rename = "hour")]
    PastHour,
    #[serde(rename = "day")]
    Past24Hours,
    #[default]
    #[serde(rename = "week")]
    PastWeek,
    #[serde(rename = "month")]
    PastMonth,
    #[serde(rename = "year")]
    PastYear,
    #[serde(rename = "all")]
    AllTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize)]
pub enum CommentSortingMode {
    #[default]
    #[serde(rename = "suggested")]
    Suggested,
    #[serde(rename = "best")]
    Best,
    #[serde(rename = "new")]
    New,
    #[serde(rename = "controversial")]
    Controversial,
    #[serde(rename = "old")]
    Old,
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "qa")]
    QAndA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize)]
pub enum SearchSortingMode {
    #[default]
    #[serde(rename = "relevance")]
    Relevance,
    #[serde(rename = "new")]
    New,
    #[serde(rename = "comments")]
    Comments,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize)]
pub enum SearchTimeOrdering {
    #[serde(rename = "hour")]
    PastHour,
    #[serde(rename = "day")]
    Past24Hours,
    #[serde(rename = "week")]
    PastWeek,
    #[serde(rename = "month")]
    PastMonth,
    #[serde(rename = "year")]
    PastYear,
    #[default]
    #[serde(rename = "all")]
    AllTime,
}
