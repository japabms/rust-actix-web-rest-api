use actix_multipart::Multipart;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::Error;
use actix_web::HttpResponse;
use chrono::NaiveDate;
use futures_util::StreamExt as _;
use futures_util::TryStreamExt as _;
use std::io::Write;

use crate::{db::establish_connection, models::evento::*};

pub fn find_all() -> Result<HttpResponse, Error> {
    let conn = establish_connection();

    let eventos = match Evento::find_all(conn) {
        Ok(eventos) => eventos,
        Err(err) => return Err(ErrorInternalServerError(err)),
    };

    let mut eventos_formatado: Vec<EventoDtoDataFormatada> = Vec::new();

    for evento in eventos.iter() {
        let evento_formatado = EventoDtoDataFormatada {
            id: evento.id,
            titulo: evento.titulo.clone(),
            sobre: evento.sobre.clone(),
            tipo: evento.tipo.clone(),
            email: evento.email.clone(),
            data_inicio: evento.data_inicio.format("%d-%m-%Y").to_string(),
            data_fim: evento.data_fim.format("%d-%m-%Y").to_string(),
        };
        eventos_formatado.push(evento_formatado);
    }

    Ok(HttpResponse::Ok().json(eventos_formatado))
}

pub fn find_by_id(id: i32) -> Result<HttpResponse, Error> {
    let conn = establish_connection();

    let evento = match Evento::find_by_id(id, conn) {
        Ok(evento) => evento,
        Err(err) => return Err(ErrorNotFound(err)),
    };

    let evento_formatado = EventoDtoDataFormatada {
        id: evento.id,
        titulo: evento.titulo.clone(),
        sobre: evento.sobre.clone(),
        tipo: evento.tipo.clone(),
        email: evento.email.clone(),
        data_inicio: evento.data_inicio.format("%d-%m-%Y").to_string(),
        data_fim: evento.data_fim.format("%d-%m-%Y").to_string(),
    };

    Ok(HttpResponse::Ok().json(evento_formatado))
}

pub fn find_icone(id: i32) -> Result<HttpResponse, Error> {
    let conn = establish_connection();

    match Evento::find_icone(id, conn) {
        Ok(img) => Ok(HttpResponse::Ok().content_type("image/png").body(img)),
        Err(err) => Err(ErrorNotFound(format!("Não foi encontrado nenhuma imagem no evento com id {}\n{}", id, err)))
    }
}
pub async fn insert(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut evento = NewEvento::default();
    let conn = establish_connection();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();

        if let Some(nome) = content_disposition.get_name() {
            match nome {
                "titulo" => {
                    let mut titulo = String::new();
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        titulo.push_str(std::str::from_utf8(&data)?);
                    }
                    evento.titulo = titulo;
                }
                "sobre" => {
                    let mut sobre = String::new();
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        sobre.push_str(std::str::from_utf8(&data)?);
                    }
                    evento.sobre = sobre;
                }
                "data_inicio" => {
                    let mut date_str = String::new();
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        date_str.push_str(std::str::from_utf8(&data)?)
                    }
                    let date = NaiveDate::parse_from_str(&date_str, "%d-%m-%Y").unwrap();
                    evento.data_inicio = date;
                }
                "data_fim" => {
                    let mut date_str = String::new();
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        date_str.push_str(std::str::from_utf8(&data)?)
                    }
                    let date = NaiveDate::parse_from_str(&date_str, "%d-%m-%Y").unwrap();
                    evento.data_fim = date;
                }
                "tipo" => {
                    let mut tipo = String::new();
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        tipo.push_str(std::str::from_utf8(&data)?)
                    }
                    evento.tipo = tipo;
                }
                "email" => {
                    let mut email = String::new();
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        email.push_str(std::str::from_utf8(&data)?)
                    }
                    evento.email = email;
                }
                "icone" => {
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        evento.icone.write_all(&data)?;
                    }
                }
                _ => {}
            }
        }
    }

    match Evento::insert(evento, conn) {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(err) => Err(ErrorInternalServerError(err))
    }
}

pub async fn update(id: i32, mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut evento = NewEvento::default();
    let conn = establish_connection();
    let conn_2 = establish_connection();

    let _evento = Evento::find_by_id(id, conn_2).unwrap();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();

        if let Some(nome) = content_disposition.get_name() {
            match nome {
                "titulo" => {
                    let mut titulo = String::new();
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        titulo.push_str(std::str::from_utf8(&data)?);
                    }
                    evento.titulo = titulo;
                }
                "sobre" => {
                    let mut sobre = String::new();
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        sobre.push_str(std::str::from_utf8(&data)?);
                    }
                    evento.sobre = sobre;
                }
                "data_inicio" => {
                    let mut date_str = String::new();
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        date_str.push_str(std::str::from_utf8(&data)?)
                    }
                    let date = NaiveDate::parse_from_str(&date_str, "%d-%m-%Y").unwrap();
                    evento.data_inicio = date;
                }
                "data_fim" => {
                    let mut date_str = String::new();
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        date_str.push_str(std::str::from_utf8(&data)?)
                    }
                    let date = NaiveDate::parse_from_str(&date_str, "%d-%m-%Y").unwrap();
                    evento.data_fim = date;
                }
                "tipo" => {
                    let mut tipo = String::new();
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        tipo.push_str(std::str::from_utf8(&data)?)
                    }
                    evento.tipo = tipo;
                }
                "email" => {
                    let mut email = String::new();
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        email.push_str(std::str::from_utf8(&data)?)
                    }
                    evento.email = email;
                }
                "icone" => {
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        evento.icone.write_all(&data)?;
                    }
                }
                _ => {}
            }
        }
    }

    if evento.icone.is_empty() {
        evento.icone = _evento.icone;
    }

    match Evento::update(id, evento, conn) {
        Ok(i) => {
            if i == 0 {
                Err(ErrorNotFound(format!(
                    "Não foi encontrado nenhum evento com o id {}",
                    id
                )))
            } else {
                Ok(HttpResponse::Ok().finish())
            }
        }
        Err(err) => Err(ErrorBadRequest(err)),
    }
}
