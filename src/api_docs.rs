use std::env;
use std::fs::File;
use std::io::Write;

use crate::controllers::{
    artigo_controller::*, atividade_controller::*, categoria_controller::*, curso_controller::*,
    evento_controller::*, inscrito_controller::*, noticia_controller::*,
};

use crate::models::{artigo::*, atividade::*, categoria::*, curso::*, evento::*, inscrito::*, noticia::*};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "CONFESC-API"),
    paths(
        get_cursos,
        get_curso_by_id,
        post_curso,
        delete_curso,
        put_curso,
        get_categorias,
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
        post_noticia,
        get_artigo_documento,
        get_artigo_by_id,
        get_artigos,
        delete_artigo,
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
pub struct ApiDoc;

pub fn update_api_docs() -> Result<(), std::io::Error> {

    let mut openapi_json: File = File::create("openapi.json")?;
    openapi_json.write_all(ApiDoc::openapi().to_pretty_json().unwrap().as_bytes())?;
    Ok(())
}

pub fn get_open_api_json_path() -> Result<String, std::io::Error> {
     match env::current_dir()?.to_str() {
        Some(path) => {
            let path_to_open_api_docs = format!("{}/{}", path.to_owned(), "openapi.json");
            Ok(path_to_open_api_docs)
        }
        None => {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Erro ao converter o diretoria para str"))
        }
    }
}
