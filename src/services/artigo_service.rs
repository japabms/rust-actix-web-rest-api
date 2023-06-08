use crate::models::artigo::*;
use actix_multipart::Multipart;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::web::Bytes;
use actix_web::Error;
use actix_web::HttpResponse;
use diesel::PgConnection;
use futures_util::StreamExt as _;
use futures_util::TryStreamExt as _;
use std::io::Write;

pub async fn insert(mut payload: Multipart, conn: &mut PgConnection) -> Result<(), Error> {
    let mut artigo = ArtigoComCategorias::default();

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
                    artigo.titulo = titulo;
                }
                "resumo" => {
                    let mut resumo = String::new();
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        resumo.push_str(std::str::from_utf8(&data)?);
                    }
                    artigo.resumo = resumo;
                }
                "palavra_chave" => {
                    let mut palavra_chave = String::new();
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        palavra_chave.push_str(std::str::from_utf8(&data)?);
                    }
                    artigo.palavra_chave = palavra_chave;
                }
                "documento" => {
                    while let Some(chunk) = field.next().await {
                        let data = chunk?;
                        artigo.documento.write_all(&data)?;
                    }
                }
                "categorias" => {
                    let mut categorias: Vec<i32> = Vec::new();

                    while let Some(item) = field.next().await {
                        let bytes: Bytes =
                            item.map_err(|_| ErrorBadRequest("Failed to read field bytes"))?;
                        let integer_str = String::from_utf8(bytes.to_vec()).map_err(|_| {
                            ErrorBadRequest("Failed to parse field as UTF-8 string")
                        })?;

                        for int in integer_str.split(',') {
                            if let Ok(integer) = int.parse::<i32>() {
                                categorias.push(integer);
                            } else {
                                return Err(ErrorBadRequest("Failed to parse field as integer"));
                            }
                        }
                    }
                    artigo.categorias.append(&mut categorias);
                }
                _ => {}
            }
        }
    }

    match Artigo::insert(artigo, conn) {
        Ok(_) => Ok(()),
        Err(err) => Err(ErrorBadRequest(format!(
            "Não foi possível completar a sua requisição.\n {}",
            err
        ))),
    }
}

pub fn find_artigo_documento(id: i32, conn: &mut PgConnection) -> Result<HttpResponse, Error> {
    match Artigo::find_documento(id, conn) {
        Ok(documento) => Ok(HttpResponse::Ok()
            .content_type("application/pdf")
            .body(documento)),
        Err(err) => Err(ErrorNotFound(err)),
    }
}

pub fn find_all(conn: &mut PgConnection) -> Result<HttpResponse, Error> {
    let mut artigos_dto: Vec<ArtigoDTO> = Vec::new();

    let artigos = match Artigo::find_all(conn) {
        Ok(artigos) => artigos,
        Err(err) => return Err(ErrorInternalServerError(err)),
    };

    for artigo in artigos {
        let artigo_dto = ArtigoDTO {
            id: artigo.id,
            titulo: artigo.titulo,
            resumo: artigo.resumo,
            palavra_chave: artigo.palavra_chave,
        };

        artigos_dto.push(artigo_dto);
    }

    Ok(HttpResponse::Ok().json(artigos_dto))
}

pub fn find_by_id(id: i32, conn: &mut PgConnection) -> Result<HttpResponse, Error> {
    let artigo = match Artigo::find_by_id(id, conn) {
        Ok(artigo) => artigo,
        Err(err) => return Err(ErrorNotFound(err)),
    };

    let artigo_dto = ArtigoDTO {
        id: artigo.id,
        titulo: artigo.titulo,
        resumo: artigo.resumo,
        palavra_chave: artigo.palavra_chave,
    };

    Ok(HttpResponse::Ok().json(artigo_dto))
}

pub fn delete(id: i32, conn: &mut PgConnection) -> Result<HttpResponse, Error> {
    match Artigo::delete(id, conn) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Err(ErrorNotFound(err)),
    }
}
