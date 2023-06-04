use actix_web::{
    error::{Error, ErrorBadRequest, ErrorInternalServerError, ErrorNotFound},
    HttpResponse,
};

use crate::{db::establish_connection, models::curso::*};

pub fn find_all() -> Result<HttpResponse, Error> {
    let conn = establish_connection();

    match Curso::find_all(conn) {
        Ok(cursos) => Ok(HttpResponse::Ok().json(cursos)),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}

pub fn find_by_id(id: i32) -> Result<HttpResponse, Error> {
    let conn = establish_connection();

    match Curso::find_by_id(id, conn) {
        Ok(curso) => Ok(HttpResponse::Ok().json(curso)),
        Err(err) => Err(ErrorNotFound(err)),
    }
}

pub fn insert(curso: CursoDTO) -> Result<HttpResponse, Error> {
    let conn = establish_connection();

    match Curso::insert(curso, conn) {
        Ok(num) => Ok(HttpResponse::NoContent().finish()),
        Err(err) => Err(ErrorBadRequest(format!(
            "Não foi possivel completar a sua requisição\n {}",
            err
        ))),
    }
}

pub fn update(id: i32, curso: CursoDTO) -> Result<HttpResponse, Error> {
    let conn = establish_connection();

    match Curso::update(id, curso, conn) {
        Ok(num) => Ok(HttpResponse::Ok().body(format!("Número de colunas atualizada: {}", num))),
        Err(err) => {
            if err.to_string().eq("Record not found") {
                Err(ErrorNotFound(format!(
                    "Não foi encontrado nenhum curso com o id {}",
                    id
                )))
            } else {
                Err(ErrorBadRequest(format!(
                    "Não foi possivel completar a sua requisição\n {}",
                    err
                )))
            }
        }
    }
}

pub fn delete(id: i32) -> Result<HttpResponse, Error> {
    let conn = establish_connection();

    match Curso::delete(id, conn) {
        Ok(curso) => {
            Ok(HttpResponse::Ok().body(format!("Número de colunas atualizada: {}", curso)))
        }
        Err(err) => {
            if err.to_string().eq("Record not found") {
                Err(ErrorNotFound(format!(
                    "Não foi encontrado nenhum curso com o id {}",
                    id
                )))
            } else {
                Err(ErrorBadRequest(format!(
                    "Não foi possivel completar a sua requisição\n {}",
                    err
                )))
            }
        }
    }
}
