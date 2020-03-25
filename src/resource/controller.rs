use actix_web::{HttpResponse, web, HttpRequest, Responder, Error, http};
use bson::{oid::ObjectId, Document};
use log::*;

use super::{Resource, ResourceQuery, service};

use crate::collection;
use crate::common::*;

pub async fn save_resource(
    resource: web::Json<Resource>
) -> Result<HttpResponse, Error> {
    let resource: Resource = resource.into_inner();
    let res = web::block(move || service::save_resource(resource))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_resource(
    req: HttpRequest
) -> impl Responder {
    let coll = collection(Resource::COLLECTION_NAME);
    let id = req.match_info().get("id").unwrap_or("");
    let result = coll.find_one(Some(doc! {"_id" => &id}), None);
    HttpResponse::Ok().json(result)
}


pub async fn get_all_resources(
    query: web::Json<ResourceQuery>
) -> impl Responder {
    let query = query.into_inner();

    let mut d: Document = doc! {};

    if !query.keyword.is_empty() {
        d.insert("$or", bson::Bson::Array(vec![
            doc! {"some_key_1": {"$regex": & query.keyword, "$options": "i"}}.into(),
            doc! {"some_key_2": {"$regex": & query.keyword, "$options": "i"}}.into(),
            doc! {"some_key_3": {"$regex": & query.keyword, "$options": "i"}}.into(),
        ]));
    }

    let coll = collection(Resource::COLLECTION_NAME);
    let cursor = coll.find(Some(d), None);
    let result = cursor.map(|mut x| x.as_vec::<Resource>());

    HttpResponse::Ok().json(result)
}

pub async fn update_resource(
    req: HttpRequest,
    resource: web::Json<Resource>
) -> impl Responder {
    let id = req.match_info().get("id").unwrap_or("");
    let resource = resource.into_inner();
    let filter = doc! {"_id" => id};
    let update_doc = doc! {"$set": struct_to_document(&resource).unwrap()};

    let effect = match collection(Resource::COLLECTION_NAME).update_one(filter, update_doc, None) {
        Ok(result) => {
            info!("update resource, id={}, effect={}", id, result.modified_count);
            result.modified_count
        }
        Err(e) => {
            error!("update_resource, failed to visit db, id={}, {}", id, e);
            return Err(anyhow!(e));
        }
    };

    HttpResponse::Ok().json(effect)
}

pub async fn remove_resource(
    req: HttpRequest
) -> impl Responder {
    let id = req.match_info().get("id").unwrap_or("");
    let filter = doc! {"_id" => ObjectId::with_string(id).unwrap()};
    let coll = collection(Resource::COLLECTION_NAME);

    let effect = match coll.delete_one(filter, None) {
        Ok(result) => {
            info!("delete resource, id={}, effect={}", id, result.deleted_count);
            result.deleted_count
        }
        Err(e) => {
            error!("remove_resource, failed to visit db, id={}, {}", id, e);
            return Err(anyhow!(e));
        }
    };

    HttpResponse::Ok().json(effect)
}