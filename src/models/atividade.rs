use crate::schema::atividades::{self, dsl::*};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Selectable, PartialEq, Debug, Serialize, Deserialize)]
pub struct Atividade {
    pub id: i32,
    pub titulo: String,
    pub descricao: String,
    pub responsavel: String,
    pub inicio: NaiveDateTime,
    pub fim: NaiveDateTime,
}

#[derive(Queryable, Selectable, AsChangeset, Insertable, Serialize, Deserialize)]
#[diesel(table_name = atividades)]
pub struct AtividadeDTO {
    pub titulo: String,
    pub descricao: String,
    pub responsavel: String,
    pub inicio: NaiveDateTime,
    pub fim: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct AtividadeDtoDataFormatada {
    pub id: i32,
    pub titulo: String,
    pub descricao: String,
    pub responsavel: String,
    pub inicio: String,
    pub fim: String,
}

impl Atividade {
    pub fn find_all(mut conn: PgConnection) -> QueryResult<Vec<Atividade>> {
        atividades.load(&mut conn)
    }

    pub fn find_by_id(i: i32, mut conn: PgConnection) -> QueryResult<Atividade> {
        atividades
            .filter(atividades::id.eq(i))
            .get_result(&mut conn)
    }

    pub fn insert(new_atividade: AtividadeDTO, mut conn: PgConnection) -> QueryResult<i32> {
        diesel::insert_into(atividades)
            .values(&new_atividade)
            .returning(atividades::id)
            .get_result(&mut conn)
    }

    pub fn update(i: i32, atividade: AtividadeDTO, mut conn: PgConnection) -> QueryResult<usize> {
        diesel::update(atividades)
            .filter(atividades::id.eq(i))
            .set(&atividade)
            .execute(&mut conn)
    }

    pub fn delete(i: i32, mut conn: PgConnection) -> QueryResult<usize> {
        diesel::delete(atividades)
            .filter(atividades::id.eq(i))
            .execute(&mut conn)
    }
}
