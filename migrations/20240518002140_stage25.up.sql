CREATE TYPE grado_parentesco AS ENUM ('primero', 'segundo', 'tercero', 'no_aplica');

CREATE TABLE parentescos (
    id_parentesco SERIAL PRIMARY KEY,
    nombre VARCHAR(30) NOT NULL,
    es_familiar BOOLEAN NOT NULL,
    grado grado_parentesco NOT NULL,
    tipo_id INTEGER NOT NULL
);

INSERT INTO parentescos (id_parentesco, nombre, es_familiar, grado, tipo_id) VALUES
(1,	' Seleccionar',	FALSE,	'no_aplica',	0),
(2,	'CONCUBINO(A)',	TRUE,	'primero',	0),
(3,	'CONYUGE',	TRUE,	'primero',	3),
(4,	'HIJO(A)',	TRUE,	'primero',	9),
(5,	'HIJO(A) ADOPTIVO',	TRUE,	'primero',	0),
(6,	'HIJO(A) POLITICO AFINIDAD',	TRUE,	'primero',	0),
(7,	'MADRE',	TRUE,	'primero',	2),
(9,	'NUERA',	TRUE,	'primero',	0),
(10,	'PADRE',	TRUE,	'primero',	1),
(12,	'SUEGRO(A)',	TRUE,	'primero',	0),
(13,	'TUTOR',	FALSE,	'segundo',	10),
(14,	'YERNO',	TRUE,	'primero',	0),
(15,	'ABUELO(A)',	TRUE,	'primero',	8),
(16,	'ABUELO(A) POLITICO AFINIDAD',	TRUE,	'segundo',	0),
(17,	'CUÑADO(A)',	TRUE,	'segundo',	7),
(18,	'HERMANO(A)',	TRUE,	'primero',	4),
(19,	'NIETO(A)',	TRUE,	'segundo',	12),
(20,	'NIETO(A) POLITICO AFINIDAD',	TRUE,	'primero',	0),
(21,	'PRIMO(A)',	TRUE,	'tercero',	0),
(27,	'SOBRINO(A)',	TRUE,	'segundo',	11),
(28,	'TIO(A)',	TRUE,	'segundo',	5),
(29,	'AMIGO(A)',	FALSE,	'no_aplica',	0),
(30,	'CONOCIDO(A)',	FALSE,	'no_aplica',	0),
(31,	'COMPAÑERO(A)',	FALSE,	'no_aplica',	0),
(32,	'NOVIO(A)',	FALSE,	'no_aplica',	0),
(33,	'SOCIO(A)',	FALSE,	'no_aplica',	0),
(34,	'BISNIETO(A)',	TRUE,	'segundo',	13),
(35,	'PADRASTRO',	TRUE,	'primero',	14),
(36,	'MADRASTRA',	TRUE,	'primero',	15),
(37,	'HIJASTRO(A)',	TRUE,	'primero',	16),
(38,	'HERMANASTRO(A)',	TRUE,	'primero',	17),
(39,	'MEDIO HERMANO(A)',	TRUE,	'primero',	18);

