CREATE TYPE quien_es_tutor_de_persona AS ENUM (
    'madre',
    'padre',
    'tutor_legal'
);

CREATE TYPE tipo_tutor_de_persona AS ENUM (
    'principal',
    'segundo',
    'tercerp'
);

CREATE TABLE tutores_de_persona (
    id_tutor_de_persona SERIAL PRIMARY KEY,
    id_persona INTEGER NOT NULL,
    tutor INTEGER NOT NULL,
    tutor_migrado VARCHAR(60) NOT NULL,
    quien_es_tutor quien_es_tutor_de_persona NOT NULL,
    documento_legal VARCHAR(60) NOT NULL,
    documento INTEGER NOT NULL,
    tipo tipo_tutor_de_persona NOT NULL,
    FOREIGN KEY (id_persona) REFERENCES personas (id_persona) ON DELETE RESTRICT,
    FOREIGN KEY (tutor) REFERENCES personas (id_persona) ON DELETE RESTRICT,
    FOREIGN KEY (documento) REFERENCES documentos (id_documento) ON DELETE RESTRICT
);
