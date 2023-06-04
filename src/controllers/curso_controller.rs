use actix_web::{delete, get, post, put, web,  Responder};

use crate::{
    models::curso::*, services::curso_service, 
};

#[get("/curso")]
async fn get_cursos() -> impl Responder {
    match curso_service::find_all() {
        Ok(res) => res,
        Err(err) => err.into()
    }
}

#[get("/curso/{id}")]
async fn get_curso_by_id(id: web::Path<i32>) -> impl Responder {
    match curso_service::find_by_id(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into()
    }
}

#[post("/curso")]
async fn post_curso(json: web::Json<CursoDTO>) -> impl Responder {
    match curso_service::insert(json.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into()
    }
}

#[put("/curso/{id}")]
async fn put_curso(id: web::Path<i32>, updated_curso: web::Json<CursoDTO>) -> impl Responder {
    match curso_service::update(id.into_inner(), updated_curso.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into()
    }
}

#[delete("/curso/{id}")]
async fn delete_curso(id: web::Path<i32>) -> impl Responder {
    match curso_service::delete(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into()
    }
}
