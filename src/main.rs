#[allow(unused_imports, unused_variables)]
pub mod controllers;
pub mod db;
pub mod models;
pub mod schema;
pub mod services;
pub mod api_docs;


use controllers::{
    artigo_controller::*, atividade_controller::*, categoria_controller::*, curso_controller::*,
    evento_controller::*, inscrito_controller::*, noticia_controller::*,
};


use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use db::{establish_connection, get_pool, run_migrations};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use api_docs::ApiDoc;
use api_docs::update_api_docs;

//TODO!:
// Testar se as migrações funciona kekw


//Ignorar
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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    update_api_docs()?;

    // rodando as migrações
    let mut conn = establish_connection();
    run_migrations(&mut conn).expect("Error");

    let pool = web::Data::new(get_pool());

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .wrap(actix_web::middleware::Logger::default())

            .configure(init_artigo_routes)
            .configure(init_categoria_routes)
            .configure(init_curso_routes)
            .configure(init_atividade_routes)
            .configure(init_evento_routes)
            .configure(init_inscrito_routes)
            .configure(init_noticia_routes)

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
