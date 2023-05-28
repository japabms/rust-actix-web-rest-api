#[allow(unused_imports, unused_variables)]
pub mod controllers;
pub mod models;
pub mod schema;
pub mod db;

use controllers::{
    curso_controller::*,
    inscrito_controller::*,
    noticia_controller::*, 
    evento_controller::*,
};

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

            //#[put("/noticia")]
            .service(put_noticia)

            //#[put("/noticia/{id}/imagem")]
            .service(put_noticia_imagem)

            //#[get("/evento/{id}")]
            .service(get_evento_by_id)

            //#[post("/evento")]
            .service(post_evento)

            //#[put("/evento/{id}")]
            .service(put_evento)

            //#[get("/evento/{id}/icone")]
            .service(get_evento_icone)

    }).bind(("0.0.0.0", 8080))? //0.0.0.0 binda o server em todas as interfaces de rede disponiveis
        .run()
        .await
}

