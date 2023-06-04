use actix_web::{delete, get, http::StatusCode, post, put, web, HttpResponse, Responder};

use crate::{
    db::establish_connection,
    models::{curso::*, inscrito::*, inscrito_cursos::*},
};

#[get("/curso")]
async fn get_cursos() -> impl Responder {
    let conn = establish_connection();

    let cursos = Curso::find_all(conn);

    match cursos {
        Ok(cursos) => HttpResponse::Ok().json(cursos),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/curso")]
async fn get_curso_by_id(id: web::Path<i32>) -> impl Responder {
    let conn = establish_connection();

    let cursos = Curso::find_by_id(id.into_inner(), conn);

    match cursos {
        Ok(cursos) => HttpResponse::Ok().json(cursos),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("/curso")]
async fn post_curso(json: web::Json<CursoDTO>) -> impl Responder {
    let conn = establish_connection();

    let id = Curso::insert(json.into_inner(), conn);

    HttpResponse::Ok().message_body(format!("Curso ID: {}", id))
}

#[put("/curso/{id}")]
async fn put_curso(id: web::Path<i32>, updated_curso: web::Json<CursoDTO>) -> impl Responder {
    let conn = establish_connection();

    let curso = Curso::update(id.into_inner(), updated_curso.into_inner(), conn);

    match curso {
        Ok(curso) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[delete("/curso/{id}")]
async fn delete_curso(id: web::Path<i32>) -> impl Responder {
    let conn = establish_connection();

    let curso = Curso::delete(id.into_inner(), conn);

    match curso {
        Ok(curso) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
