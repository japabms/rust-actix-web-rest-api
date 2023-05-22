use actix_web::{get, post, Responder, web, http::StatusCode, HttpResponse};

use crate::{
    models::{
        curso::*,
        inscrito::*,
        inscrito_cursos::*
    },
    db::{Pool, establish_connection}
};

#[get("/inscrito")]
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
    
    let inscrito = InscritoWithCursosDTO {
        nome: json.nome.clone(),
        nome_cracha: json.nome_cracha.clone(),
        email: json.email.clone(),
        cpf: json.cpf.clone(),
        modalidade_nome: json.modalidade_nome.clone(),
        modalidade_preco: json.modalidade_preco,
        instituicao: json.instituicao.clone(),
        cursos: json.cursos.clone(),
    };

    let id = Inscrito::insert(inscrito, &pool);

    HttpResponse::Ok().message_body(format!("Inscrito ID: {}", id))
}
