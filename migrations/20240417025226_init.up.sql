CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL
);

INSERT INTO
    roles (nombre)
VALUES
    ('admin'),
    ('normal');


CREATE OR REPLACE FUNCTION obtener_id_rol_normal() RETURNS INT AS $$
DECLARE
    id_rol INT;
BEGIN
    SELECT id INTO id_rol FROM roles WHERE nombre = 'normal' LIMIT 1;
    RETURN id_rol;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE usuarios (
    id SERIAL PRIMARY KEY,
    usuario VARCHAR(100) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL,
    contraseña VARCHAR(100) NOT NULL,
    id_rol INT NOT NULL DEFAULT obtener_id_rol_normal(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    FOREIGN KEY (id_rol) REFERENCES roles(id)
);

CREATE INDEX usuarios_usuario_idx ON usuarios (usuario);