use actix_web::{
    delete, get, post, put, web, HttpResponse, Responder, 
};

use crate::{db::establish_connection, models::atividade::*};

#[get("/atividade")]
async fn get_atividades() -> impl Responder {
    let conn = establish_connection();

    let mut atividades =  match Atividade::find_all(conn) {
        Ok(ativ) => ativ,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    
    let mut atividades_formatada: Vec<AtividadeDtoDataFormatada> = Vec::new();

    for atividade in atividades.iter_mut() {
        let atividade_formatada = AtividadeDtoDataFormatada {
            id: atividade.id,
            titulo: atividade.titulo.clone(),
            descricao: atividade.descricao.clone(),
            responsavel: atividade.responsavel.clone(),
            inicio: atividade.inicio.format("%Y-%m-%d %H:%M:%S").to_string(),
            fim: atividade.fim.format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        atividades_formatada.push(atividade_formatada);
    }

    HttpResponse::Ok().json(atividades_formatada)
}

#[get("/atividade/{id}")]
async fn get_atividade_by_id(id: web::Path<i32>) -> impl Responder {
    let pool = establish_connection();

    let atividade = match Atividade::find_by_id(id.into_inner(), pool) {
        Ok(ativ) => ativ,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let atividade_formatada = AtividadeDtoDataFormatada {
        id: atividade.id,
        titulo: atividade.titulo.clone(),
        descricao: atividade.descricao.clone(),
        responsavel: atividade.responsavel.clone(),
        inicio: atividade.inicio.format("%Y-%m-%d %H:%M:%S").to_string(),
        fim: atividade.fim.format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    HttpResponse::Ok().json(atividade_formatada)
}

#[post("/atividade")]
async fn post_atividade(json: web::Json<AtividadeDTO>) -> impl Responder {
    let pool = establish_connection();

    let id = Atividade::insert(json.into_inner(), pool);

    HttpResponse::Ok().message_body(format!("Atividade inserida, seu id é {}", id.unwrap()))
}

#[put("/atividade/{id}")]
async fn put_atividade(id: web::Path<i32>, json: web::Json<AtividadeDTO>) -> impl Responder {
    let pool = establish_connection();

    match Atividade::update(id.into_inner(), json.into_inner(), pool) {
        Ok(num) => HttpResponse::Ok().body(format!("Numero de colunas atualizadas {}", num)), 
        Err(_) => HttpResponse::BadRequest().body("A data deve estar no seguinte formato: yyyy-MM-dd"),
    }
}

#[delete("/atividade/{id}")]
async fn delete_atividade(id: web::Path<i32>) -> impl Responder {
    let conn = establish_connection();

    match Atividade::delete(id.into_inner(), conn) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
