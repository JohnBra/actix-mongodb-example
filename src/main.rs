#[macro_use]
extern crate bson;

use actix_web::{web, App, HttpServer};

use crate::logging::*;
use crate::common::get_env_or_panic;

mod common;
mod logging;
mod resource;
mod db;


fn get_binding_address() -> String {
    let port = get_env_or_panic("PORT");
    "0.0.0.0:".to_owned() + &port
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    init_logger();
    let binding_address = get_binding_address();

    HttpServer::new(|| App::new()
        .service(
            web::scope("/resource")
                .route("", web::get().to(resource::get_all))
                .route("", web::post().to(resource::save))
                .route("{id}", web::get().to(resource::get))
                .route("{id}", web::put().to(resource::update))
                .route("{id}", web::delete().to(resource::delete))
        ))
        .bind(&binding_address)
        .expect(&format!("Can not bind to {}", &binding_address) )
        .run()
        .await
}
