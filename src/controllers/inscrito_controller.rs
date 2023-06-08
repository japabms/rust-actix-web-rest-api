use std::ops::DerefMut;

use actix_web::{get, post, web, Responder};

use crate::{models::inscrito::*, services::inscrito_service, db::DbPool};

#[utoipa::path(tag = "Inscrito")]
#[get("/inscrito")]
async fn get_inscritos(pool: web::Data<DbPool>) -> impl Responder {
    match inscrito_service::find_all(pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(tag = "Inscrito")]
#[get("/inscrito/{id}")]
async fn get_inscrito_by_id(id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match inscrito_service::find_by_id(id.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(tag = "Inscrito")]
#[get("/inscrito/{id}/cursos")]
async fn get_inscrito_cursos(id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match inscrito_service::find_inscrito_cursos(id.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Inscrito",
    request_body = InscritoWithCursosDTO
)]
#[post("/inscrever")]
async fn post_inscrito(json: web::Json<InscritoWithCursosDTO>, pool: web::Data<DbPool>) -> impl Responder {
    match inscrito_service::insert(json.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}
