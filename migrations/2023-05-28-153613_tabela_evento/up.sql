-- Your SQL goes here

CREATE TABLE eventos (
    id SERIAL PRIMARY KEY,
    titulo VARCHAR(300) NOT NULL, 
    sobre VARCHAR NOT NULL,
    data_inicio DATE NOT NULL,
    data_fim DATE NOT NULL,
    tipo VARCHAR(100) NOT NULL,
    email VARCHAR(300) NOT NULL,
    icone BYTEA NOT NULL
);
