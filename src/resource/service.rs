use bson::{oid::ObjectId, Document};
use log::*;

use super::Resource;

use crate::collection;
use crate::common::*;


pub fn db_create_resource(
    resource: Resource
) -> Result<std::option::Option<bson::ordered::OrderedDocument>, mongodb::error::Error> {
    let d: Document = struct_to_document(&resource).unwrap();
    let coll = collection(Resource::COLLECTION_NAME);
    let insertion_result = coll.insert_one(d, None)?;
    let new_id: String = insertion_result.inserted_id
        .as_object_id()
        .map(ObjectId::to_hex)
        .unwrap();
    info!("save resource, id={}", new_id);
    let res = coll.find_one(
        Some(doc! {"_id" => ObjectId::with_string(&new_id).unwrap()}),
        None
    );
    Ok(res.unwrap())
}

pub fn db_read_resource(
    id: &str
) -> Result<std::option::Option<bson::ordered::OrderedDocument>, mongodb::error::Error> {
    let coll = collection(Resource::COLLECTION_NAME);
    let res = coll.find_one(
        Some(doc! {"_id" => ObjectId::with_string(id).unwrap()}),
        None);
    Ok(res.unwrap())
}

pub fn db_read_all_resources(
) -> Result<std::vec::Vec<Resource>, mongodb::error::Error> {
    let coll = collection(Resource::COLLECTION_NAME);
    let cursor = coll.find(None, None);
    let res = cursor.map(|mut x| x.as_vec::<Resource>());
    Ok(res.unwrap())
}

pub fn db_update_resource(
    id: &str,
    resource: Resource
) -> Result<std::option::Option<bson::ordered::OrderedDocument>, mongodb::error::Error> {
    let coll = collection(Resource::COLLECTION_NAME);
    let filter = doc! {"_id" => ObjectId::with_string(id).unwrap()};
    let update_doc = doc! {"$set": struct_to_document(&resource).unwrap()};

    let effect = coll.update_one(filter, update_doc, None);
    if effect.unwrap().modified_count < 1 {
        ()
    }

    let res = coll.find_one(
        Some(doc! {"_id" => ObjectId::with_string(id).unwrap()}),
        None
    );
    Ok(res.unwrap())
}

pub fn db_delete_resource(
    id: &str
) -> Result<(), mongodb::error::Error> {
    let coll = collection(Resource::COLLECTION_NAME);
    let filter = doc! {"_id" => ObjectId::with_string(id).unwrap()};

    let effect = coll.delete_one(filter, None);
    if effect.unwrap().deleted_count < 1 {
        ()
    }
    Ok(())
}
