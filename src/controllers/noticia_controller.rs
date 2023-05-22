use std::io::Write;
use actix_multipart::Multipart;
use actix_web::Error;
use actix_web::HttpRequest;
use futures_util::StreamExt as _;
use futures_util::TryStreamExt as _;
use actix_web::{get, post, Responder, web, http::StatusCode, HttpResponse, ResponseError};
use bytes::Bytes;


use crate::{
    models::noticia::*,
    db::{Pool, establish_connection}
};

#[post("/noticia")]
async fn post_noticia(mut payload: Multipart, req: HttpRequest) -> Result<HttpResponse, Error>{

    let mut noticia = NoticiaDTO::default(); 
    let pool = establish_connection(); 

    loop {
        if let Ok(Some(mut field)) = payload.try_next().await {
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
        }else {
            break;
        }
    }

    Noticia::insert(noticia, pool);

    Ok(HttpResponse::Ok().finish())
}


#[get("/noticias")]
async fn get_noticias() -> impl Responder{
    let pool = establish_connection();

    match Noticia::find_all(pool) {
        Ok(noticias) => HttpResponse::Ok().json(noticias),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/noticia/{id}/imagem")]
async fn get_noticia_image(id: web::Path<i32>) -> impl Responder {
    let pool = establish_connection();

    let imagem = Noticia::find_noticia_image(id.into_inner(), pool).unwrap();

    HttpResponse::Ok().content_type("image/png").body(imagem)
}
