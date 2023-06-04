use crate::schema::categorias::{self, dsl::*};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, ToSchema, Serialize, Deserialize)]
pub struct Categoria {
    id: i32,
    nome: String,
}

#[derive(Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = categorias)]
pub struct NewCategoria {
    nome: String,
}

impl Categoria {
    pub fn find_all(mut conn: PgConnection) -> QueryResult<Vec<Categoria>> {
        categorias.load(&mut conn)
    }

    pub fn insert(new_categoria: NewCategoria, mut conn: PgConnection) -> QueryResult<usize> {
        diesel::insert_into(categorias)
            .values(&new_categoria)
            .execute(&mut conn)
    }

    pub fn delete(i: i32, mut conn: PgConnection) -> QueryResult<usize> {
        diesel::delete(categorias)
            .filter(categorias::id.eq(i))
            .execute(&mut conn)
    }
}
