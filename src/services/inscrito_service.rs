use actix_web::{error::{Error,ErrorBadRequest,  ErrorNotFound}, HttpResponse};

use diesel::PgConnection;
use crate::{
    models::{
        inscrito::*,
        inscrito_cursos::*
    },
    db::establish_connection
};

pub fn find_all(conn:  &mut PgConnection) -> Result<HttpResponse, Error> {

    match Inscrito::find_all(conn) {
        Ok(inscrito) => Ok(HttpResponse::Ok().json(inscrito)),
        Err(err) => Err(ErrorBadRequest(format!(
            "Não foi possivel completar a sua requisição\n {}",
            err
        ))),
    }
}

pub fn find_by_id(id: i32, conn:  &mut PgConnection) -> Result<HttpResponse, Error> {
    
    match Inscrito::find_by_id(id, conn) {
        Ok(inscrito) => Ok(HttpResponse::Ok().json(inscrito)),
        Err(_) => Err(ErrorNotFound(format!("Nenhum inscrito com o id {} foi encontrado", id)))
    }
}

pub fn find_inscrito_cursos(id: i32, conn:  &mut PgConnection) -> Result<HttpResponse, Error> {

    match InscritoCurso::find_inscrito_cursos(id, conn) {
        Ok(cursos) => Ok(HttpResponse::Ok().json(cursos)),
        Err(_) => Err(ErrorNotFound(format!("O inscrito com id {} não tem nenhum curso.", id)))
    }
}

pub fn insert(inscrito: InscritoWithCursosDTO, conn:  &mut PgConnection) -> Result<HttpResponse, Error> {

    match Inscrito::insert(inscrito, conn) {
        Ok(num) => Ok(HttpResponse::Ok().body(format!("Inscrito ID: {}", num))),
        Err(err) => Err(ErrorBadRequest(format!(
            "Não foi possivel completar a sua requisição\n {}",
            err
        ))),
    }
}
