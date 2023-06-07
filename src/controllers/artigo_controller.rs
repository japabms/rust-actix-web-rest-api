use actix_multipart::Multipart;
use actix_web::{get, post, put, web, HttpResponse, Responder, delete};
use diesel::IntoSql;

use crate::{services::artigo_service, models::artigo::ArtigoComCategorias};

#[utoipa::path(tag = "Artigo")]
#[get("/artigo")]
async fn get_artigos() -> impl Responder {
    match artigo_service::find_all() {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(tag = "Artigo")]
#[get("/artigo/{id}")]
async fn get_artigo_by_id(id: web::Path<i32>) -> impl Responder {
    match artigo_service::find_by_id(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}


#[utoipa::path(
    tag = "Artigo",
    request_body(content = ArtigoComCategorias, description = "Artigo to store the database", content_type = "multipart/form-data"),
    responses (
        (status = 200, description = "Artigo postado com sucesso.",),
        (status = NOT_FOUND)
    ),
)]
#[post("/artigo")]
async fn post_artigo(payload: Multipart) -> impl Responder {
    match artigo_service::insert(payload).await {
        Ok(()) => HttpResponse::Ok().body("Oi"),
        Err(res) => res.into(),
    }
}

#[utoipa::path(tag = "Artigo")]
#[get("/artigo/{id}/documento")]
async fn get_artigo_documento(id: web::Path<i32>) -> impl Responder {
    match artigo_service::find_artigo_documento(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(tag = "Artigo")]
#[delete("/artigo/{id}")]
async fn delete_artigo(id: web::Path<i32>) -> impl Responder {
    match artigo_service::delete(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}
