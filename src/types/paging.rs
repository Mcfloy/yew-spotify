use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Paging<T> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: i64,
    pub next: Option<String>,
    pub offset: i64,
    pub previous: Option<String>,
    pub total: i64
}
