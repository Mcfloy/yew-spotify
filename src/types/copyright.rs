use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Copyright {
    pub text: String,
    pub r#type: String
}