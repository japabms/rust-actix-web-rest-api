-- Your SQL goes here
CREATE TABLE inscrito_cursos(
    inscrito_id INT,
    curso_id INT,
    FOREIGN KEY(inscrito_id) REFERENCES inscrito(id),
    FOREIGN KEY(curso_id) REFERENCES curso(id),
    PRIMARY KEY(inscrito_id, curso_id)
);
