use actix_web::{
    error::Error,
    error::{ErrorBadRequest, ErrorNotFound},
    HttpResponse,
};

use crate::models::atividade::*;
use diesel::PgConnection;

pub fn find_all(conn: &mut PgConnection) -> Result<HttpResponse, Error> {
    let mut atividades = match Atividade::find_all(conn) {
        Ok(ativ) => ativ,
        Err(_) => {
            return Err(ErrorBadRequest(
                "Não foi possivel completar a sua requisição",
            ))
        }
    };

    let mut atividades_formatada: Vec<AtivdadeDTO> = Vec::new();

    for atividade in atividades.iter_mut() {
        let atividade_formatada = AtivdadeDTO {
            id: atividade.id,
            titulo: atividade.titulo.clone(),
            descricao: atividade.descricao.clone(),
            responsavel: atividade.responsavel.clone(),
            inicio: atividade.inicio.format("%Y-%m-%d %H:%M:%S").to_string(),
            fim: atividade.fim.format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        atividades_formatada.push(atividade_formatada);
    }

    Ok(HttpResponse::Ok().json(atividades_formatada))
}

pub fn find_by_id(id: i32, conn: &mut PgConnection) -> Result<HttpResponse, Error> {
    let atividade = match Atividade::find_by_id(id, conn) {
        Ok(ativ) => ativ,
        Err(_) => {
            return Err(ErrorNotFound(format!(
                "Não foi encontrado nenhuma atividade com o id: {}",
                id
            )))
        }
    };

    let atividade_formatada = AtivdadeDTO {
        id: atividade.id,
        titulo: atividade.titulo.clone(),
        descricao: atividade.descricao.clone(),
        responsavel: atividade.responsavel.clone(),
        inicio: atividade.inicio.format("%Y-%m-%d %H:%M:%S").to_string(),
        fim: atividade.fim.format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    Ok(HttpResponse::Ok().json(atividade_formatada))
}

pub fn insert(atividade: NewAtividade, conn: &mut PgConnection) -> Result<HttpResponse, Error> {
    match Atividade::insert(atividade, conn) {
        Ok(id) => Ok(HttpResponse::Ok().body(format!("Atividade com o id {} foi inserida", id))),
        Err(err) => Err(ErrorBadRequest(format!(
            "Não foi possivel completar a sua requisição\n {}",
            err
        ))),
    }
}

pub fn update(
    id: i32,
    atividade: NewAtividade,
    conn: &mut PgConnection,
) -> Result<HttpResponse, Error> {
    match Atividade::update(id, atividade, conn) {
        Ok(i) => {
            if i == 0 {
                Err(ErrorNotFound(format!(
                    "Não foi encontrado nenhuma atividade com o id {}",
                    id
                )))
            } else {
                Ok(HttpResponse::Ok().finish())
            }
        }
        Err(err) => Err(ErrorBadRequest(err)),
    }
}

pub fn delete(id: i32, conn: &mut PgConnection) -> Result<HttpResponse, Error> {
    match Atividade::delete(id, conn) {
        Ok(i) => {
            if i == 0 {
                Err(ErrorNotFound(format!(
                    "Não foi encontrado nenhuma atividade com o id {}",
                    id
                )))
            } else {
                Ok(HttpResponse::Ok().finish())
            }
        }
        Err(err) => Err(ErrorBadRequest(err)),
    }
}
