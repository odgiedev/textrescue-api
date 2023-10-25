use serde::{Serialize, Deserialize};
use validator::Validate;
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, PartialEq, Validate)]
pub struct Paste {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[validate(required, length(min=1))]
    pub user_id: Option<String>,
    #[validate(required, length(min=1))]
    pub username: Option<String>,
    #[validate(required, length(min=1, max=90, message="Must be less than 90 characters."))]
    pub title: Option<String>,
    #[validate(required, length(min=1, message="Is required."))]
    pub paste: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub paste_password: Option<String>,
    pub is_private: Option<bool>,
    pub created_at: Option<String>,
}
