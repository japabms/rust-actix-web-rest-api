use actix_multipart::Multipart;
use actix_web::error::ErrorBadRequest;
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::{get, http::StatusCode, post, put, web, HttpResponse, Responder, ResponseError};
use chrono::NaiveDate;
use diesel::PgConnection;
use futures_util::StreamExt as _;
use futures_util::TryStreamExt as _;
use std::io::Write;
use std::str::FromStr;

use crate::{db::establish_connection, models::evento::*};

#[post("/evento")]
async fn post_evento(mut payload: Multipart, req: HttpRequest) -> Result<HttpResponse, Error> {
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
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => panic!("F"),
    }
}

#[put("/evento/{id}")]
async fn put_evento(id: web::Path<i32>, mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut evento = NewEvento::default();
    let conn = establish_connection();
    let conn_2 = establish_connection();

    let _evento = Evento::find_by_id(*id, conn_2).unwrap();

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

    match Evento::update(id.into_inner(), evento, conn) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(ErrorBadRequest("Something is wrong! kekw")),
    }
}

#[get("/evento/{id}/imagem")]
async fn get_evento_icone(id: web::Path<i32>) -> impl Responder {
    let conn = establish_connection();

    let imagem = Evento::find_icone(id.into_inner(), conn).unwrap();

    HttpResponse::Ok().content_type("image/png").body(imagem)
}

#[get("/evento/{id}")]
async fn get_evento_by_id(id: web::Path<i32>) -> impl Responder {
    let conn = establish_connection();

    let evento = Evento::find_by_id(id.into_inner(), conn).unwrap();

    let evento_formatado = EventoDtoDataFormatada {
        id: evento.id,
        titulo: evento.titulo.clone(),
        sobre: evento.sobre.clone(),
        tipo: evento.tipo.clone(),
        email: evento.email.clone(),
        data_inicio: evento.data_inicio.format("%d-%m-%Y").to_string(),
        data_fim: evento.data_fim.format("%d-%m-%Y").to_string(),
    };

    HttpResponse::Ok().json(evento_formatado)
}

#[get("/evento")]
async fn get_eventos() -> impl Responder {
    let conn = establish_connection();

    let eventos = Evento::find_all(conn).unwrap();
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

    HttpResponse::Ok().json(eventos_formatado)
}
