use std::ops::DerefMut;

use actix_multipart::Multipart;
use actix_web::{delete, get, post, put, web, Responder};

use crate::{services::noticia_service, db::DbPool};

#[utoipa::path(
    tag = "Noticia",
    request_body(content = NewNoticia, description = "Noticia do store in the database",content_type = "multipart/form-data")
)]
#[post("/noticia")]
async fn post_noticia(payload: Multipart, pool: web::Data<DbPool>) -> impl Responder {
    match noticia_service::insert(payload, pool.get().unwrap().deref_mut()).await {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Noticia",
    request_body(content = NewNoticia, content_type = "multipart/form-data")
)]
#[put("/noticia/{id}")]
async fn put_noticia(id: web::Path<i32>, payload: Multipart, pool: web::Data<DbPool>) -> impl Responder {
    match noticia_service::update(id.into_inner(), payload, pool.get().unwrap().deref_mut()).await {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(tag = "Noticia")]
#[get("/noticia")]
async fn get_noticias(pool: web::Data<DbPool>) -> impl Responder {
    match noticia_service::find_all(pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(tag = "Noticia")]
#[get("/noticia/{id}")]
async fn get_noticia_by_id(id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match noticia_service::find_by_id(id.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(tag = "Noticia")]
#[get("/noticia/{id}/imagem")]
async fn get_noticia_imagem(id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match noticia_service::find_imagem(id.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(tag = "Noticia")]
#[get("/noticia/recentes")]
async fn get_noticias_recentes( pool: web::Data<DbPool>) -> impl Responder {
    match noticia_service::find_noticia_recente( pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(tag = "Noticia")]
#[delete("/noticia/{id}")]
async fn delete_noticia(id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match noticia_service::delete(id.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}
