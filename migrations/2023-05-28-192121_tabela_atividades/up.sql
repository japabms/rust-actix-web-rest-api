-- Your SQL goes here
CREATE TABLE atividades (
    id SERIAL PRIMARY KEY,
    titulo VARCHAR(300) NOT NULL,
    descricao VARCHAR NOT NULL,
    responsavel VARCHAR(200) NOT NULL,
    inicio TIMESTAMP NOT NULL, 
    fim TIMESTAMP NOT NULL
);
