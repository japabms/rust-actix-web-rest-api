use actix_multipart::Multipart;
use actix_web::{
    error::Error,
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound},
    HttpResponse,
};
use futures_util::StreamExt as _;
use futures_util::TryStreamExt as _;
use std::io::Write;

use diesel::PgConnection;
use crate::{db::establish_connection, models::noticia::*};

pub fn find_all(conn:  &mut PgConnection) -> Result<HttpResponse, Error> {

    let noticias = match Noticia::find_all(conn) {
        Ok(noticias) => noticias,
        Err(err) => return Err(ErrorInternalServerError(err)),
    };

    let mut noticias_formatado: Vec<NoticiaDTO> = Vec::new();

    for noticia in noticias.iter() {
        let noticia_formatado = NoticiaDTO {
            id: noticia.id,
            titulo: noticia.titulo.clone(),
            data: noticia.data.format("%d-%m-%Y %H:%M:%S").to_string(),
            autor: noticia.autor.clone(),
            conteudo: noticia.conteudo.clone(),
        };

        noticias_formatado.push(noticia_formatado);
    }

    Ok(HttpResponse::Ok().json(noticias_formatado))
}

pub fn find_by_id(id: i32, conn:  &mut PgConnection) -> Result<HttpResponse, Error> {

    let noticia = match Noticia::find_by_id(id, conn) {
        Ok(noticia) => noticia,
        Err(err) => {
            return Err(ErrorNotFound(format!(
                "Não foi encontrado nenhuma noticia com o id {}\n{}",
                id, err
            )))
        }
    };

    let noticia_formatado = NoticiaDTO {
        id: noticia.id,
        titulo: noticia.titulo.clone(),
        data: noticia.data.format("%d-%m-%Y %H:%M:%S").to_string(),
        autor: noticia.autor.clone(),
        conteudo: noticia.conteudo.clone(),
    };

    Ok(HttpResponse::Ok().json(noticia_formatado))
}

pub fn find_imagem(id: i32, conn:  &mut PgConnection) -> Result<HttpResponse, Error> {

    match Noticia::find_image(id, conn) {
        Ok(img) => Ok(HttpResponse::Ok().content_type("image/png").body(img)),
        Err(err) => Err(ErrorNotFound(format!(
            "Não foi encontrado nenhuma noticia com o id {}\n{}",
            id, err
        ))),
    }
}

pub fn find_noticia_recente(conn:  &mut PgConnection) -> Result<HttpResponse, Error> {

    match Noticia::find_noticias_recente(conn) {
        Ok(noticias) => Ok(HttpResponse::Ok().json(noticias)),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}

pub async fn insert(mut payload: Multipart, conn:  &mut PgConnection) -> Result<HttpResponse, Error> {
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
    match Noticia::insert(noticia, conn) {
        Ok(_) => Ok(HttpResponse::NotFound().finish()),
        Err(err) => Err(ErrorBadRequest(err)),
    }
}

pub async fn update(id: i32, mut payload: Multipart, conn:  &mut PgConnection) -> Result<HttpResponse, Error> {
    let mut conn_2 = establish_connection();

    let _noticia = match Noticia::find_by_id(id, &mut conn_2) {
        Ok(noticia) => noticia,
        Err(err) => return Err(ErrorNotFound(err)),
    };

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

    if noticia.imagem.is_empty() {
        noticia.imagem = _noticia.imagem;
    }

    match Noticia::update(id, noticia, conn) {
        Ok(i) => {
            if i == 0 {
                Err(ErrorNotFound(format!(
                    "Não foi encontrado nenhuma noticia com o id {}",
                    id
                )))
            } else {
                Ok(HttpResponse::Ok().finish())
            }
        }
        Err(err) => Err(ErrorBadRequest(err)),
    }
}

pub fn delete(id: i32, conn:  &mut PgConnection) -> Result<HttpResponse, Error> {

    match Noticia::delete_noticia(id, conn) {
        Ok(i) => {
            if i == 0 {
                Err(ErrorNotFound(format!(
                    "Não foi encontrado nenhuma noticia com o id {}",
                    id
                )))
            } else {
                Ok(HttpResponse::Ok().finish())
            }
        }
        Err(err) => Err(ErrorBadRequest(err)),
    }
}
