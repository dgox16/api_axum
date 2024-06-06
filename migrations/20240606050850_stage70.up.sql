CREATE TABLE formas_liquidacion (
    id_forma_liquidacion SERIAL PRIMARY KEY,
    nombre VARCHAR(70) NOT NULL,
    factor_riesgo REAL NOT NULL
);

INSERT INTO formas_liquidacion (id_forma_liquidacion, nombre, factor_riesgo) VALUES
(1,	'NO APLICA / NO HAY DATOS',	1.00),
(2,	'DOCUMENTOS',	1.00),
(3,	'EFECTIVO MAYOR A $200,000',	3.00),
(4,	'EFECTIVO MÁS DE 100,000 HASTA 200,000',	3.00),
(5,	'EFECTIVO MÁS DE 50,000 HASTA 100,000',	2.00),
(6,	'EFECTIVO HASTA $50,000',	1.00),
(7,	'TRANSFERENCIAS BANCARIAS HASTA $100,000',	1.00),
(8,	'TRANSFERENCIAS BANCARIAS MÁS DE $100,000 A $200,000',	2.00),
(9,	'TRANSFERENCIAS BANCARIAS MÁS DE $200,000 A $300,000',	2.00),
(10, 'TRANSFERENCIAS BANCARIAS MAYORES A $300,000',	3.00);

CREATE TABLE antecedentes_crediticio (
    id_antecedente_crediticio SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL,
    factor_riesgo REAL NOT NULL
);

INSERT INTO antecedentes_crediticio (id_antecedente_crediticio, nombre, factor_riesgo) VALUES
(1,	'No aplica / No hay datos',	1.00),
(2,	'Experiencia crediticia negativa',	3.00),
(3,	'Experiencia crediticia positiva',	1.00),
(4,	'Sin experiencia crediticia',	2.00);

CREATE TABLE prestamos_lavado (
    id_prestamo_lavado SERIAL PRIMARY KEY,
    forma_liquidacion INTEGER NOT NULL,
    antecedente_crediticio INTEGER NOT NULL,
    tipo_de_cliente INTEGER NOT NULL,
    observacion_credito INTEGER NOT NULL,
    FOREIGN KEY (forma_liquidacion) REFERENCES formas_liquidacion(id_forma_liquidacion) ON DELETE RESTRICT,
    FOREIGN KEY (antecedente_crediticio) REFERENCES antecedentes_crediticio(id_antecedente_crediticio) ON DELETE RESTRICT

);
