use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{artigo::Artigo, categoria::Categoria};
use crate::schema::artigo_categorias::{self, dsl::*};

#[derive(
    Associations,
    Insertable,
    Identifiable,
    Selectable,
    Queryable,
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
#[diesel(belongs_to(Categoria, foreign_key = categoria_id))]
#[diesel(belongs_to(Artigo, foreign_key = artigo_id))]
#[diesel(table_name = artigo_categorias)]
#[diesel(primary_key(artigo_id, categoria_id))]
pub struct ArtigoCategorias {
    pub artigo_id: i32,
    pub categoria_id: i32,
}

impl ArtigoCategorias {
    pub fn insert(ins_categorias: Vec<ArtigoCategorias>, mut conn: PgConnection) -> usize {
        diesel::insert_into(artigo_categorias)
            .values(&ins_categorias)
            .execute(&mut conn)
            .expect("Deu ruim")
    }
}
