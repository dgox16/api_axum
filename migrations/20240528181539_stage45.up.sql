CREATE TYPE tipo_documento AS ENUM (
    'personas',
    'captacion',
    'prestamos'
);

CREATE TABLE documentos (
    id_documento SERIAL PRIMARY KEY,
    nombre VARCHAR(60) NOT NULL,
    tipo tipo_documento NOT NULL
);

INSERT INTO documentos (nombre, tipo) VALUES
    ('CURP', 'personas'),
    ('RFC', 'personas');

CREATE TABLE documentos_de_persona (
    id_documento_de_persona SERIAL PRIMARY KEY,
    id_persona INTEGER NOT NULL,
    documento INTEGER NOT NULL,
    numero_identificacion VARCHAR(60) NOT NULL,
    es_identificacion BOOLEAN NOT NULL,
    vigencia DATE NOT NULL,
    FOREIGN KEY (id_persona) REFERENCES personas (id_persona) ON DELETE RESTRICT,
    FOREIGN KEY (documento) REFERENCES documentos (id_documento) ON DELETE RESTRICT
);
