CREATE TYPE tipo_lengua AS ENUM ('oficial', 'indigena', 'extranjera');

CREATE TABLE lenguas (
    id_lenguas SERIAL PRIMARY KEY,
    nombre VARCHAR(30) NOT NULL,
    tipo tipo_lengua NOT NULL
);

INSERT INTO lenguas (nombre, tipo) VALUES 
('español', 'oficial'),
('ingles', 'extranjera');
