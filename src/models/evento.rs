use crate::{
    schema::eventos::{self, dsl::*},
    db::Pool
};

use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable, Debug, PartialEq)]
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

#[derive(Default, AsChangeset, Insertable, Serialize, Deserialize)]
#[diesel(table_name = eventos)]
pub struct EventoDTO {
    pub titulo: String,
    pub sobre: String,
    pub data_inicio: NaiveDate,
    pub data_fim: NaiveDate,
    pub tipo: String,
    pub email: String,
    pub icone: Vec<u8>,
}

#[derive(Default, Queryable, Selectable, Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = eventos)]
pub struct EventoDtoMinimal {
    pub titulo: String,
    pub sobre: String,
    pub data_inicio: NaiveDate,
    pub data_fim: NaiveDate,
    pub tipo: String,
    pub email: String,
}

impl Evento {
    pub fn find_all(pool: Pool) -> QueryResult<Vec<EventoDtoMinimal>>{
        eventos.select(EventoDtoMinimal::as_select())
            .load(&mut pool.get().unwrap())
    }

    pub fn find_by_id(i: i32, pool: Pool) -> QueryResult<EventoDtoMinimal> {
        eventos.filter(eventos::id.eq(i))
            .select(EventoDtoMinimal::as_select())
            .get_result(&mut pool.get().unwrap())
    }

    pub fn find_icone(i: i32, pool: Pool) -> QueryResult<Vec<u8>> {
        eventos.filter(eventos::id.eq(i))
            .select(eventos::icone)
            .get_result::<Vec<u8>>(&mut pool.get().unwrap())
    }
 
    pub fn insert(new_evento: EventoDTO, pool: Pool) -> QueryResult<i32> {
        diesel::insert_into(eventos)
            .values(&new_evento)
            .returning(eventos::id)
            .get_result::<i32>(&mut pool.get().unwrap())
    }

    pub fn update(i: i32, evento: EventoDtoMinimal, pool: Pool) -> QueryResult<usize> {
        diesel::update(eventos)
            .filter(eventos::id.eq(i))
            .set(&evento)
            .execute(&mut pool.get().unwrap())
    }
}
