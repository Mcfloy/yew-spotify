use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Image {
    pub url: String,
    pub width: Option<i32>,
    pub height: Option<i32>
}