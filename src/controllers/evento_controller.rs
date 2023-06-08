use std::ops::DerefMut;

use actix_multipart::Multipart;
use actix_web::{get, post, put, web, Responder};

use crate::db::DbPool;
use crate::services::evento_service;

#[utoipa::path(
    tag = "Evento",
    request_body(content = ArtigoComCategorias, description = "Artigo to store the database", content_type = "multipart/form-data"),
)]
#[post("/evento")]
async fn post_evento(payload: Multipart, pool: web::Data<DbPool>) -> impl Responder {
    match evento_service::insert(payload, pool.get().unwrap().deref_mut()).await {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}
#[utoipa::path(
    tag = "Evento",
    request_body(content = ArtigoComCategorias, description = "Artigo to store the database", content_type = "multipart/form-data"),
)]
#[put("/evento/{id}")]
async fn put_evento(
    id: web::Path<i32>,
    payload: Multipart,
    pool: web::Data<DbPool>,
) -> impl Responder {
    match evento_service::update(id.into_inner(), payload, pool.get().unwrap().deref_mut()).await {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(tag = "Evento")]
#[get("/evento/{id}/imagem")]
async fn get_evento_icone(id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match evento_service::find_icone(id.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(tag = "Evento")]
#[get("/evento/{id}")]
async fn get_evento_by_id(id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match evento_service::find_by_id(id.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(tag = "Evento")]
#[get("/evento")]
async fn get_eventos(pool: web::Data<DbPool>) -> impl Responder {
    match evento_service::find_all(pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}
