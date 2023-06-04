use crate::schema::artigos::{self, dsl::*};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::artigo_categorias::ArtigoCategorias;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Default)]
pub struct ArtigoComCategorias {
    pub titulo: String,
    pub resumo: String,
    pub palavra_chave: String,
    pub documento: Vec<u8>,
    pub categorias: Vec<i32>,
}

impl Artigo {
    pub fn find_all(mut conn: PgConnection) -> QueryResult<Vec<Artigo>> {
        artigos.load(&mut conn)
    }

    pub fn find_by_id(i: i32, mut conn: PgConnection) -> QueryResult<Artigo> {
        artigos
            .filter(artigos::id.eq(i))
            .get_result(&mut conn)
    }

    pub fn insert(artigo: ArtigoComCategorias, mut conn: PgConnection) -> QueryResult<usize> {
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
            .get_result(&mut conn)
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
}
