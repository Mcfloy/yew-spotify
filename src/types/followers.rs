use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Followers {
    pub href: Option<String>,
    pub total: i32
}