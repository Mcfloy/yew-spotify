use serde::{Deserialize, Serialize};
use crate::types::external_url::ExternalUrl;
use crate::types::image::Image;
use crate::types::followers::Followers;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PublicUser {
    pub display_name: Option<String>,
    pub external_urls: ExternalUrl,
    pub followers: Option<Followers>,
    pub href: String,
    pub id: String,
    pub images: Option<Vec<Image>>,
    pub r#type: String,
    pub uri: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PrivateUser{
    pub country: String, // private
    pub display_name: Option<String>,
    pub email: String, // private
    pub external_urls: ExternalUrl,
    pub followers: Followers,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub product: String, // private
    pub r#type: String,
    pub uri: String,
}