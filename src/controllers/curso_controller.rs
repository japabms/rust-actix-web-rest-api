use actix_web::{get, post,put,  Responder, web, http::StatusCode, HttpResponse};

use crate::{
    models::{
        curso::*,
        inscrito::*,
        inscrito_cursos::*
    },
    db::{Pool, establish_connection}
};

#[get("/cursos")]
async fn get_cursos() -> impl Responder {
    let pool = establish_connection();

    let cursos = Curso::find_all(pool);

    match cursos {
        Ok(cursos) => HttpResponse::Ok().json(cursos),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("/")]
async fn post_curso(json: web::Json<CursoDTO>) -> impl Responder {
    let pool = establish_connection();

    let id = Curso::insert(json.into_inner(), pool);

    HttpResponse::Ok().message_body(format!("Curso ID: {}", id ))
}

#[put("/curso/{id}")]
async fn put_curso(id: web::Path<i32>, updated_curso: web::Json<CursoDTO>) -> impl Responder {
    let pool = establish_connection();

    let curso =  Curso::update(id.into_inner(), updated_curso.into_inner() , pool);

    match curso{
        Ok(curso) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
