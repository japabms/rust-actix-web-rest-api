use actix_web::{get, post, put, Responder, web, http::StatusCode, HttpResponse, ResponseError};

use crate::{
    models::atividade::*,
    db::{Pool, establish_connection}
};

#[get("/atividade")]
async fn get_atividades() -> impl Responder {
    let pool = establish_connection();
    
    let mut atividades = Atividade::find_all(pool);
    let mut atividades_formatada: Vec<AtividadeDtoDataFormatada> = Vec::new();

    for atividade in atividades.iter_mut() {
        let atividade_formatada = AtividadeDtoDataFormatada {
            id: atividade.id,
            titulo: atividade.titulo.clone(),
            descricao: atividade.descricao.clone(),
            responsavel: atividade.responsavel.clone(),
            inicio: atividade.inicio.format("%d-%m-%Y %H:%M:%S").to_string(),
            fim: atividade.fim.format("%d-%m-%Y %H:%M:%S").to_string(),
        };

        atividades_formatada.push(atividade_formatada);
    }
    
    HttpResponse::Ok().json(atividades_formatada)
}

#[get("/atividade/{id}")]
async fn get_atividade_by_id(id: web::Path<i32>) -> impl Responder {
    let pool = establish_connection();

    let atividade = Atividade::find_by_id(id.into_inner(), pool);

    let atividade_formatada = AtividadeDtoDataFormatada {
            id: atividade.id,
            titulo: atividade.titulo.clone(),
            descricao: atividade.descricao.clone(),
            responsavel: atividade.responsavel.clone(),
            inicio: atividade.inicio.format("%d-%m-%Y %H:%M:%S").to_string(),
            fim: atividade.fim.format("%d-%m-%Y %H:%M:%S").to_string(),
    };

    HttpResponse::Ok().json(atividade_formatada)
}

#[post("/atividade")]
async fn post_atividade(json: web::Json<AtividadeDTO>) -> impl Responder{
    let pool = establish_connection();
    
    let id = Atividade::insert(json.into_inner(), pool);

    HttpResponse::Ok().message_body(format!("Atividade inserida, seu id é {}", id.unwrap()))
}

#[put("/atividade/{id}")]
async fn put_atividade(id: web::Path<i32>, json: web::Json<AtividadeDTO>) -> impl Responder {
    let pool = establish_connection();

    let atividade_atualizada = Atividade::update(id.into_inner(), json.into_inner(), pool).unwrap();

    HttpResponse::Ok().json(atividade_atualizada)  
}
