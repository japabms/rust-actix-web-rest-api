use std::ops::DerefMut;

use actix_multipart::Multipart;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::IntoSql;

use crate::{db::DbPool, models::artigo::ArtigoInput, services::artigo_service};

#[utoipa::path(tag = "Artigo")]
#[get("/artigo")]
async fn get_artigos(pool: web::Data<DbPool>) -> impl Responder {
    match artigo_service::find_all(pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(tag = "Artigo")]
#[get("/artigo/{id}")]
async fn get_artigo_by_id(id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match artigo_service::find_by_id(id.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Artigo",
    request_body(content = ArtigoInput, description = "Artigo to store the database", content_type = "multipart/form-data"),
    responses (
        (status = 200, description = "Artigo postado com sucesso.",),
        (status = NOT_FOUND)
    ),
)]
#[post("/artigo")]
async fn post_artigo(payload: Multipart, pool: web::Data<DbPool>) -> impl Responder {
    match artigo_service::insert(payload, pool.get().unwrap().deref_mut()).await {
        Ok(()) => HttpResponse::Ok().body("Oi"),
        Err(res) => res.into(),
    }
}

#[utoipa::path(tag = "Artigo")]
#[get("/artigo/{id}/documento")]
async fn get_artigo_documento(id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match artigo_service::find_artigo_documento(id.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(tag = "Artigo")]
#[delete("/artigo/{id}")]
async fn delete_artigo(id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match artigo_service::delete(id.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

pub fn init_artigo_routes(config: &mut web::ServiceConfig) {
    config.service(post_artigo)
        .service(get_artigos)
        .service(get_artigo_documento)
        .service(get_artigo_by_id)
        .service(delete_artigo);
}
