-- Your SQL goes here
CREATE TABLE noticias (
    id SERIAL PRIMARY KEY,
    titulo VARCHAR(300) NOT NULL,
    conteudo VARCHAR NOT NULL,
    autor VARCHAR(100) NOT NULL,
    data TIMESTAMP DEFAULT current_timestamp NOT NULL,
    imagem BYTEA NOT NULL
);
