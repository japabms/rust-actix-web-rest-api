#[allow(unused_imports, unused_variables)]
pub mod controllers;
pub mod models;
pub mod schema;
pub mod db;

use controllers::{inscrito_controller::{get_inscritos, get_inscrito_by_id, get_inscrito_cursos, post_inscrito}, curso_controller::{get_cursos, post_curso}, noticia_controller::{post_noticia, get_noticias, get_noticia_image}};
use actix_web::{HttpServer, App};


//TODO!:
// adicionar web::Data<Pool> ao App para evitar abrir uma conexão com o banco a cada chamada
// Criar put para todos as entradas, mudar a imagem para opcional

#[actix_web::main]
async fn main() -> std::io::Result<()>  {

    HttpServer::new(||{
        App::new()
            .service(get_inscritos)
            .service(get_inscrito_by_id)
            .service(get_inscrito_cursos)
            .service(post_inscrito)
            .service(get_cursos)
            .service(post_curso)
            .service(post_noticia)
            .service(get_noticias)
            .service(get_noticia_image)

    }).bind(("127.0.0.1", 8080))?
        .run()
        .await
}

