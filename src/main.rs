#[allow(unused_imports, unused_variables)]
pub mod controllers;
pub mod db;
pub mod models;
pub mod schema;

use controllers::{
    artigo_controller::*,
    atividade_controller::*,
    curso_controller::*,
    evento_controller::*,
    inscrito_controller::*,
    noticia_controller::*, 
    categoria_controller::*,
};

use actix_cors::Cors;
use actix_web::{App, HttpServer, web, Responder, HttpResponse, get};
use db::establish_connection;

//TODO!:
// adicionar web::Data<Pool> ao App para evitar abrir uma conexão com o banco a cada chamada
// Criar put para todos as entradas, mudar a imagem para opcional

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
            .service(get_noticia_image)
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
    })
    .bind(("0.0.0.0", 8080))? //0.0.0.0 binda o server em todas as interfaces de rede disponiveis
    .run()
    .await
    }
