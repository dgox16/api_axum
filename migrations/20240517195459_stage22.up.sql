CREATE TABLE escolaridades (
    id_escolaridad SERIAL PRIMARY KEY,
    nombre VARCHAR(60) NOT NULL
);

INSERT INTO escolaridades (id_escolaridad, nombre) VALUES
(1,	'MATERNAL'),
(2,	'PREESCOLAR'),
(3,	'PRIMARIA'),
(4,	'SECUNDARIA'),
(5,	'PREPARATORIA'),
(6,	'LICENCIATURA'),
(7,	'MAESTRIA'),
(8,	'DOCTORADO'),
(9,	'SIN ESTUDIOS');

