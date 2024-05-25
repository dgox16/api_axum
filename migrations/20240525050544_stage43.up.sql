CREATE TABLE beneficiarios_persona (
    id_persona_beneficiario SERIAL PRIMARY KEY,
    id_persona INTEGER NOT NULL,
    entre_calle BIGINT NOT NULL,
    y_calle BIGINT NOT NULL,
    FOREIGN KEY (id_persona) REFERENCES personas (id_persona) ON DELETE RESTRICT,
    FOREIGN KEY (entre_calle) REFERENCES calles (id_calle) ON DELETE RESTRICT,
    FOREIGN KEY (y_calle) REFERENCES calles (id_calle) ON DELETE RESTRICT
);
