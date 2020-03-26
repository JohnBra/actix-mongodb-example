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
    let res = web::block(move || service::db_create_resource(resource))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_resource(
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let id = req.match_info().get("id").unwrap_or("");
    let res = web::block(move || service::db_read_resource(&id))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(HttpResponse::Ok().json(res))
}


pub async fn get_all_resources() -> Result<HttpResponse, Error> {
    let res = web::block(move || service::db_read_all_resources())
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));

    Ok(HttpResponse::Ok().json(res))
}

pub async fn update_resource(
    req: HttpRequest,
    resource: web::Json<Resource>
) -> Result<HttpResponse, Error> {
    let id = req.match_info().get("id").unwrap_or("");
    let resource = resource.into_inner();
    let res = web::block(move || service::db_update_resource(id, resource))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));

    Ok(HttpResponse::Ok().json(res))
}

pub async fn remove_resource(
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let id = req.match_info().get("id").unwrap_or("");
    let res = web::block(move || service::db_delete_resource(id))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));

    Ok(HttpResponse::Ok().json(res))
}