use actix_web::{
    delete, get, http::StatusCode, post, put, web, HttpResponse, Responder, ResponseError,
};

use crate::{db::establish_connection, models::categoria::*};

#[get("/categoria")]
async fn get_categoria() -> impl Responder {
    let conn = establish_connection();

    let categorias = Categoria::find_all(conn);

    match categorias {
        Ok(categorias) => HttpResponse::Ok().json(categorias),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/categoria")]
async fn post_categoria(json: web::Json<NewCategoria>) -> impl Responder {
    let conn = establish_connection();

    match Categoria::insert(json.into_inner(), conn) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[delete("/categoria/{id}")]
async fn delete_categoria(id: web::Path<i32>) -> impl Responder {
    let conn = establish_connection();

    match Categoria::delete(id.into_inner(), conn) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
