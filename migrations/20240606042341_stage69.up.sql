CREATE TYPE riesgo_calificacion_buro AS ENUM (
    'riesgo_bajo',
    'riesgo_medio_bajo',
    'riesgo_medio',
    'riesgo_medio_alto',
    'riesgo_alto'
);

CREATE TABLE calificaciones_buro (
    id_calificacion_buro SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL,
    riesgo riesgo_calificacion_buro NOT NULL,
    ponderacion_5c REAL NOT NULL
);

INSERT INTO calificaciones_buro (id_calificacion_buro, nombre, riesgo, ponderacion_5c) VALUES
(1,	'CUENTA MUY RECIENTE PARA SER CALIFICADA',	'riesgo_medio',	0.00),
(2,	'CUENTA CON PAGO PUNTUAL Y ADECUADO',	'riesgo_bajo',	100.00),
(3,	'CUENTA CON ATRASO DE 1 A 29 DÍAS',	'riesgo_medio_bajo',	80.00),
(4,	'CUENTA CON ATRASO DE 30 A 59 DÍAS',	'riesgo_medio_bajo',	60.00),
(5,	'CUENTA CON ATRASO DE 60 A 89 DÍAS',	'riesgo_medio',	40.00),
(6,	'CUENTA CON ATRASO DE 90 A 119 DÍAS',	'riesgo_medio',	20.00),
(7,	'CUENTA CON ATRASO DE 120 A 149 DÍAS',	'riesgo_alto',	0.00),
(8,	'CUENTA CON ATRASO DE 150 DÍAS HASTA 12 MESES',	'riesgo_alto',	0.00),
(9,	'CUENTA CON ATRASO DE MÁS DE 12 MESES',	'riesgo_alto',	0.00),
(10,	'CUENTA CON DEUDA PARCIAL O TOTAL SIN RECUPERAR',	'riesgo_alto',	0.00),
(11,	'FRAUDE COMETIDO POR EL CONSUMIDOR',	'riesgo_alto',	0.00),
(12,	'INFORMACIÓN ANULADA A SOLICITUD DEL OTORGANTE',	'riesgo_bajo',	0.00),
(13,	'OTRAS CALIFICACIONES',	'riesgo_alto',	0.00),
(14,	'CUENTA NO CALIFICADA',	'riesgo_alto',	0.00);

CREATE TABLE prestamos_crediticio (
    id_prestamo_crediticio SERIAL PRIMARY KEY,
    fecha_buro DATE NOT NULL,
    folio_buro VARCHAR(15) NOT NULL,
    calificacion_buro INTEGER NOT NULL,
    bc_score INTEGER NOT NULL,
    indice_capacidad_crediticia INTEGER NOT NULL,
    FOREIGN KEY (calificacion_buro) REFERENCES calificaciones_buro(id_calificacion_buro) ON DELETE RESTRICT
);
