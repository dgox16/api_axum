
-- Add up migration script here
CREATE TABLE prestamos (
    id_prestamo SERIAL PRIMARY KEY,
    -- Prestamo anlisis capacidad
    firma_a_ruego VARCHAR(50) NOT NULL,
    _cartera_pasiva INTEGER NOT NULL,
    capacidad_pago REAL NOT NULL,
    capacidad_pago_ajustada REAL NOT NULL,
    independencia_financiera REAL NOT NULL,
    solvencia_general REAL NOT NULL,
    indice_de_liquidez REAL NOT NULL,
    rentabilidad REAL NOT NULL,
    grado_de_riesgo grado_riesgo_prestamo NOT NULL,
    -- Prestamo anterior
    incumplimiento_credito_anterior INTEGER NOT NULL,
    periodo_pactado_original INTEGER NOT NULL,
    -- Prestamo Ultimo calculo
    dias_mora INTEGER NOT NULL,
    interes_moratorio REAL NOT NULL,
    interes_normal REAL NOT NULL,
    fecha_pago DATE NOT NULL,
    saldo_capital REAL NOT NULL,
    observaciones VARCHAR(300) NOT NULL,
    -- Prestamo estructura datos
    monto_original_antes_de_reestructura REAL NOT NULL,
    adelantar BOOLEAN NOT NULL,
    anticipado BOOLEAN NOT NULL,
    -- Prestamo legal
    esta_en_litigio BOOLEAN NOT NULL,
    emproblemado VARCHAR(50) NOT NULL,
    -- Prestamo adicional
    modificado INTEGER NOT NULL,
    modalidad VARCHAR(25) NOT NULL,
    tasa_subsidiada REAL NOT NULL,
    meses_de_gracia INTEGER NOT NULL,
    -- Prestamo impresiones
    solicitud_impresa BOOLEAN NOT NULL
);
