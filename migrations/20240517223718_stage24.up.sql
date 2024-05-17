CREATE TABLE lavado_antiguedad (
    id_lavado_antiguedad SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL,
    antiguedad INTEGER NOT NULL,
    factor_riesgo REAL NOT NULL
);

INSERT INTO lavado_antiguedad (id_lavado_antiguedad, nombre, antiguedad, factor_riesgo) VALUES
(1,	'Menor a 1 año en la actividad',	1,	3.00),
(2,	'Mayor a 1 año y menor a 3 años en la actividad',	3,	2.00),
(3,	'Mayor a 3 año y menor a 5 años en la actividad',	5,	2.00),
(4,	'Mayor a 5 años y menor a 10  años en la actividad',	10,	1.00),
(5,	'Mayor a 10 años en la actividad',	999,	1.00),
(6,	'No aplica / No hay datos',	0,	1.51);

