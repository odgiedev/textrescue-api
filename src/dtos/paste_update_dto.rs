use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PasteUpdateDto {
    pub title: Option<String>,
    pub paste: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub paste_password: Option<String>,
    pub is_private: Option<bool>,
}