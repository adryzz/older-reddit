#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum SortingMode {
    #[default]
    Hot,
    New,
    Rising,
    Controversial,
    Top(TopSortingTime)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum TopSortingTime {
    PastHour,
    Past24Hours,
    #[default]
    PastWeek,
    PastMonth,
    PastYear,
    AllTime
}

