//#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;

mod api;
mod models;
mod repository;
mod auth;
mod dtos;

use rocket::{launch, Build, routes, Rocket};
use api::user_api::{index, create_user, signin};
use api::paste_api::{create_paste, get_paste, get_user_paste, delete_paste, update_paste, search_paste};

use repository::mongodb_repo::MongoRepo;


use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[catch(400)]
fn bad_request() -> &'static str {
    "Bad Request. It's OK with your JSON?"
} 

#[catch(422)]
fn unprocessable_entity() -> &'static str {
    "Unprocessable Entity. It's OK with your JSON?"
} 

#[launch]
async fn rocket() -> Rocket<Build> {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete, Method::Put].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();

    let db = MongoRepo::init();

    rocket::build()
        .manage(db)
        .mount("/", routes![index, create_user, signin])
        .mount("/", routes![create_paste, get_paste, get_user_paste, delete_paste, update_paste, search_paste])
        .register("/", catchers![bad_request, unprocessable_entity])
        .attach(cors)          
}
