use std::io::Write;
use actix_multipart::Multipart;
use actix_web::Error;
use actix_web::HttpRequest;
use chrono::NaiveDateTime;
use futures_util::StreamExt as _;
use futures_util::TryStreamExt as _;
use actix_web::{get, post, put, Responder, web, http::StatusCode, HttpResponse, ResponseError};


use crate::{
    models::noticia::*,
    db::establish_connection,
};

#[post("/noticia")]
async fn post_noticia(mut payload: Multipart, req: HttpRequest) -> Result<HttpResponse, Error>{

    let mut noticia = NewNoticia::default(); 
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
                        noticia.titulo = titulo;
                    }
                    "conteudo" => {
                        let mut conteudo = String::new();
                        while let Some(chunk) = field.next().await {
                            let data = chunk?;
                            conteudo.push_str(std::str::from_utf8(&data)?);
                        }
                        noticia.conteudo = conteudo;
                    }
                    "autor" => {
                        let mut autor = String::new();
                        while let Some(chunk) = field.next().await {
                            let data = chunk?;
                            autor.push_str(std::str::from_utf8(&data)?);
                        }
                        noticia.autor = autor;
                    }
                    "imagem" => {
                        while let Some(chunk) = field.next().await {
                            let data = chunk?;
                            noticia.imagem.write_all(&data)?;
                        }
                    }
                    _ => {}
                }
            }
        }
    Noticia::insert(noticia, conn);

    Ok(HttpResponse::Ok().finish())
}

#[put("/noticia/{id}")]
async fn put_noticia(id: web::Path<i32>, mut payload: Multipart) -> Result<HttpResponse, Error>{
    let conn = establish_connection();
    let conn_2 = establish_connection();

    let _noticia = Noticia::find_by_id(*id, conn_2).unwrap();
    let mut noticia = NewNoticia::default(); 

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
                        noticia.titulo = titulo;
                    }
                    "conteudo" => {
                        let mut conteudo = String::new();
                        while let Some(chunk) = field.next().await {
                            let data = chunk?;
                            conteudo.push_str(std::str::from_utf8(&data)?);
                        }
                        noticia.conteudo = conteudo;
                    }
                    "autor" => {
                        let mut autor = String::new();
                        while let Some(chunk) = field.next().await {
                            let data = chunk?;
                            autor.push_str(std::str::from_utf8(&data)?);
                        }
                        noticia.autor = autor;
                    }
                    "imagem" => {
                        while let Some(chunk) = field.next().await {
                            let data = chunk?;
                            noticia.imagem.write_all(&data)?;
                        }
                    }
                    _ => {}
                }
            }
        }

    if  noticia.imagem.is_empty() {
        noticia.imagem = _noticia.imagem;
    }

    Noticia::update(id.into_inner(), noticia, conn).unwrap();

    Ok(HttpResponse::Ok().finish())
}


#[get("/noticia")]
async fn get_noticias() -> impl Responder{
    let conn = establish_connection();

    let noticias = Noticia::find_all(conn).unwrap();
    let mut noticias_formatado: Vec<NoticiaDTO> = Vec::new();

    for noticia in noticias.iter() {
        let noticia_formatado = NoticiaDTO {
            id: noticia.id,
            titulo: noticia.titulo.clone(),
            data: noticia.data.format("%d-%m-%Y %H:%M:%S").to_string(),
            autor: noticia.autor.clone(),
            conteudo: noticia.conteudo.clone()
        };

        noticias_formatado.push(noticia_formatado);
    }

    HttpResponse::Ok().json(noticias_formatado)
}

#[get("/noticia/{id}/imagem")]
async fn get_noticia_image(id: web::Path<i32>) -> impl Responder {
    let conn = establish_connection();

    let imagem = Noticia::find_image(id.into_inner(), conn).unwrap();

    HttpResponse::Ok().content_type("image/png").body(imagem)
}

#[get("/noticia/recentes")]
async fn get_noticias_recentes() -> impl Responder {
    let conn = establish_connection();

    let noticias_recentes = Noticia::find_noticias_recente(conn).unwrap();

    HttpResponse::Ok().json(noticias_recentes)
}
