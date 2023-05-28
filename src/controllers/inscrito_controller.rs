use actix_web::{get, post, Responder, web, http::StatusCode, HttpResponse};

use crate::{
    models::{
        curso::*,
        inscrito::*,
        inscrito_cursos::*
    },
    db::{Pool, establish_connection}
};

#[get("/inscritos")]
async fn get_inscritos() ->  impl Responder {
    let pool = establish_connection();

    let inscritos = Inscrito::find_all(&pool);

    match inscritos {
        Ok(inscrito) => HttpResponse::Ok().json(inscrito),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/inscrito/{id}")]
async fn get_inscrito_by_id(id: web::Path<i32>) -> impl Responder {
    let pool = establish_connection();

    let inscrito = Inscrito::find_by_id(id.into_inner(), &pool);
    
    match inscrito {
        Ok(inscrito) => HttpResponse::Ok().json(inscrito),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/inscrito/{id}/cursos")]
async fn get_inscrito_cursos(id: web::Path<i32>) -> impl Responder {
    let pool = establish_connection();

    let cursos = InscritoCurso::find_inscrito_cursos(id.into_inner(), &pool);

    match cursos {
        Ok(cursos) => HttpResponse::Ok().json(cursos),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("/inscrever")]
async fn post_inscrito(json: web::Json<InscritoWithCursosDTO>) -> impl Responder {
    let pool = establish_connection();

    let id = Inscrito::insert(json.into_inner(), &pool);

    HttpResponse::Ok().message_body(format!("Inscrito ID: {}", id))
}  
