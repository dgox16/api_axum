CREATE TABLE conyuges_persona (
    id_persona_conyuge SERIAL PRIMARY KEY,
    id_persona INTEGER NOT NULL,
    fecha_residencia DATE NOT NULL,
    lugar_nacimiento VARCHAR(75) NOT NULL,
    estado_nacimiento INTEGER NOT NULL,
    regimen_conyugal regimen_conyugal_persona NOT NULL,
    FOREIGN KEY (id_persona) REFERENCES personas (id_persona) ON DELETE RESTRICT,
    FOREIGN KEY (estado_nacimiento) REFERENCES estados_mexico (id_estado) ON DELETE RESTRICT
);
