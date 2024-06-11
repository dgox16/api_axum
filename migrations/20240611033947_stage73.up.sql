CREATE TABLE prestamos_anterior (
    id_prestamo_anterior SERIAL PRIMARY KEY,
    incumplimiento_credito_anterior INTEGER NOT NULL,
    periodo_pactado_original INTEGER NOT NULL
);

CREATE TABLE prestamos_ultimo_calculo (
    id_prestamo_ultimo_calculo SERIAL PRIMARY KEY,
    dias_mora INTEGER NOT NULL,
    interes_moratorio REAL NOT NULL,
    interes_normal REAL NOT NULL,
    fecha_pago DATE NOT NULL,
    saldo_capital REAL NOT NULL,
    observaciones VARCHAR(300) NOT NULL
);

CREATE TABLE prestamos_estructura_datos (
    id_prestamo_estructura_datos SERIAL PRIMARY KEY,
    monto_original_antes_de_reestructura REAL NOT NULL,
    adelantar BOOLEAN NOT NULL,
    anticipado BOOLEAN NOT NULL
);

CREATE TABLE prestamos_legal (
    id_prestamo_legal SERIAL PRIMARY KEY,
    esta_en_litigio BOOLEAN NOT NULL,
    emproblemado VARCHAR(50) NOT NULL
);

CREATE TABLE prestamos_adicional (
    id_prestamo_adicional SERIAL PRIMARY KEY,
    modificado INTEGER NOT NULL,
    modalidad VARCHAR(25) NOT NULL,
    tasa_subsidiada REAL NOT NULL,
    meses_de_gracia INTEGER NOT NULL
);

CREATE TABLE prestamos_impresiones (
    id_prestamo_impresiones SERIAL PRIMARY KEY,
    solicitud_impresa BOOLEAN NOT NULL
);
