-- Your SQL goes here
CREATE TABLE categorias (
    id SERIAL PRIMARY KEY,
    nome VARCHAR NOT NULL
);

CREATE TABLE artigos (
    id SERIAL PRIMARY KEY,
    titulo VARCHAR NOT NULL,
    resumo VARCHAR NOT NULL,
    palavra_chave VARCHAR NOT NULL,
    documento BYTEA NOT NULL
);

CREATE TABLE artigo_categorias (
    artigo_id INT,
    categoria_id INT,
    PRIMARY KEY(artigo_id, categoria_id),
    FOREIGN KEY(artigo_id) REFERENCES artigos(id),
    FOREIGN KEY(categoria_id) REFERENCES categorias(id)
);

