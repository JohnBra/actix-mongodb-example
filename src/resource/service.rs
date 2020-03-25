use bson::{oid::ObjectId, Document};

use super::Resource;

use crate::collection;
use crate::common::*;

pub fn save_resource(
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
    let result = coll.find_one(Some(doc! {"_id" => &new_id}), None);
    Ok(result)
}
