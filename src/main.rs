pub mod controllers;
pub mod db;
pub mod models;
pub mod schema;
#[allow(unused_imports, unused_variables)]
pub mod services;

// use std::{fs::File, io::Write};

use controllers::{
    artigo_controller::*, atividade_controller::*, categoria_controller::*, curso_controller::*,
    evento_controller::*, inscrito_controller::*, noticia_controller::*,
};

use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use db::establish_connection;
use utoipa_swagger_ui::SwaggerUi;

use models::{artigo::*, atividade::*, categoria::*, curso::*, evento::*, noticia::*, inscrito::*};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "CESC-API"),
    paths(
        get_cursos,
        get_curso_by_id,
        post_curso,
        delete_curso,
        put_curso,
        get_categoria,
        post_categoria,
        delete_categoria,
        post_artigo,
        get_atividades,
        get_atividade_by_id,
        put_atividade,
        post_atividade,
        delete_atividade,
        post_evento,
        get_eventos,
        get_evento_icone,
        get_evento_by_id,
        put_evento,
        get_inscritos,
        get_inscrito_by_id,
        get_inscrito_cursos,
        post_inscrito,
        get_noticias,
        get_noticia_by_id,
        get_noticia_imagem,
        get_noticias_recentes,
        put_noticia,
        delete_noticia,
        post_noticia
    ),
    components(schemas(
        Curso,
        CursoDTO,
        Categoria,
        NewCategoria,
        Artigo,
        ArtigoComCategorias,
        Atividade,
        AtividadeDTO,
        Evento,
        NewEvento,
        InscritoWithCursosDTO,
        NewNoticia,
    ))
)]
struct ApiDoc;

//TODO!:
// adicionar web::Data<Pool> ao App para evitar abrir uma conexão com o banco a cada chamada

#[get("/db/{table_name}")]
async fn serial_util(table: web::Path<String>) -> impl Responder {
    let conn = establish_connection();

    match db::reset_serial(table.into_inner().as_str(), "id", conn) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // let mut file = File::create("openapi.json")?;
    // file.write_all(ApiDoc::openapi().to_pretty_json().unwrap().as_bytes())?;

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(get_inscritos)
            .service(get_inscrito_by_id)
            .service(get_inscrito_cursos)
            .service(post_inscrito)
            .service(get_cursos)
            .service(get_curso_by_id)
            .service(put_curso)
            .service(post_curso)
            .service(delete_curso)
            .service(post_noticia)
            .service(get_noticias)
            .service(get_noticia_imagem)
            .service(put_noticia)
            .service(get_noticia_by_id)
            .service(get_noticias_recentes)
            .service(delete_noticia)
            .service(get_evento_by_id)
            .service(post_evento)
            .service(put_evento)
            .service(get_eventos)
            .service(get_evento_icone)
            .service(get_atividades)
            .service(get_atividade_by_id)
            .service(put_atividade)
            .service(post_atividade)
            .service(delete_atividade)
            .service(post_artigo)
            .service(post_categoria)
            .service(get_categoria)
            .service(delete_categoria)
            .service(serial_util)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
                "/home/victor/workspace/rust/cesc-api/openapi.json",
                ApiDoc::openapi(),
            ))
    })
    .bind(("0.0.0.0", 8080))? //0.0.0.0 binda o server em todas as interfaces de rede disponiveis
    .run()
    .await
}
