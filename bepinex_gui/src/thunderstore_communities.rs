use serde::{Deserialize, Serialize};

pub const URL: &str = "https://thunderstore.io/api/experimental/community/";

#[derive(Debug, Serialize, Deserialize)]
pub struct Communities {
    pub pagination: Option<Pagination>,
    pub results: Option<Vec<Result>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub next_link: Option<serde_json::Value>,
    pub previous_link: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    pub identifier: Option<String>,
    pub name: Option<String>,
    pub discord_url: Option<String>,
    pub wiki_url: Option<String>,
    pub require_package_listing_approval: Option<bool>,
}
