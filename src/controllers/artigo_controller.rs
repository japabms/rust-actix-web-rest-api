use actix_multipart::Multipart;
use actix_web::error::ErrorBadRequest;
use actix_web::web::Bytes;
use actix_web::Error;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use futures_util::StreamExt as _;
use futures_util::TryStreamExt as _;
use std::io::Write;

use crate::{db::establish_connection, models::artigo::*};

#[post("/artigo")]
async fn post_artigo(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut artigo = ArtigoComCategorias::default();
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

                        for int in integer_str.split_whitespace() {
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

    Artigo::insert(artigo, conn);

    Ok(HttpResponse::Ok().finish())
}
