#[macro_use]
extern crate bson;
#[macro_use]
extern crate anyhow;

use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use actix_web::{web, App, HttpServer, FromRequest};

use crate::resource::Resource;
use crate::common::*;
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
        .app_data(
              web::Json::<Resource>::configure(|cfg| {
                  cfg.error_handler(|err, req| {
                      log::error!("json extractor error, path={}, {}", req.uri(), err);
                      BusinessError::ArgumentError.into()
                  })
              })
        )
        .service(
            web::scope("/resource")
                .route("", web::get().to(resource::list_resource))
                .route("", web::post().to(resource::save_resource))
                .route("{id}", web::put().to(resource::update_resource))
                .route("{id}", web::delete().to(resource::remove_resource))
        ))
        .bind(binding_address)
        .expect(&format!("Can not bind to {}", binding_address) )
        .run()
        .await
}
