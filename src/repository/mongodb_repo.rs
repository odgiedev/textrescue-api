extern crate dotenv;
use std::env;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::InsertOneResult,
    sync::{Client, Collection,}, options::FindOptions,
};
use crate::models::{user_model::User, user_signin_model::UserSignIn};
use crate::models::paste_model::Paste;
use rocket::serde::json::Json;
use crate::dtos::password_dto::PasswordDto;


pub struct MongoRepo {
    col_user: Collection<User>,
    col_paste: Collection<Paste>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("tr_web");
        let col_user: Collection<User> = db.collection("users");
        let col_paste: Collection<Paste> = db.collection("pastes");
        MongoRepo { col_user, col_paste }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Json<String>> {
        let check_user = UserSignIn {
            id: None,
            email: new_user.email.to_owned(),
            passwd: new_user.passwd.to_owned(),
        };

        match self.get_user_by_email(&check_user) {
            Ok(_) => return Err(Json("Email already exists.".to_string())),
            _ => () 
        };

        let n_user = match self.col_user.insert_one(new_user, None) {
            Ok(u) => u,
            _ => return Err(Json("An error occured while creating the user".to_string()))
        };
        
        Ok(n_user)
    }

    pub fn get_user_by_email(&self, user_data: &UserSignIn) -> Result<User, ()> {
        let user_detail = self.col_user.find_one(doc!{"email": user_data.email.to_owned()}, None).unwrap_or(None);
        if user_detail == None {
            return Err(());
        }

        Ok(user_detail.unwrap())
    }

    //PASTE

    pub fn get_user_paste(&self, user_id: &String, page_param: &Option<u64>) -> Result<Vec<Paste>, ()> {
        //let userid = ObjectId::parse_str(user_id).unwrap_or(ObjectId::new());

        let page = match page_param {
            Some(p) => p,
            _ => &1
        };

        let to_skip = (page - 1) * 10;
        
        let filter_public = doc! {"user_id": user_id};
        let find_options = FindOptions::builder()
            .limit(10)
            .skip(to_skip)
            .build();

        let cursors_public = self.col_paste.find(filter_public, find_options).ok().unwrap();
        let user_paste: Vec<Paste> = cursors_public.map(|doc| {
            doc.unwrap()
        }).collect();

        /*let filter_private = doc! {"user_id": user_id};
        let cursors_private = self.col_paste.find(filter_private, None).ok().unwrap();
        let user_paste_private: Vec<Paste> = cursors_private.map(|doc| {
            doc.unwrap()
        }).collect();*/

        Ok(user_paste)
    }

    pub fn create_paste(&self, new_paste: Paste) -> Result<InsertOneResult, ()>{
        let paste = match self.col_paste.insert_one(new_paste, None) {
            Ok(paste) => paste,
            _ => return Err(()),
        };
        Ok(paste)
    }

    pub fn get_paste(&self, username: &String, id: &String, paste_pass: PasswordDto) -> Result<Paste, String> {
        let obj_id = ObjectId::parse_str(id).unwrap_or(ObjectId::new());
        let filter = doc! {"_id": obj_id, "username": username};
        let paste_detail = self.col_paste.find_one(filter, None).unwrap_or(None);
        if paste_detail == None {
            return Err("NF,Not Found.".to_string());
        }

        let paste_detail = paste_detail.unwrap();

        if paste_detail.is_private.is_some() && paste_detail.paste_password.is_some() {
            if paste_pass.password.is_some() {
                if paste_detail.paste_password.as_ref().unwrap() == paste_pass.password.as_ref().unwrap() {
                    return Ok(paste_detail);
                } else {
                    return Err("XIPP,Invalid password.".to_string());
                }
            }
        }

        Ok(Paste{
            id: None,
            user_id: None,
            username: None,
            title: None,
            paste: None,
            description: None,
            tags: None,
            paste_password: None,
            is_private: None,
            created_at: None,
        })
    }

    pub fn delete_paste(&self, id: &String) -> Result<Paste, ()> {
        let obj_id = match ObjectId::parse_str(id) {
            Ok(obj_id) => obj_id,
            _ => return Err(()),
        };

        let filter = doc! {"_id": obj_id};

        let paste_detail = match self.col_paste.find_one_and_delete(filter, None){
            Ok(paste) => paste,
            _ => return Err(()),
        };

        if paste_detail == None {
            return Err(());
        }

        Ok(paste_detail.unwrap())
    }

    pub fn update_paste(&self, id: &String, paste_to_update: Paste) -> Result<Paste, ()> {
        let obj_id = match ObjectId::parse_str(id) {
            Ok(obj_id) => obj_id,
            _ => return Err(()),
        };

        let filter = doc! {"_id": obj_id};

        let paste_data = doc! {
            "$set": {
                "title": paste_to_update.title,
                "paste": paste_to_update.paste,
                "description": paste_to_update.description,
                "tags": paste_to_update.tags,
                "paste_password": paste_to_update.paste_password,
            }
        };

        let paste_update = match self.col_paste.find_one_and_update(filter, paste_data, None){
            Ok(paste) => paste,
            _ => return Err(()),
        };
        
        if paste_update == None {
            return Err(());
        }
        
        Ok(paste_update.unwrap())
    }

    pub fn search_paste(&self, tag: &String) -> Result<Vec<Paste>, ()> {
        let filter = doc! { "$and": [{"paste_password": {"$eq": ""}, "tags": {"$regex": tag, "$options": "i"}}]};
        
        let paste_cursor = match self.col_paste.find(filter, None){
            Ok(paste) => paste,
            _ => return Err(()),
        };
        
        let res: Vec<Paste> = paste_cursor.map(|doc| doc.unwrap()).collect();

        Ok(res)
    }
}