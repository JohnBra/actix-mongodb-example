use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;

pub use controller::*;
use crate::common::*;

mod controller;


impl Resource {
    pub const COLLECTION_NAME: &'static str = "resource";
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Resource {
    #[serde(serialize_with = "serialize_object_id")]
    _id: Option<ObjectId>,
    some_key_1: String,
    some_key_2: String,
    some_key_3: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResourceQuery {
    #[serde(deserialize_with = "deserialize_object_id", default)]
    _id: Option<ObjectId>,
    #[serde(default)]
    keyword: String,
}