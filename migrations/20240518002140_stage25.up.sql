CREATE TYPE grado_parentesco AS ENUM ('primero', 'segundo', 'tercero', 'no_aplica');

CREATE TABLE parentescos (
    id_parentesco SERIAL PRIMARY KEY,
    nombre VARCHAR(30) NOT NULL,
    es_familiar BOOLEAN NOT NULL,
    grado grado_parentesco NOT NULL,
    tipo_id INTEGER NOT NULL
);

