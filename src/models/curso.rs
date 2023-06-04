use crate::schema::curso::{self, dsl::*};
use utoipa::ToSchema;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(ToSchema, Queryable, Selectable, Identifiable, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = curso)]
#[diesel(primary_key(id))]
pub struct Curso {
    pub id: i32,
    pub nome: String,
    pub preco: i32,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Clone, ToSchema)]
#[diesel(table_name = curso)]
pub struct CursoDTO {
    pub nome: String,
    pub preco: i32,
}

impl Curso {
    pub fn find_all(mut conn: PgConnection) -> QueryResult<Vec<Curso>> {
        curso.load::<Curso>(&mut conn)
    }

    pub fn find_by_id(i: i32, mut conn: PgConnection) -> QueryResult<Curso> {
        curso.find(i).get_result::<Curso>(&mut conn)
    }

    pub fn insert(new_curso: CursoDTO, mut conn: PgConnection) -> QueryResult<usize> {
        diesel::insert_into(curso)
            .values(&new_curso)
            .execute(&mut conn)
    }

    pub fn update(i: i32, edited_curso: CursoDTO, mut conn: PgConnection) -> QueryResult<usize> {
        diesel::update(curso::table)
            .filter(curso::id.eq(i))
            .set(&edited_curso)
            .execute(&mut conn)
    }

    pub fn delete(i: i32, mut conn: PgConnection) -> QueryResult<usize> {
        diesel::delete(curso.find(i)).execute(&mut conn)
    }
}
