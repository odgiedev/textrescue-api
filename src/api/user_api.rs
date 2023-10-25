use crate::{models::user_model::User, models::user_signin_model::UserSignIn, repository::mongodb_repo::MongoRepo, auth::jwt_auth::create_token};
use rocket::{post, get, http::Status, serde::json::Json, State};
use rocket::response::status;
use validator::Validate;
use bcrypt::{DEFAULT_COST, hash, verify};

#[get("/")]
pub fn index() -> String {
    format!("Hello, World!")
}

#[post("/user/signin", data="<user_data>")]
pub fn signin(db: &State<MongoRepo>, user_data: Json<UserSignIn>) -> Result<Json<String>, status::Custom<String>> {
    let user_data = UserSignIn {
        id: None,
        email: user_data.email.to_owned(),
        passwd: user_data.passwd.to_owned(),
    };

    match user_data.validate() {
        Ok(_) => (),
        Err(err) => return Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
    
    if user_data.passwd.to_owned().unwrap().is_empty() {
        return Err(status::Custom(Status::BadRequest, "Password is required.".to_string()))
    }

    let user = match db.get_user_by_email(&user_data) {
        Ok(user) => user,
        _ => return Err(status::Custom(Status::UnprocessableEntity, "Invalid email.".to_string()))
    };
    
    let buff_passwd_user = user.passwd.clone().unwrap(); 
    let buff_passwd_form = user_data.passwd.unwrap();

    let passwd_user = buff_passwd_user.as_str();
    let passwd_form = buff_passwd_form.as_str();

    let valid = verify(passwd_form, passwd_user);

    if valid.unwrap() {
        let user_id = user.id.unwrap().to_string();
        let token_msg = format!("token {} user_id {} username {}", create_token(&user_id), &user_id, &user.username.unwrap());
        Ok(Json(token_msg))
    } else {
        return Err(status::Custom(Status::UnprocessableEntity, "Invalid password.".to_string()))
    }
}

#[post("/user/create", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<&str>, status::Custom<String>> {
    let p = new_user.passwd.as_ref().unwrap();

    if p.is_empty() {
        return Err(status::Custom(Status::BadRequest, "Password is required.".to_string()));
    }

    let hash_pass = match hash(p, DEFAULT_COST) {
        Ok(hash) => hash,
        _ => return Err(status::Custom(Status::InternalServerError, "An error ocurred.".to_string()))
    };

    let mut data = User {
        id: None,
        username: new_user.username.to_owned(),
        email: new_user.email.to_owned(),
        passwd: new_user.passwd.to_owned()
    };

    match data.validate() {
        Ok(_) => (),
        Err(err) => return Err(status::Custom(Status::BadRequest, err.to_string()))
    }

    data.passwd = Some(hash_pass);

    let user_detail = db.create_user(data);

    match user_detail {
        Ok(_) => Ok(Json("User created.")),
        Err(err) => return Err(status::Custom(Status::BadRequest, err.to_string()))
    }
}