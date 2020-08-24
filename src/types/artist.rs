use serde::{Deserialize, Serialize};
use crate::types::ExternalUrl;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SimplifiedArtist {
    pub external_urls: ExternalUrl,
    pub href: String,
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub uri: String,
}