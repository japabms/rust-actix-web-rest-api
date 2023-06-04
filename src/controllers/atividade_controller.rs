use actix_web::{delete, get, post, put, web, Responder};

use crate::{models::atividade::*, services::atividade_service};

#[get("/atividade")]
async fn get_atividades() -> impl Responder {
    match atividade_service::find_all(){
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[get("/atividade/{id}")]
async fn get_atividade_by_id(id: web::Path<i32>) -> impl Responder {
    match atividade_service::find_by_id(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[post("/atividade")]
async fn post_atividade(json: web::Json<AtividadeDTO>) -> impl Responder {
    match atividade_service::insert(json.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[put("/atividade/{id}")]
async fn put_atividade(id: web::Path<i32>, json: web::Json<AtividadeDTO>) -> impl Responder {
    match atividade_service::update(id.into_inner(), json.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[delete("/atividade/{id}")]
async fn delete_atividade(id: web::Path<i32>) -> impl Responder {
    match atividade_service::delete(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}
