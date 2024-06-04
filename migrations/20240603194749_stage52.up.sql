CREATE TABLE servicios_subgrupo (
    id_servicio_subgrupo SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL
);

CREATE TABLE plazos (
    id_plazo SERIAL PRIMARY KEY,
    nombre VARCHAR(25) NOT NULL,
    nombre_2 VARCHAR(50) NOT NULL,
    dias INTEGER NOT NULL,
    meses INTEGER NOT NULL,
    prestamo INTEGER NOT NULL,
    inversiones INTEGER NOT NULL,
    ponderacion_5c_abonos REAL NOT NULL,
    ponderacion_5c_plazo REAL NOT NULL
);

CREATE TABLE grupos_finalidad (
    id_grupo_finalidad SERIAL PRIMARY KEY,
    nombre VARCHAR(20) NOT NULL
);

INSERT INTO grupos_finalidad (nombre) VALUES 
('PERSONALES'),
('FAMILIARES'),
('TRABAJO'),
('AGRICULTURA'),
('GANADERIA'),
('PESCA'),
('MAQUILA'),
('OTROS');


INSERT INTO plazos (id_plazo, nombre, nombre_2, dias, meses, prestamo, inversiones, ponderacion_5c_abonos, ponderacion_5c_plazo) VALUES
(1,	'DIARIO',	'DIARIOS',	1,	0,	0,	0,	60.00,	100.00),
(2,	'SEMANAL',	'SEMANAL (ES)',	7,	0,	1,	0,	60.00,	100.00),
(3,	'CATORCENA',	'CATORCENAL (ES)',	14,	0,	0,	0,	60.00,	100.00),
(5,	'1 MES ',	'MENSUAL (ES)',	30,	1,	1,	1,	60.00,	100.00),
(6,	'2 MESES',	'BIMESTRAL (ES)',	60,	2,	1,	1,	70.00,	100.00),
(7,	'3 MESES',	'TRIMESTRAL (ES)',	90,	3,	1,	1,	70.00,	100.00),
(8,	'6 MESES',	'SEMESTRAL (ES)',	180,	6,	1,	1,	100.00,	100.00),
(9,	'1 AÑO',	'ANUAL (ES)',	365,	12,	1,	1,	40.00,	90.00),
(10,	'INDEFINIDO',	'AL VENCIMIENTO',	9999,	0,	0,	0,	40.00,	30.00),
(11,	'2 AÑOS',	'2 AÑOS',	720,	24,	1,	0,	40.00,	90.00),
(12,	'3 AÑOS',	'3 AÑOS',	1080,	36,	1,	0,	40.00,	80.00),
(13,	'QUINCENAL',	'QUINCENALES',	15,	0,	1,	0,	60.00,	100.00),
(14,	'270 DIAS',	'270 DIAS',	270,	9,	0,	0,	100.00,	100.00),
(15,	'18 MESES',	'18 MESES',	540,	18,	1,	0,	40.00,	90.00),
(16,	'1 AÑO',	'ANUALES',	360,	12,	0,	0,	40.00,	90.00),
(17,	'8 MESES',	'8 MESES',	240,	8,	1,	0,	100.00,	100.00),
(18,	'10 MESES',	'10 MESES',	300,	10,	1,	0,	40.00,	90.00),
(19,	'4 MESES ',	'CUATRIMESTRAL (ES)',	120,	4,	1,	0,	100.00,	100.00),
(20,	'11 MESES',	'11 MESES',	330,	11,	1,	0,	40.00,	90.00),
(21,	'7 MESES',	'7 MESES',	210,	7,	1,	0,	100.00,	100.00),
(22,	'9 MESES',	'9 MESES',	270,	9,	1,	0,	100.00,	100.00),
(23,	'5 MESES',	'5 MESES',	150,	5,	1,	0,	100.00,	100.00),
(24,	'28 DIAS',	'28 DIAS',	28,	0,	0,	0,	60.00,	100.00),
(25,	'14 MESES',	'14 MESES',	425,	14,	1,	0,	40.00,	90.00),
(26,	'4 AÑOS',	'4 AÑOS',	1460,	48,	1,	0,	40.00,	80.00),
(27,	'5 AÑOS',	'5 AÑOS',	1825,	60,	1,	0,	40.00,	80.00),
(28,	'6 AÑOS',	'6 AÑOS',	2190,	72,	1,	0,	40.00,	80.00),
(29,	'7 AÑOS',	'7 AÑOS',	2555,	84,	1,	0,	40.00,	80.00),
(30,	'8 AÑOS',	'8 AÑOS',	2920,	96,	1,	0,	40.00,	80.00),
(31,	'9 AÑOS',	'9 AÑOS',	3285,	108,	1,	0,	40.00,	80.00),
(32,	'10 AÑOS',	'10 AÑOS',	3650,	120,	1,	0,	40.00,	80.00),
(33,	'11 AÑOS',	'11 AÑOS',	4015,	132,	1,	0,	40.00,	80.00),
(34,	'12 AÑOS',	'12 AÑOS',	4380,	144,	1,	0,	40.00,	80.00),
(35,	'13 AÑOS',	'13 AÑOS',	4745,	156,	0,	0,	40.00,	80.00),
(36,	'14 AÑOS',	'14 AÑOS',	5110,	168,	1,	0,	40.00,	80.00),
(37,	'15 AÑOS',	'15 AÑOS',	5475,	180,	1,	0,	40.00,	80.00),
(38,	'20 AÑOS',	'20 AÑOS',	7305,	240,	1,	0,	40.00,	80.00);

