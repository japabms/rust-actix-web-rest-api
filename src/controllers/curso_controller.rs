use utoipa;
use actix_web::{delete, get, post, put, web,  Responder};

use crate::{
    models::curso::*, services::curso_service, 
};

#[utoipa::path(
    tag = "Curso",
    responses (
        (status = 200, description = "Cursos pego com sucesso.", body = Vec<Curso>),
        (status = NOT_FOUND)
    ),
)]
#[get("/curso")]
async fn get_cursos() -> impl Responder {
    match curso_service::find_all() {
        Ok(res) => res,
        Err(err) => err.into()
    }
}

#[utoipa::path(
    tag = "Curso",
    responses (
        (status = 200, description = "Cursos pego com sucesso.", body = Curso),
        (status = NOT_FOUND)
    ),
)]
#[get("/curso/{id}")]
async fn get_curso_by_id(id: web::Path<i32>) -> impl Responder {
    match curso_service::find_by_id(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into()
    }
}

#[utoipa::path(
    tag = "Curso",
    request_body = CursoDTO,
    responses (
        (status = 200, description = "Cursos pego com sucesso.",),
        (status = NOT_FOUND)
    ),
)]
#[post("/curso")]
async fn post_curso(json: web::Json<CursoDTO>) -> impl Responder {
    match curso_service::insert(json.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into()
    }
}

#[utoipa::path(
    tag = "Curso",
    request_body = CursoDTO,
    responses (
        (status = 200, description = "Cursos pego com sucesso."),
        (status = NOT_FOUND)
    ),
)]
#[put("/curso/{id}")]
async fn put_curso(id: web::Path<i32>, updated_curso: web::Json<CursoDTO>) -> impl Responder {
    match curso_service::update(id.into_inner(), updated_curso.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into()
    }
}

#[utoipa::path(
    tag = "Curso",
    responses (
        (status = 200, description = "Cursos pego com sucesso.",),
        (status = NOT_FOUND)
    ),
)]
#[delete("/curso/{id}")]
async fn delete_curso(id: web::Path<i32>) -> impl Responder {
    match curso_service::delete(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into()
    }
}
