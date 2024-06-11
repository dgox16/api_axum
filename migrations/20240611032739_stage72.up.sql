CREATE TYPE grado_riesgo_prestamo AS ENUM (
    'bajo',
    'medio',
    'alto'
);

CREATE TABLE prestamos_analisis_capacidad (
    id_prestamo_analisis_capacidad SERIAL PRIMARY KEY,
    firma_a_ruego VARCHAR(50) NOT NULL,
    cartera_pasiva INTEGER NOT NULL,
    capacidad_pago REAL NOT NULL,
    capacidad_pago_ajustada REAL NOT NULL,
    independencia_financiera REAL NOT NULL,
    solvencia_general REAL NOT NULL,
    indice_de_liquidez REAL NOT NULL,
    rentabilidad REAL NOT NULL,
    grado_de_riesgo grado_riesgo_prestamo NOT NULL,
    FOREIGN KEY (cartera_pasiva) REFERENCES carteras_pasiva(id_cartera_pasiva) ON DELETE RESTRICT
);
