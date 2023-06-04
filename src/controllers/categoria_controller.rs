use actix_web::{delete, get, post, web, Responder};

use crate::models::categoria::NewCategoria;
use crate::services::categoria_service;

#[utoipa::path(
    tag = "Categoria",
    responses (
        (status = 200, description = "Categorias pega com sucesso.", body = Vec<Categoria>),
        (status = NOT_FOUND)
    ),
)]
#[get("/categoria")]
async fn get_categoria() -> impl Responder {
    match categoria_service::find_all() {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Categoria",
    request_body = NewCategoria,
    responses (
        (status = 200, description = "Categoria inserida com sucesso."),
        (status = NOT_FOUND)
    ),
)]
#[post("/categoria")]
async fn post_categoria(json: web::Json<NewCategoria>) -> impl Responder {
    match categoria_service::insert(json.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

#[utoipa::path(
    tag = "Categoria",
    responses (
        (status = 200, description = "Categoria apagada com sucesso."),
        (status = NOT_FOUND)
    ),
)]
#[delete("/categoria/{id}")]
async fn delete_categoria(id: web::Path<i32>) -> impl Responder {
    match categoria_service::delete(id.into_inner()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}
