use crate::{
    models::inscrito_cursos::InscritoCurso,
    schema::inscrito::{self, dsl::*},
};
use diesel::prelude::*;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    ToSchema, Queryable, Selectable, Identifiable, PartialEq, Debug, Clone, Serialize, Deserialize,
)]
#[diesel(table_name = inscrito)]
pub struct Inscrito {
    pub id: i32,
    pub nome: String,
    pub nome_cracha: String,
    pub email: String,
    pub cpf: String,
    pub modalidade_nome: String,
    pub modalidade_preco: i32,
    pub instituicao: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = inscrito)]
pub struct InscritoDTO {
    pub nome: String,
    pub nome_cracha: String,
    pub email: String,
    pub cpf: String,
    pub modalidade_nome: String,
    pub modalidade_preco: i32,
    pub instituicao: String,
}

#[derive(ToSchema, Serialize, Deserialize, Clone)]
pub struct InscritoWithCursosDTO {
    pub nome: String,
    pub nome_cracha: String,
    pub email: String,
    pub cpf: String,
    pub modalidade_nome: String,
    pub modalidade_preco: i32,
    pub instituicao: String,
    pub cursos: Option<Vec<i32>>,
}

impl Inscrito {
    pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<Inscrito>> {
        inscrito.order_by(inscrito::id).load::<Inscrito>(conn)
    }

    pub fn find_by_id(i: i32, conn: &mut PgConnection) -> QueryResult<Inscrito> {
        inscrito.find(i).get_result::<Inscrito>(conn)
    }

    pub fn insert(
        new_inscrito: InscritoWithCursosDTO,
        conn: &mut PgConnection,
    ) -> QueryResult<usize> {
        //Inscrito a ser inserido
        let insert_inscrito = InscritoDTO {
            nome: new_inscrito.nome,
            nome_cracha: new_inscrito.nome_cracha,
            email: new_inscrito.email,
            cpf: new_inscrito.cpf,
            modalidade_nome: new_inscrito.modalidade_nome,
            modalidade_preco: new_inscrito.modalidade_preco,
            instituicao: new_inscrito.instituicao,
        };

        // Inserindo inscrito e retornando seu ID, para possivelmente adicionar os seus cursos
        // posteriormente
        let inserted_inscrito_id: i32 = diesel::insert_into(inscrito::table)
            .values(&insert_inscrito)
            .returning(inscrito::id)
            .get_result::<i32>(conn)
            .expect("Erro ao realizar a inscrição");

        //Verificando se existe algum curso, e registrando no BD
        if let Some(cursos) = new_inscrito.cursos {
            let c = cursos
                .iter()
                .map(|&c_id| InscritoCurso {
                    inscrito_id: inserted_inscrito_id,
                    curso_id: c_id,
                })
                .collect::<Vec<InscritoCurso>>();

            InscritoCurso::insert(c, conn)
        } else {
            Ok(1)
        }
    }

    pub fn delete(i: i32, conn: &mut PgConnection) -> QueryResult<usize> {
        diesel::delete(inscrito.find(i)).execute(conn)
    }
}
