use std::io::Write;
use std::str::FromStr;
use actix_multipart::Multipart;
use actix_web::Error;
use actix_web::HttpRequest;
use chrono::NaiveDate;
use futures_util::StreamExt as _;
use futures_util::TryStreamExt as _;
use actix_web::{get, post, put, Responder, web, http::StatusCode, HttpResponse, ResponseError};


use crate::models::evento;
use crate::{
    models::evento::*,
    db::{Pool, establish_connection}
};



#[post("/evento")]
async fn post_evento(mut payload: Multipart, req: HttpRequest) -> Result<HttpResponse, Error>{

    let mut evento = EventoDTO::default(); 
    let pool = establish_connection(); 

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
                    evento.tipo = email;
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
    
    match Evento::insert(evento, pool) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => panic!("F"), 
    }
}

#[put("/evento/{id}")]
async fn put_evento(id: web::Path<i32>, json: web::Json<EventoDtoMinimal>) -> impl Responder{
    let pool = establish_connection();

    match Evento::update(id.into_inner(), json.into_inner(), pool) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/evento/{id}/icone")]
async fn get_evento_icone(id: web::Path<i32>) -> impl Responder {
    let pool = establish_connection();

    let imagem = Evento::find_icone(id.into_inner(), pool).unwrap();

    HttpResponse::Ok().content_type("image/png").body(imagem)
}

#[get("/evento/{id}")]
async fn get_evento_by_id(id: web::Path<i32>) -> impl Responder {
    let pool = establish_connection();

    let evento = Evento::find_by_id(id.into_inner(), pool);

    match evento {
        Ok(evento) => HttpResponse::Ok().json(evento),
        Err(_) => HttpResponse::NotFound().finish()
    }
}
