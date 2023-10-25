use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, PartialEq)]
pub struct Auth {
    #[validate(required, length(min=1, message="Is required to auth."))]
    pub token: Option<String>
}