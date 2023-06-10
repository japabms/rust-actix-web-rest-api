use std::ops::DerefMut;

use actix_web::{get, post, web, Responder};

use crate::{db::DbPool, models::inscrito::*, services::inscrito_service};

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
async fn post_inscrito(
    json: web::Json<InscritoWithCursosDTO>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    match inscrito_service::insert(json.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

pub fn init_inscrito_routes(config: &mut web::ServiceConfig) {
    config.service(get_inscritos)
        .service(get_inscrito_by_id)
        .service(get_inscrito_cursos)
        .service(post_inscrito);
}
