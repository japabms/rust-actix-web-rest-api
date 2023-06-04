use actix_multipart::Multipart;
use actix_web::{get, post, put, web,  Responder};

use crate::services::evento_service;

#[utoipa::path(
    tag = "Evento",
    request_body(content = ArtigoComCategorias, description = "Artigo to store the database", content_type = "multipart/form-data"),
)]
#[post("/evento")]
async fn post_evento(payload: Multipart) -> impl Responder {
    match evento_service::insert(payload).await {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}
#[utoipa::path(
    tag = "Evento",
    request_body(content = ArtigoComCategorias, description = "Artigo to store the database", content_type = "multipart/form-data"),
)]
#[put("/evento/{id}")]
async fn put_evento(id: web::Path<i32>, payload: Multipart) -> impl Responder {
    match evento_service::update(id.into_inner(), payload).await {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Evento",
)]
#[get("/evento/{id}/imagem")]
async fn get_evento_icone(id: web::Path<i32>) -> impl Responder {
    match evento_service::find_icone(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Evento",
)]
#[get("/evento/{id}")]
async fn get_evento_by_id(id: web::Path<i32>) -> impl Responder {
    match evento_service::find_by_id(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Evento",
)]
#[get("/evento")]
async fn get_eventos() -> impl Responder {
    match evento_service::find_all() {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}
