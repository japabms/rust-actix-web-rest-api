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
            //#[get("/inscrito")]
            .service(get_inscritos)

            //#[get("/inscrito/{id}")]
            .service(get_inscrito_by_id)

            //#[get("/inscrito/{id}/cursos")]
            .service(get_inscrito_cursos)

            //#[post("/inscrever")]
            .service(post_inscrito)
            
            //#[get("/cursos")]
            .service(get_cursos)

            //#[post("/")]
            .service(post_curso)

            //#[post("/noticia")]
            .service(post_noticia)

            //#[get("/noticias")]
            .service(get_noticias)

            //#[get("/noticia/{id}/imagem")]
            .service(get_noticia_image)

    }).bind(("192.168.1.13", 8080))?
        .server_hostname("192.168.1.13")
        .run()
        .await
}

