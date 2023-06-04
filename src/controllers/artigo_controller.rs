use actix_multipart::Multipart;
use actix_web::{get, post, put, web, HttpResponse, Responder};

use crate::{services::artigo_service, models::artigo::ArtigoComCategorias};

#[utoipa::path(
    tag = "Artigo",
    request_body(content = ArtigoComCategorias, description = "Artigo to store the database", content_type = "multipart/form-data"),
    responses (
        (status = 200, description = "Artigo postado com sucesso.",),
        (status = NOT_FOUND)
    ),
)]
#[post("/artigo")]
async fn post_artigo(payload: Multipart) -> impl Responder {
    match artigo_service::insert(payload).await {
        Ok(()) => HttpResponse::Ok().body("Oi"),
        Err(res) => res.into(),
    }
}
