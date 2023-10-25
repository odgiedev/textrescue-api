use chrono::Utc;
use crate::models::auth_dto::Auth;
use crate::{repository::mongodb_repo::MongoRepo, models::paste_model::Paste, models::paste_dto::PasteDTO, auth::jwt_auth::verify_token};
use rocket::response::status;
use rocket::{post, get, http::Status, serde::json::Json, State};
use validator::Validate;
use crate::dtos::password_dto::PasswordDto;

#[post("/paste/create", data="<new_paste>")]
pub fn create_paste(db: &State<MongoRepo>, new_paste: Json<PasteDTO>
) -> Result<Json<&str>, status::Custom<String>> { 
    let paste = &new_paste.paste;
    let auth = &new_paste.auth;

    if auth.token.is_some() && auth.token.as_ref().unwrap() == "guest" {
        let paste_data = Paste {
            id: None,
            user_id: Some("123guestid".to_string()),
            username: Some("guest".to_string()),
            title: paste.title.to_owned(),
            paste: paste.paste.to_owned(),
            description: paste.description.to_owned(),
            tags: paste.tags.to_owned(),
            paste_password: Some(String::new()),
            is_private: Some(false),
            created_at: Some(Utc::now().date_naive().to_string().to_owned()),
        };
    
        match paste_data.validate() {
            Ok(_) => (),
            Err(err) => return Err(status::Custom(Status::BadRequest, err.to_string()))
        }   

        let paste = db.create_paste(paste_data);
    
        match paste {
            Ok(_) => Ok(Json("Paste created successfully.")),
            Err(_) => Err(status::Custom(Status::InternalServerError, "An error occurred while creating paste.".to_string())),
        }
    }
    else {
        let auth_data = Auth {
            token: auth.token.to_owned(),
        };
    
        let paste_data = Paste {
            id: None,
            user_id: paste.user_id.to_owned(),
            username: paste.username.to_owned(),
            title: paste.title.to_owned(),
            paste: paste.paste.to_owned(),
            description: paste.description.to_owned(),
            tags: paste.tags.to_owned(),
            paste_password: paste.paste_password.to_owned(),
            is_private: paste.is_private.to_owned(),
            created_at: Some(Utc::now().date_naive().to_string().to_owned()),
        };
    
        match auth_data.validate() {
            Ok(_) => (),
            Err(err) => return Err(status::Custom(Status::BadRequest, err.to_string()))
        } 
    
        match paste_data.validate() {
            Ok(_) => (),
            Err(err) => return Err(status::Custom(Status::BadRequest, err.to_string()))
        }   
    
        if !verify_token(&paste.user_id.as_ref().unwrap(), &auth.token.as_ref().unwrap()).unwrap_or(false) {
            return Err(status::Custom(Status::InternalServerError, "Not authorized.".to_string()));
        }
    
        let paste = db.create_paste(paste_data);
    
        match paste {
            Ok(_) => Ok(Json("Paste created successfully.")),
            Err(_) => Err(status::Custom(Status::InternalServerError, "An error occurred while creating paste.".to_string())),
        }
    }

}

#[post("/paste/<username>/<id>", data="<pass>")]
pub fn get_paste(db: &State<MongoRepo>, username: String, id: String, pass: Option<Json<PasswordDto>>) -> Result<Json<Paste>, status::Custom<String>> {
    if id.is_empty() {
        return Err(status::Custom(Status::BadRequest, "Paste ID is required.".to_string()));
    };

    if username.is_empty() {
        return Err(status::Custom(Status::BadRequest, "Username is required.".to_string()));
    };

    if pass.is_some() {
        let pass_paste = PasswordDto {
            password: pass.unwrap().password.to_owned()
        };

        match pass_paste.validate() {
            Ok(_) => (),
            Err(err) => return Err(status::Custom(Status::BadRequest, err.to_string()))
        }

        let paste = db.get_paste(&username, &id, pass_paste);

        return match paste {
            Ok(p) => Ok(Json(p)),
            Err(err) => Err(status::Custom(Status::Unauthorized, err.to_string())),
        }
    }

    let paste = db.get_paste(&username, &id, PasswordDto{password: None});

    match paste {
        Ok(p) => Ok(Json(p)),
        Err(err) => Err(status::Custom(Status::NotFound, err.to_string())),
    }

}

#[get("/paste/user/<id>/<page>")]
pub fn get_user_paste(db: &State<MongoRepo>, id: String, page: u64) -> Result<Json<Vec<Paste>>, status::Custom<&'static str>> {
    if id.is_empty() {
        return Err(status::Custom(Status::BadRequest, "Paste ID required or page query not found."));
    };

    let user_detail = db.get_user_paste(&id, &Some(page));

    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(status::Custom(Status::NotFound, "Pastes not found.")),
    }
}

#[delete("/paste/<paste_id>")]
pub fn delete_paste(db: &State<MongoRepo>, paste_id: String) -> Result<Json<&str>, status::Custom<&'static str>> {
    if paste_id.is_empty() {
        return Err(status::Custom(Status::BadRequest, "Invalid ID."));
    };

    let user_detail = db.delete_paste(&paste_id);

    match user_detail {
        Ok(_) => Ok(Json("Paste deleted successfully.")),
        Err(_) => Err(status::Custom(Status::InternalServerError, "Error while deleting paste.")),
    }
}

#[put("/paste/<paste_id>", data="<paste_up>")]
pub fn update_paste(db: &State<MongoRepo>, paste_id: String, paste_up: Json<PasteDTO>) -> Result<Json<&str>, status::Custom<String>> {
    if paste_id.is_empty() {
        return Err(status::Custom(Status::BadRequest, "Invalid ID.".to_string()));
    };

    let paste = &paste_up.paste;
    let auth = &paste_up.auth;

    let auth_data = Auth {
        token: auth.token.to_owned(),
    };

    let paste_to_update: Paste = Paste {
        id: None,
        user_id: paste.user_id.to_owned(),
        username: paste.username.to_owned(),
        title: paste.title.to_owned(),
        paste: paste.paste.to_owned(),
        description: paste.description.to_owned(),
        tags: paste.tags.to_owned(),
        paste_password: paste.paste_password.to_owned(),
        is_private: paste.is_private.to_owned(),
        created_at: Some(Utc::now().date_naive().to_string().to_owned()),
    };

    match auth_data.validate() {
        Ok(_) => (),
        Err(err) => return Err(status::Custom(Status::BadRequest, err.to_string()))
    } 

    match paste_to_update.validate() {
        Ok(_) => (),
        Err(err) => return Err(status::Custom(Status::BadRequest, err.to_string()))
    }   

    if !verify_token(&paste.user_id.as_ref().unwrap(), &auth.token.as_ref().unwrap()).unwrap_or(false) {
        return Err(status::Custom(Status::InternalServerError, "Not authorized.".to_string()));
    }

    let paste_detail = db.update_paste(&paste_id, paste_to_update);
    
    match paste_detail {
        Ok(_) => Ok(Json("Paste updated successfully.")),
        Err(_) => Err(status::Custom(Status::NotFound, "Paste not found.".to_string())),
    }
}

#[post("/paste/search/tags/<tags>")]
pub fn search_paste(db: &State<MongoRepo>, tags: String) -> Result<Json<Vec<Paste>>, status::Custom<&'static str>> {
    let paste_detail = db.search_paste(&tags);

    match paste_detail {
        Ok(paste) => Ok(Json(paste)),
        Err(_) => Err(status::Custom(Status::NotFound, "No results.")),
    }
}