// @generated automatically by Diesel CLI.

diesel::table! {
    atividades (id) {
        id -> Int4,
        titulo -> Varchar,
        descricao -> Varchar,
        responsavel -> Varchar,
        inicio -> Timestamp,
        fim -> Timestamp,
    }
}

diesel::table! {
    curso (id) {
        id -> Int4,
        nome -> Varchar,
        preco -> Int4,
    }
}

diesel::table! {
    eventos (id) {
        id -> Int4,
        titulo -> Varchar,
        sobre -> Varchar,
        data_inicio -> Date,
        data_fim -> Date,
        tipo -> Varchar,
        email -> Varchar,
        icone -> Bytea,
    }
}

diesel::table! {
    inscrito (id) {
        id -> Int4,
        nome -> Varchar,
        nome_cracha -> Varchar,
        email -> Varchar,
        cpf -> Varchar,
        modalidade_nome -> Varchar,
        modalidade_preco -> Int4,
        instituicao -> Varchar,
    }
}

diesel::table! {
    inscrito_cursos (inscrito_id, curso_id) {
        inscrito_id -> Int4,
        curso_id -> Int4,
    }
}

diesel::table! {
    noticias (id) {
        id -> Int4,
        titulo -> Varchar,
        conteudo -> Varchar,
        autor -> Varchar,
        data -> Timestamp,
        imagem -> Bytea,
    }
}

diesel::joinable!(inscrito_cursos -> curso (curso_id));
diesel::joinable!(inscrito_cursos -> inscrito (inscrito_id));

diesel::allow_tables_to_appear_in_same_query!(
    atividades,
    curso,
    eventos,
    inscrito,
    inscrito_cursos,
    noticias,
);
