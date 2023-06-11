use crate::schema::artigos::{self, dsl::*};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::models::artigo_categorias::ArtigoCategorias;

#[derive(ToSchema, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
pub struct Artigo {
    pub id: i32,
    pub titulo: String,
    pub resumo: String,
    pub palavra_chave: String,
    pub documento: Vec<u8>,
}

#[derive(Insertable, Serialize, Deserialize, Default)]
#[diesel(table_name = artigos)]
pub struct NewArtigo {
    pub titulo: String,
    pub resumo: String,
    pub palavra_chave: String,
    pub documento: Vec<u8>,
}

#[derive(Serialize, Deserialize, Default, IntoParams, ToSchema)]
pub struct ArtigoComCategorias {
    pub titulo: String,
    pub resumo: String,
    pub palavra_chave: String,
    pub documento: Vec<u8>,
    pub categorias: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ArtigoDTO {
    pub id: i32,
    pub titulo: String,
    pub resumo: String,
    pub palavra_chave: String,
}

impl Artigo {
    pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<Artigo>> {
        artigos.order_by(artigos::id).load(conn)
    }

    pub fn find_by_id(i: i32, conn: &mut PgConnection) -> QueryResult<Artigo> {
        artigos.filter(artigos::id.eq(i)).get_result(conn)
    }

    pub fn find_documento(i: i32, conn: &mut PgConnection) -> QueryResult<Vec<u8>> {
        artigos
            .filter(artigos::id.eq(i))
            .select(artigos::documento)
            .get_result(conn)
    }

    pub fn insert(artigo: ArtigoComCategorias, conn: &mut PgConnection) -> QueryResult<usize> {
        let inserir_artigo = NewArtigo {
            titulo: artigo.titulo,
            resumo: artigo.resumo,
            palavra_chave: artigo.palavra_chave,
            documento: artigo.documento,
        };

        for cat in artigo.categorias.iter() {
            println!("{:?}", cat);
        }

        let a_id = diesel::insert_into(artigos)
            .values(&inserir_artigo)
            .returning(artigos::id)
            .get_result(conn)
            .expect("Erro ao inserir artigo");

        let a = artigo
            .categorias
            .iter()
            .map(|&c_id| ArtigoCategorias {
                artigo_id: a_id,
                categoria_id: c_id,
            })
            .collect::<Vec<ArtigoCategorias>>();

        ArtigoCategorias::insert(a, conn)
    }

    pub fn delete(i: i32, conn: &mut PgConnection) -> QueryResult<usize> {
        diesel::delete(artigos)
            .filter(artigos::id.eq(i))
            .execute(conn)
    }
}
