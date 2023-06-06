use actix_web::{
    error::{Error, ErrorBadRequest, ErrorInternalServerError, ErrorNotFound},
    HttpResponse,
};

use crate::{db::establish_connection, models::categoria::*};

pub fn find_all() -> Result<HttpResponse, Error> {
    let conn = establish_connection();

    match Categoria::find_all(conn) {
        Ok(categorias) => Ok(HttpResponse::Ok().json(categorias)),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}

pub fn insert(categoria: NewCategoria) -> Result<HttpResponse, Error> {
    let conn = establish_connection();

    match Categoria::insert(categoria, conn) {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}

pub fn delete(id: i32) -> Result<HttpResponse, Error> {
    let conn = establish_connection();

    match Categoria::delete(id, conn) {
        Ok(i) => {
            if i == 0 {
                Err(ErrorNotFound(format!(
                    "Não foi encontrado nenhuma categoria com o id {}",
                    id
                )))
            } else {
                Ok(HttpResponse::Ok().finish())
            }
        }
        Err(err) => Err(ErrorBadRequest(err)),
    }
}
