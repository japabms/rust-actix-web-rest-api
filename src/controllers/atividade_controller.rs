use std::ops::DerefMut;

use actix_web::{delete, get, post, put, web, Responder};

use crate::{db::DbPool, models::atividade::*, services::atividade_service};

#[utoipa::path(tag = "Atividade")]
#[get("/atividade")]
async fn get_atividades(pool: web::Data<DbPool>) -> impl Responder {
    match atividade_service::find_all(pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(tag = "Atividade")]
#[get("/atividade/{id}")]
async fn get_atividade_by_id(id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match atividade_service::find_by_id(id.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Atividade",
    request_body = NewAtividade,
)]
#[post("/atividade")]
async fn post_atividade(json: web::Json<NewAtividade>, pool: web::Data<DbPool>) -> impl Responder {
    match atividade_service::insert(json.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Atividade",
    request_body = NewAtividade,
)]
#[put("/atividade/{id}")]
async fn put_atividade(
    id: web::Path<i32>,
    json: web::Json<NewAtividade>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    match atividade_service::update(
        id.into_inner(),
        json.into_inner(),
        pool.get().unwrap().deref_mut(),
    ) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(tag = "Atividade")]
#[delete("/atividade/{id}")]
async fn delete_atividade(id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match atividade_service::delete(id.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

pub fn init_atividade_routes(config: &mut web::ServiceConfig) {
    config
        .service(get_atividades)
        .service(get_atividade_by_id)
        .service(post_atividade)
        .service(put_atividade)
        .service(delete_atividade);
}
