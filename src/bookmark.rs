use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Bookmarks {
    pub bookmark: Vec<Bookmark>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bookmark {
    pub url: String,
    pub inner: String,
}
