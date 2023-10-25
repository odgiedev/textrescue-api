use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PasswordDto {
    pub password: Option<String>
}