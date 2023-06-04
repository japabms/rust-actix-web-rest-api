use actix_web::{delete, get, post, web, Responder};

use crate::models::categoria::NewCategoria;
use crate::services::categoria_service;

#[get("/categoria")]
async fn get_categoria() -> impl Responder {
    match categoria_service::find_all() {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[post("/categoria")]
async fn post_categoria(json: web::Json<NewCategoria>) -> impl Responder {
    match categoria_service::insert(json.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[delete("/categoria/{id}")]
async fn delete_categoria(id: web::Path<i32>) -> impl Responder {
    match categoria_service::delete(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}
