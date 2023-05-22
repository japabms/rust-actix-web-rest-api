-- Your SQL goes here
CREATE TABLE inscrito (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(50) NOT NULL,
    nome_cracha VARCHAR(50) NOT NULL,
    email VARCHAR(80) NOT NULL,
    cpf VARCHAR(15) NOT NULL,
    modalidade_nome VARCHAR(80) NOT NULL,
    modalidade_preco INT NOT NULL,
    instituicao VARCHAR(150) NOT NULL
);

CREATE TABLE curso (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(200) NOT NULL,
    preco INT
);

