use actix_multipart::Multipart;
use actix_web::{get, post, put, web, Responder};

use crate::services::evento_service;

#[post("/evento")]
async fn post_evento(payload: Multipart) -> impl Responder {
    match evento_service::insert(payload).await {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[put("/evento/{id}")]
async fn put_evento(id: web::Path<i32>, payload: Multipart) -> impl Responder {
    match evento_service::update(id.into_inner(), payload).await {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[get("/evento/{id}/imagem")]
async fn get_evento_icone(id: web::Path<i32>) -> impl Responder {
    match evento_service::find_icone(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[get("/evento/{id}")]
async fn get_evento_by_id(id: web::Path<i32>) -> impl Responder {
    match evento_service::find_by_id(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[get("/evento")]
async fn get_eventos() -> impl Responder {
    match evento_service::find_all() {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}
