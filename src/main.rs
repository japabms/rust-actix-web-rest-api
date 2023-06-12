pub mod api_docs;
#[allow(unused_imports, unused_variables)]
pub mod controllers;
pub mod db;
pub mod models;
pub mod schema;
pub mod services;

use api_docs::get_open_api_json_path;
use controllers::{
    artigo_controller::*, atividade_controller::*, categoria_controller::*, curso_controller::*,
    evento_controller::*, inscrito_controller::*, noticia_controller::*,
};

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use db::{establish_connection, get_pool, run_migrations};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use api_docs::update_api_docs;
use api_docs::ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    //loading api definition
    //this propably dont work on windows.
    let swagger_docs = get_open_api_json_path()?;
    update_api_docs()?;

    // rodando as migrações
    let mut conn = establish_connection();
    run_migrations(&mut conn).expect("Erro ao rodar migrações");

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
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url(swagger_docs.clone(), ApiDoc::openapi()),
            )
    })
    .bind(("0.0.0.0", 8080))? //0.0.0.0 to deploy
    .run()
    .await
}
