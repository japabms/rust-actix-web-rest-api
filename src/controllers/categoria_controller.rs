use std::ops::DerefMut;

use actix_web::{delete, get, post, web, Responder};

use crate::{db::DbPool, models::categoria::NewCategoria, services::categoria_service};

#[utoipa::path(
    tag = "Categoria",
    responses (
        (status = 200, description = "Categorias pega com sucesso.", body = Vec<Categoria>),
        (status = NOT_FOUND)
    ),
)]
#[get("/categoria")]
async fn get_categorias(pool: web::Data<DbPool>) -> impl Responder {
    match categoria_service::find_all(pool.get().unwrap().deref_mut()) {
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
async fn post_categoria(json: web::Json<NewCategoria>, pool: web::Data<DbPool>) -> impl Responder {
    match categoria_service::insert(json.into_inner(), pool.get().unwrap().deref_mut()) {
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
async fn delete_categoria(id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match categoria_service::delete(id.into_inner(), pool.get().unwrap().deref_mut()) {
        Ok(res) => res,
        Err(err) => err.into(),
    }
}

pub fn init_categoria_routes(config: &mut web::ServiceConfig) {
    config.service(get_categorias)
        .service(post_categoria)
        .service(delete_categoria);
}
