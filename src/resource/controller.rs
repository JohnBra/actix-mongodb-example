use actix_web::{HttpResponse, web, Error, http};

use super::{Resource, service};

pub async fn save(
    resource: web::Json<Resource>
) -> Result<Result<HttpResponse, HttpResponse>, Error> {
    let resource: Resource = resource.into_inner();
    let res = web::block(move || service::db_create_resource(resource))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(res)
}

pub async fn get(
    id: web::Path<String>
) -> Result<Result<HttpResponse, HttpResponse>, Error> {
    let res = web::block(move || service::db_read_resource(&id))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(res)
}


pub async fn get_all() -> Result<Result<HttpResponse, HttpResponse>, Error> {
    let res = web::block(move || service::db_read_all_resources())
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));

    Ok(res)
}

pub async fn update(
    id: web::Path<String>,
    resource: web::Json<Resource>
) -> Result<Result<HttpResponse, HttpResponse>, Error> {
    let resource = resource.into_inner();
    let res = web::block(move || service::db_update_resource(&id, resource))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));

    Ok(res)
}

pub async fn delete(
    id: web::Path<String>
) -> Result<Result<HttpResponse, HttpResponse>, Error> {
    let res = web::block(move || service::db_delete_resource(&id))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));

    Ok(res)
}