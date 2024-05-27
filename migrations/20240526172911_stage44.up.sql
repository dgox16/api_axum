CREATE TYPE tipo_contacto_de_persona AS ENUM (
    'telefono_fijo',
    'telefono_movil',
    'correo_electronico',
    'facebook',
    'whatsapp'
);

CREATE TABLE contactos_de_persona (
    id_contacto_de_persona SERIAL PRIMARY KEY,
    id_persona INTEGER NOT NULL,
    contacto VARCHAR(70) NOT NULL,
    tipo tipo_contacto_de_persona NOT NULL,
    es_principal BOOLEAN NOT NULL,
    FOREIGN KEY (id_persona) REFERENCES personas (id_persona) ON DELETE RESTRICT
);
