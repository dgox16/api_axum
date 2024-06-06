CREATE TYPE tiene_seguro_prestamo AS ENUM (
    'no',
    'si',
    'no_aplica',
    'hipoteca',
    'avales'
);

CREATE TABLE prestamos_seguro (
    id_prestamo_seguro SERIAL PRIMARY KEY,
    tiene_seguro tiene_seguro_prestamo NOT NULL,
    aseguradora INTEGER,
    afectacion REAL NOT NULL,
    tasa_interes REAL NOT NULL,
    tasa_moratoria REAL NOT NULL,
    tasa_iva REAL NOT NULL,
    pagos_otras_deudas REAL NOT NULL,
    interes_otras_deudas REAL NOT NULL,
    FOREIGN KEY (aseguradora) REFERENCES aseguradoras(id_aseguradora) ON DELETE RESTRICT
);
