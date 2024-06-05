CREATE TYPE tipo_fuente_financiamiento AS ENUM (
    'recursos_propios',
    'fideocomiso',
    'banca_comercial',
    'otros'
);

CREATE TABLE fuentes_financiamiento (
    id_fuente_financiamiento SERIAL PRIMARY KEY,
    nombre VARCHAR(70) NOT NULL,
    tipo tipo_fuente_financiamiento NOT NULL,
    puntos REAL NOT NULL
);

INSERT INTO fuentes_financiamiento (id_fuente_financiamiento, nombre, tipo, puntos) VALUES
(1,	'RECURSOS PROPIOS',	'recursos_propios',	0.00),
(2,	'FIRA',	'fideocomiso',	1.45),
(3,	'BANCA MIFEL',	'banca_comercial',	0.00),
(4,	'BANCO DEL BAJIO',	'banca_comercial',	0.00);


CREATE TABLE prestamos_linea_credito (
    id_prestamo_linea_credito SERIAL PRIMARY KEY,
    linea_de_credito REAL NOT NULL,
    ultima_ministracion REAL NOT NULL,
    monto REAL NOT NULL,
    numero_abonos INTEGER NOT NULL,
    plazo INTEGER NOT NULL,
    servicio INTEGER NOT NULL,
    fuente_financiamiento INTEGER NOT NULL,
    FOREIGN KEY (plazo) REFERENCES plazos(id_plazo) ON DELETE RESTRICT,
    FOREIGN KEY (servicio) REFERENCES servicios(id_servicio) ON DELETE RESTRICT,
    FOREIGN KEY (fuente_financiamiento) REFERENCES fuentes_financiamiento(id_fuente_financiamiento) ON DELETE RESTRICT
);

