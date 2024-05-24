CREATE TABLE menores_persona (
    id_persona_menor SERIAL PRIMARY KEY,
    id_persona INTEGER NOT NULL,
    entre_calle BIGINT NOT NULL,
    y_calle BIGINT NOT NULL,
    fecha_residencia DATE NOT NULL,
    lugar_nacimiento VARCHAR(75) NOT NULL,
    estado_nacimiento INTEGER NOT NULL,
    escolaridad INTEGER NOT NULL,
    autorizo_compartir_informacion_ifai BOOLEAN NOT NULL,
    autorizo_publicidad BOOLEAN NOT NULL,
    FOREIGN KEY (id_persona) REFERENCES personas (id_persona) ON DELETE RESTRICT,
    FOREIGN KEY (entre_calle) REFERENCES calles (id_calle) ON DELETE RESTRICT,
    FOREIGN KEY (y_calle) REFERENCES calles (id_calle) ON DELETE RESTRICT,
    FOREIGN KEY (estado_nacimiento) REFERENCES estados_mexico (id_estado) ON DELETE RESTRICT,
    FOREIGN KEY (escolaridad) REFERENCES escolaridades (id_escolaridad) ON DELETE RESTRICT
);
