use mongodb::{Client, Collection};
use lazy_static::lazy_static;

use crate::common::get_env_or_panic;

// TODO: possibly remove lazy static reference to simplify
lazy_static! {
    pub static ref MONGO: Client = create_mongo_client();
}

fn create_mongo_client() -> Client {
    let mongo_connection_string = get_connection_string();
    Client::with_uri_str(&mongo_connection_string)
        .expect("Failed to initialize standalone mongo client.")
}

fn get_connection_string() -> String {
    let host = get_env_or_panic("MONGO_HOST");  // TODO check if this is shit for performance
    let port = get_env_or_panic("MONGO_PORT");  // TODO check if this is shit for performance
    "mongodb://".to_owned() + &host + ":" + &port
}

pub fn collection(coll_name: &str) -> Collection {
    MONGO.database("db_name").collection(coll_name)
}