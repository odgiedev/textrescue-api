use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use validator::Validate;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PASS_RGX: Regex = Regex::new(r"^[a-zA-Z]*[0-9]+[a-zA-Z0-9!@#$%^&*()_+{}\[\]:;<>,.?~\-=\\/]*[!@#$%^&*()_+{}\[\]:;<>,.?~\-=\\/][a-zA-Z0-9!@#$%^&*()_+{}\[\]:;<>,.?~\-=\\/]*$").unwrap();
    static ref EMAIL_RGX: Regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
}

#[derive(Debug, Validate, Serialize, Deserialize, PartialEq)]
pub struct UserSignIn {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[validate(required, regex(path="EMAIL_RGX", message="Please enter a valid email address."), length(max=100, message="Must be less than 100 characters."))]
    pub email: Option<String>,
    #[validate(required)]
    pub passwd: Option<String>,
}