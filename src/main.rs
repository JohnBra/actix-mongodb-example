#[macro_use]
extern crate bson;
#[macro_use]
extern crate anyhow;

use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use actix_web::{web, App, HttpServer};

use crate::logging::*;

mod common;
mod logging;
mod resource;


lazy_static! {
    pub static ref MONGO: Client = create_mongo_client();
}

fn create_mongo_client() -> Client {
    Client::with_uri_str("mongodb://localhost:27017")
        .expect("Failed to initialize standalone client.")
}

fn collection(coll_name: &str) -> Collection {
    MONGO.database("db_name").collection(coll_name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    init_logger();

    let binding_address = "0.0.0.0:8000";
    HttpServer::new(|| App::new()
        .service(
            web::scope("/resource")
                .route("", web::get().to(resource::get_all))
                .route("", web::post().to(resource::save))
                .route("{id}", web::get().to(resource::get))
                .route("{id}", web::put().to(resource::update))
                .route("{id}", web::delete().to(resource::delete))
        ))
        .bind(binding_address)
        .expect(&format!("Can not bind to {}", binding_address) )
        .run()
        .await
}
