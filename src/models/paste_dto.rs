use crate::models::{paste_model::Paste, auth_dto::Auth};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PasteDTO {
    pub paste: Paste,
    pub auth: Auth
}