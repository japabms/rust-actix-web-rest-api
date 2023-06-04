use actix_multipart::Multipart;
use actix_web::{delete, get, post, put, web, Responder};

use crate::services::noticia_service;

#[utoipa::path(
    tag = "Noticia",
    request_body(content = NewNoticia, description = "Noticia do store in the database",content_type = "multipart/form-data")
)]
#[post("/noticia")]
async fn post_noticia(payload: Multipart) -> impl Responder {
    match noticia_service::insert(payload).await {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Noticia",
    request_body(content = NewNoticia, content_type = "multipart/form-data")
)]
#[put("/noticia/{id}")]
async fn put_noticia(id: web::Path<i32>, payload: Multipart) -> impl Responder {
    match noticia_service::update(id.into_inner(), payload).await {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Noticia",
)]
#[get("/noticia")]
async fn get_noticias() -> impl Responder {
    match noticia_service::find_all() {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Noticia",
)]
#[get("/noticia/{id}")]
async fn get_noticia_by_id(id: web::Path<i32>) -> impl Responder {
    match noticia_service::find_by_id(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Noticia",
)]
#[get("/noticia/{id}/imagem")]
async fn get_noticia_imagem(id: web::Path<i32>) -> impl Responder {
    match noticia_service::find_imagem(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Noticia",
)]
#[get("/noticia/recentes")]
async fn get_noticias_recentes() -> impl Responder {
    match noticia_service::find_noticia_recente() {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Noticia",
)]
#[delete("/noticia/{id}")]
async fn delete_noticia(id: web::Path<i32>) -> impl Responder {
    match noticia_service::delete(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}
