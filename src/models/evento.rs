use crate::schema::eventos::{self, dsl::*};
use utoipa::ToSchema;
use diesel::PgConnection;

use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable, Debug, PartialEq, ToSchema)]
#[diesel(table_name = eventos)]
pub struct Evento {
    pub id: i32,
    pub titulo: String,
    pub sobre: String,
    pub data_inicio: NaiveDate,
    pub data_fim: NaiveDate,
    pub tipo: String,
    pub email: String,
    pub icone: Vec<u8>,
}

#[derive(Debug, Default, AsChangeset, Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = eventos)]
pub struct NewEvento {
    pub titulo: String,
    pub sobre: String,
    pub data_inicio: NaiveDate,
    pub data_fim: NaiveDate,
    pub tipo: String,
    pub email: String,
    pub icone: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct EventoDtoDataFormatada {
    pub id: i32,
    pub titulo: String,
    pub sobre: String,
    pub data_inicio: String,
    pub data_fim: String,
    pub tipo: String,
    pub email: String,
}

impl Evento {
    pub fn find_all(mut conn: PgConnection) -> QueryResult<Vec<Evento>>{
        eventos
            .load(&mut conn)
    }

    pub fn find_by_id(i: i32, mut conn: PgConnection) -> QueryResult<Evento> {
        eventos.filter(eventos::id.eq(i))
            .get_result(&mut conn)
    }

    pub fn find_icone(i: i32, mut conn: PgConnection) -> QueryResult<Vec<u8>> {
        eventos.filter(eventos::id.eq(i))
            .select(eventos::icone)
            .get_result::<Vec<u8>>(&mut conn)
    }
 
    pub fn insert(new_evento: NewEvento, mut conn: PgConnection) -> QueryResult<i32> {
        diesel::insert_into(eventos)
            .values(&new_evento)
            .returning(eventos::id)
            .get_result::<i32>(&mut conn)
    }

    pub fn update(i: i32, evento: NewEvento, mut conn: PgConnection) -> QueryResult<usize> {
        diesel::update(eventos)
            .filter(eventos::id.eq(i))
            .set(&evento)
            .execute(&mut conn)
    }
}
