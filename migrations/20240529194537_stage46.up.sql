CREATE TABLE relaciones_de_persona (
    id_relacion_de_persona SERIAL PRIMARY KEY,
    id_persona INTEGER NOT NULL,
    persona_relacionada INTEGER NOT NULL,
    parentesco INTEGER NOT NULL,
    FOREIGN KEY (id_persona) REFERENCES personas (id_persona) ON DELETE RESTRICT,
    FOREIGN KEY (persona_relacionada) REFERENCES personas (id_persona) ON DELETE RESTRICT,
    FOREIGN KEY (parentesco) REFERENCES parentescos (id_parentesco) ON DELETE RESTRICT
);
