use crate::schema::noticias::{self, dsl::*};

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = noticias)]
pub struct Noticia {
    pub id: i32,
    pub titulo: String,
    pub conteudo: String,
    pub autor: String,
    pub data: NaiveDateTime,
    pub imagem: Vec<u8>,
}

#[derive(Default, AsChangeset, Insertable, Serialize, Deserialize)]
#[diesel(table_name = noticias)]
pub struct NewNoticia {
    pub titulo: String,
    pub conteudo: String,
    pub autor: String,
    pub imagem: Vec<u8>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct NoticiaDTO {
    pub id: i32,
    pub titulo: String,
    pub conteudo: String,
    pub autor: String,
    pub data: String,
}

impl Noticia {
    pub fn find_all(mut conn: PgConnection) -> QueryResult<Vec<Noticia>> {
        noticias.load(&mut conn)
    }

    pub fn find_by_id(i: i32, mut conn: PgConnection) -> QueryResult<Noticia> {
        noticias.filter(noticias::id.eq(i)).get_result(&mut conn)
    }

    pub fn find_image(i: i32, mut conn: PgConnection) -> QueryResult<Vec<u8>> {
        noticias
            .filter(noticias::id.eq(i))
            .select(noticias::imagem)
            .get_result(&mut conn)
    }
    pub fn insert(new_noticia: NewNoticia, mut conn: PgConnection) -> i32 {
        diesel::insert_into(noticias)
            .values(&new_noticia)
            .returning(noticias::id)
            .get_result::<i32>(&mut conn)
            .expect("Erro ao inserir nova noticia")
    }

    pub fn update(i: i32, updated_noticia: NewNoticia, mut conn: PgConnection) -> QueryResult<usize> {
        diesel::update(noticias)
            .filter(noticias::id.eq(i))
            .set((
                noticias::titulo.eq(updated_noticia.titulo),
                noticias::autor.eq(updated_noticia.autor),
                noticias::conteudo.eq(updated_noticia.conteudo),
                noticias::imagem.eq(updated_noticia.imagem),
            ))
            .execute(&mut conn)
    }

    pub fn find_noticias_recente(mut conn: PgConnection) -> QueryResult<Vec<Noticia>> {
        noticias
            .order(noticias::data.desc())
            .limit(5)
            .load(&mut conn)
    }

    pub fn delete_noticia(i: i32, mut conn: PgConnection) -> QueryResult<usize> {
        diesel::delete(noticias)
            .filter(noticias::id.eq(i))
            .execute(&mut conn)
    }
}
