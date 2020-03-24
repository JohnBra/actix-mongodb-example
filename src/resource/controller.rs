use actix_web::{HttpResponse, web, HttpRequest};
use bson::{oid::ObjectId, Document};
use log::*;

use super::Resource;
use crate::collection;
use crate::common::*;
use crate::resource::ResourceQuery;


pub async fn save_resource(
    resource: web::Json<Resource>
) -> Result<HttpResponse, BusinessError> {
    let article: Resource = resource.into_inner();
    let d: Document = struct_to_document(&article).unwrap();

    let rs = collection(Resource::COLLECTION_NAME).insert_one(d, None)?;
    let new_id: String = rs.inserted_id
        .as_object_id()
        .map(ObjectId::to_hex)
        .unwrap();
    info!("save resource, id={}", new_id);
    Resp::ok(new_id).to_json_result()
}


pub async fn list_resource(
    query: web::Json<ResourceQuery>
) -> Result<HttpResponse, BusinessError> {
    let query = query.into_inner();

    // 构造查询参数
    let mut d: Document = doc! {};
    if query._id.is_some() {
        d.insert("_id", query._id.unwrap());
    }

    if !query.keyword.is_empty() {
        d.insert("$or", bson::Bson::Array(vec![
            doc! {"some_key_1": {"$regex": & query.keyword, "$options": "i"}}.into(),
            doc! {"some_key_2": {"$regex": & query.keyword, "$options": "i"}}.into(),
            doc! {"some_key_3": {"$regex": & query.keyword, "$options": "i"}}.into(),
        ]));
    }

    let coll = collection("resource");
    let cursor = coll.find(Some(d), None);
    let result = cursor.map(|mut x| x.as_vec::<Resource>());
    match result {
        Ok(list) => Resp::ok(list).to_json_result(),
        Err(e) => {
            error!("list_resource error, {}", e);
            return Err(BusinessError::InternalError { source: anyhow!(e) });
        }
    }
}

pub async fn update_resource(
    req: HttpRequest,
    resource: web::Json<Resource>
) -> Result<HttpResponse, BusinessError> {
    let id = req.match_info().get("id").unwrap_or("");

    let oid = ObjectId::with_string(id).map_err(|e| {
        log::error!("update_resource, can't parse id to ObjectId, {:?}", e);
        BusinessError::ValidationError { field: "id".to_owned() }
    })?;

    let resource = resource.into_inner();

    let filter = doc! {"_id" => oid};

    let update = doc! {"$set": struct_to_document( & resource).unwrap()};

    let effect = match collection(Resource::COLLECTION_NAME).update_one(filter, update, None) {
        Ok(result) => {
            info!("update resource, id={}, effect={}", id, result.modified_count);
            result.modified_count
        }
        Err(e) => {
            error!("update_resource, failed to visit db, id={}, {}", id, e);
            return Err(BusinessError::InternalError { source: anyhow!(e) });
        }
    };

    Resp::ok(effect).to_json_result()
}

pub async fn remove_resource(
    req: HttpRequest
) -> Result<HttpResponse, BusinessError> {
    let id = req.match_info().get("id").unwrap_or("");
    if id.is_empty() {
        return Err(BusinessError::ValidationError { field: "id".to_owned() });
    }

    let filter = doc! {"_id" => ObjectId::with_string(id).unwrap()};

    let effect = match collection(Resource::COLLECTION_NAME).delete_one(filter, None) {
        Ok(result) => {
            info!("delete resource, id={}, effect={}", id, result.deleted_count);
            result.deleted_count
        }
        Err(e) => {
            error!("remove_resource, failed to visit db, id={}, {}", id, e);
            return Err(BusinessError::InternalError { source: anyhow!(e) });
        }
    };

    Resp::ok(effect).to_json_result()
}