use actix_web::{
    delete, get,  post, web, HttpResponse, Responder, error::{self, ErrorInternalServerError, Error, ErrorNotFound}
};

use crate::{db::establish_connection, models::categoria::*};

pub fn find_all() -> Result<HttpResponse, Error> {
    let conn = establish_connection();

    match Categoria::find_all(conn){
        Ok(categorias) => Ok(HttpResponse::Ok().json(categorias)),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}

pub fn insert(categoria: NewCategoria) -> Result<HttpResponse, Error>  {
    let conn = establish_connection();

    match Categoria::insert(categoria, conn) {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(err) => Err(ErrorInternalServerError(err))
    }
}

pub fn delete(id: i32) -> Result<HttpResponse, Error> {
    let conn = establish_connection();

    match Categoria::delete(id, conn) {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(err) => Err(ErrorNotFound(format!("Não foi encontrado nenhuma categoria com o id: {}\n{}", id, err)))
    }
}
