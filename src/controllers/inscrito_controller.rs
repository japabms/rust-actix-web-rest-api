use actix_web::{ get, post, web, Responder};

use crate::{models::inscrito::*, services::inscrito_service};

#[get("/inscrito")]
async fn get_inscritos() -> impl Responder {
    match inscrito_service::find_all() {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[get("/inscrito/{id}")]
async fn get_inscrito_by_id(id: web::Path<i32>) -> impl Responder {
    match inscrito_service::find_by_id(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[get("/inscrito/{id}/cursos")]
async fn get_inscrito_cursos(id: web::Path<i32>) -> impl Responder {
    match inscrito_service::find_inscrito_cursos(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[post("/inscrever")]
async fn post_inscrito(json: web::Json<InscritoWithCursosDTO>) -> impl Responder {
    match inscrito_service::insert(json.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}
