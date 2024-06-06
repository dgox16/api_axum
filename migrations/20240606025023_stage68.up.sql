CREATE TABLE prestamos_autorizacion (
    id_prestamo_autorizacion SERIAL PRIMARY KEY,
    id_autoriza INTEGER NOT NULL,
    autorizo_gerente BOOLEAN NOT NULL,
    comision REAL NOT NULL
);

CREATE TABLE prestamos_entrega (
    id_prestamo_entrega SERIAL PRIMARY KEY,
    poliza INTEGER NOT NULL,
    ficha INTEGER NOT NULL,
    cat REAL NOT NULL,
    FOREIGN KEY (poliza) REFERENCES polizas(id_poliza) ON DELETE RESTRICT,
    FOREIGN KEY (ficha) REFERENCES fichas(id_ficha) ON DELETE RESTRICT
);

CREATE TABLE prestamos_garantias (
    id_prestamo_garantias SERIAL PRIMARY KEY,
    id_prestamo_relacionado INTEGER NOT NULL,
    porcentaje_reciprocidad REAL NOT NULL,
    garantia_fija BOOLEAN NOT NULL,
    garantia_liquida REAL NOT NULL,
    id_servicio_garantia_liquida INTEGER NOT NULL,
    id_captacion_garantia INTEGER NOT NULL,
    id_inversion_garantia INTEGER NOT NULL,
    monto_garantia_liquida REAL NOT NULL,
    garantia_prendaria REAL NOT NULL,
    garantia_hipotecaria REAL NOT NULL,
    garantia_avales REAL NOT NULL
);

CREATE TYPE clasificacion_prestamo AS ENUM (
    'nuevo',
    'renovado',
    'estructurado'
);

CREATE TYPE estrato_prestamo AS ENUM (
    'no_aplica',
    'E1',
    'E2',
    'E3',
    'E4',
    'E5'
);

CREATE TYPE subclasificacion_prestamo AS ENUM (
    'normal',
    'micro_credito'
);

CREATE TABLE prestamos_documentos (
    id_prestamo_documentos SERIAL PRIMARY KEY,
    compromiso_ahorro REAL NOT NULL,
    folio INTEGER NOT NULL,
    contrato INTEGER NOT NULL,
    pagare INTEGER NOT NULL,
    opinion_analista VARCHAR(200) NOT NULL,
    clasificacion clasificacion_prestamo NOT NULL, 
    estrato estrato_prestamo NOT NULL,
    subclasificacion subclasificacion_prestamo NOT NULL,
    id_prestamo_migrado VARCHAR(16) NOT NULL,
    reestructurado BOOLEAN NOT NULL,
    pagos_sostenidos INTEGER NOT NULL
);
