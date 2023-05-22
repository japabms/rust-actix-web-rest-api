use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::{
    models::{
        inscrito::Inscrito,
        curso::Curso
    },
    schema::{inscrito_cursos, inscrito, curso},
    db::Pool,
};

#[derive(Associations, Insertable, Identifiable, Selectable, Queryable, Debug, Clone, Serialize, Deserialize)]
#[diesel(belongs_to(Curso, foreign_key = curso_id))]
#[diesel(belongs_to(Inscrito, foreign_key = inscrito_id))]
#[diesel(table_name = inscrito_cursos)]
#[diesel(primary_key(inscrito_id, curso_id))]
pub struct InscritoCurso {
    pub inscrito_id:  i32,
    pub curso_id:  i32
}

impl InscritoCurso {
    pub fn insert(ins_cursos: Vec<InscritoCurso>, pool: &Pool) -> QueryResult<usize> {
        diesel::insert_into(inscrito_cursos::table)
            .values(&ins_cursos)
            .execute(&mut pool.get().unwrap())
    }

    pub fn find_inscrito_cursos(i: i32, pool: &Pool) -> QueryResult<Vec<Curso>> {
        let inscrito = inscrito::table
            .filter(inscrito::id.eq(i))
            .select(Inscrito::as_select())
            .get_result(&mut pool.get().unwrap());
        
        InscritoCurso::belonging_to(&inscrito?)
            .inner_join(curso::table)
            .select(Curso::as_select())
            .load(&mut pool.get().unwrap())
    }
}
