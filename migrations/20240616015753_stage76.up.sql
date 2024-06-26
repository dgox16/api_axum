CREATE TABLE detalles_ficha_temporal (
    id_detalle_ficha_temporal SERIAL PRIMARY KEY,
    persona INTEGER NOT NULL,
    servicio INTEGER,
    captacion INTEGER,
    prestamo INTEGER,
    inversion INTEGER,
    cargo REAL NOT NULL,
    abono REAL NOT NULL,
    FOREIGN KEY (persona) REFERENCES personas(id_persona) ON DELETE RESTRICT,
    FOREIGN KEY (servicio) REFERENCES servicios(id_servicio) ON DELETE RESTRICT,
    FOREIGN KEY (captacion) REFERENCES contratos_captacion(id_contrato_captacion) ON DELETE RESTRICT,
    FOREIGN KEY (prestamo) REFERENCES prestamos(id_prestamo) ON DELETE RESTRICT,
    FOREIGN KEY (inversion) REFERENCES inversiones(id_inversion) ON DELETE RESTRICT
);
