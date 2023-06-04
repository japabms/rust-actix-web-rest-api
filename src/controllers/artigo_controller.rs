use actix_multipart::Multipart;
use actix_web::{get, post, put, web, HttpResponse, Responder};

use crate::services::artigo_service;

#[post("/artigo")]
async fn post_artigo(payload: Multipart) -> impl Responder {
    match artigo_service::insert(payload).await {
        Ok(()) => HttpResponse::Ok().body("Oi"),
        Err(res) => res.into(),
    }
}
