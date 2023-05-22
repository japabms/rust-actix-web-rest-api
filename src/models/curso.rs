use crate::{
    schema::curso::{self, dsl::*},
    db::Pool,
};

use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = curso)]
#[diesel(primary_key(id))]
pub struct Curso {
    pub id: i32,
    pub nome: String,
    pub preco: i32,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Clone)]
#[diesel(table_name = curso)]
pub struct CursoDTO {
    pub nome: String, 
    pub preco: i32,
}

impl Curso {
    pub fn find_all(pool: Pool) -> QueryResult<Vec<Curso>> {
       curso.load::<Curso>(&mut pool.get().unwrap()) 
    }


    pub fn find_by_id(i: i32, pool: Pool) -> QueryResult<Curso> {
        curso.find(i).get_result::<Curso>(&mut pool.get().unwrap())
    }

    pub fn insert(new_curso: CursoDTO, pool: Pool) -> i32 {
        diesel::insert_into(curso)
            .values(&new_curso)
            .returning(curso::id)
            .get_result::<i32>(&mut pool.get().unwrap())
            .expect("Erro ao registrar novo curso")
    }

    pub fn update(i: i32, edited_curso: CursoDTO, pool: Pool) -> QueryResult<usize> {
        diesel::update(curso::table)
            .filter(curso::id.eq(i))
            .set(&edited_curso)
            .execute(&mut pool.get().unwrap())
    }
    
    pub fn delete(i: i32, pool: Pool) -> QueryResult<usize> {
        diesel::delete(curso.find(i))
            .execute(&mut pool.get().unwrap())
    }
}
