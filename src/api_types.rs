use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize)]
pub enum SortingMode {
    #[default]
    Default,
    Hot,
    New,
    Rising,
    Controversial,
    Top(TopSortingTime),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize)]
pub enum TopSortingTime {
    PastHour,
    Past24Hours,
    #[default]
    PastWeek,
    PastMonth,
    PastYear,
    AllTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize)]
pub enum CommentSortingMode {
    #[default]
    Suggested,
    Best,
    New,
    Controversial,
    Old,
    Top,
    QAndA,
}
