use crate::{
    schema::noticias::{self, dsl::*},
    db::Pool
};

use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

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


#[derive(Default, Insertable, Serialize, Deserialize)]
#[diesel(table_name = noticias)]
pub struct NoticiaDTO {
    pub titulo: String,
    pub conteudo: String,
    pub autor: String,
    pub imagem: Vec<u8>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = noticias)]
pub struct NoticiaDtoMinimal {
    pub titulo: String,
    pub conteudo: String,
    pub autor: String,
    pub data: NaiveDateTime,
}

impl Noticia {
    pub fn insert(new_noticia: NoticiaDTO, pool: Pool) -> i32 {
        diesel::insert_into(noticias)
            .values(&new_noticia)
            .returning(noticias::id)
            .get_result::<i32>(&mut pool.get().unwrap())
            .expect("Erro ao inserir nova noticia")
    }

    pub fn find_all(pool: Pool) -> QueryResult<Vec<NoticiaDtoMinimal>>  {
       noticias.select(NoticiaDtoMinimal::as_select()).load(&mut pool.get().unwrap())
    }

    pub fn find_noticia_image(i: i32, pool: Pool) -> QueryResult<Vec<u8>> {
        noticias.filter(noticias::id.eq(i))
            .select(noticias::imagem)
            .get_result(&mut pool.get().unwrap())
    }
}
