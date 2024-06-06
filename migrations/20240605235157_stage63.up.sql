CREATE TABLE grupos_finalidad_prestamo (
    id_grupo_finalidad_prestamo SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL,
    monto_minimo REAL NOT NULL,
    monto_maximo REAL NOT NULL,
    porcentaje_capital_neto REAL NOT NULL,
    plazo_minimo INTEGER NOT NULL,
    plazo_maximo INTEGER NOT NULL,
    tasa_normal REAL NOT NULL,
    tantos_tasa_moratoria REAL NOT NULL,
    activo BOOLEAN NOT NULL
);

INSERT INTO grupos_finalidad_prestamo (
    id_grupo_finalidad_prestamo, nombre, monto_minimo, monto_maximo, porcentaje_capital_neto, plazo_minimo, plazo_maximo, tasa_normal, tantos_tasa_moratoria, activo
) VALUES
(1,	'CREDITOS AL CONSUMO',	500.00,	0.00,	3.00,	6,	96,	30.00,	12.00,	true),
(2,	'CREDITOS COMERCIALES',	500.00,	0.00,	3.00,	6,	96,	21.00,	12.00,	true),
(3,	'CREDITOS A LA VIVIENDA',	500.00,	0.00,	3.00,	6,	96,	23.00,	12.00,	true);
