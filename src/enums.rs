#[derive(Eq, PartialEq)]
pub(crate) enum Sorting {
    Random,
    Toplist,
    Hot,
}

impl Sorting {
    pub fn file(&self) -> Option<String> {
        match self {
            Sorting::Random => None,
            Sorting::Toplist => Some(".toplist".to_string()),
            Sorting::Hot => Some(".hot".to_string()),
        }
    }
    pub fn param(&self) -> String {
        match self {
            Sorting::Random => "random".to_string(),
            Sorting::Toplist => "toplist".to_string(),
            Sorting::Hot => "hot".to_string(),
        }
    }
}
