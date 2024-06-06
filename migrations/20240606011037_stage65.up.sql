CREATE TYPE tipo_aseguradora AS ENUM (
    'no',
    'si',
    'no_aplica',
    'hipoteca',
    'avales'
);

CREATE TABLE aseguradoras (
    id_aseguradora SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    tipo tipo_aseguradora NOT NULL,
    especial INTEGER NOT NULL
);
