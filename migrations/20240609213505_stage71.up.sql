CREATE TYPE estrato_cartera_pasiva AS ENUM (
    'e1',
    'e2',
    'e3',
    'e4',
    'e5'
);

CREATE TYPE clasificacion_fira_cartera_pasiva AS ENUM (
    'avio',
    'refaccionario',
    'prendario'
);

CREATE TYPE etapa_cartera_pasiva AS ENUM (
    'solicitud',
    'activa',
    'pagada'
);

CREATE TYPE tipo_de_sujeto_de_credito_cartera_pasiva AS ENUM (
    'persona_fisica',
    'persona_moral'
);

CREATE TYPE tipo_tenencia_cartera_pasiva AS ENUM (
    'ejidal',
    'pequeña_propiedad',
    'mixta',
    'comunal',
    'otras'
);

CREATE TYPE denominacion_cartera_pasiva AS ENUM (
    'pesos',
    'dolares'
);

CREATE TYPE tasa_cartera_pasiva AS ENUM (
    'tiie',
    'cetes'
);

CREATE TABLE carteras_pasiva (
    id_cartera_pasiva SERIAL PRIMARY KEY,
    persona INTEGER NOT NULL,
    sucursal INTEGER NOT NULL,
    servicio INTEGER NOT NULL,
    credito_fira INTEGER NOT NULL,
    direccion_regional INTEGER NOT NULL,
    residencia_estatal INTEGER NOT NULL,
    agencia INTEGER NOT NULL,
    evaluador INTEGER NOT NULL,
    dtuid INTEGER NOT NULL,
    fecha_inicial DATE NOT NULL,
    fecha_descuento DATE NOT NULL,
    fecha_vencimiento DATE NOT NULL,
    fecha_interes_normal DATE NOT NULL,
    fecha_interes_moratorio DATE NOT NULL,
    fecha_pagada DATE NOT NULL,
    numero_pagos INTEGER NOT NULL,
    plazo INTEGER NOT NULL,
    estrato estrato_cartera_pasiva NOT NULL,
    clasificacion_fira clasificacion_fira_cartera_pasiva NOT NULL,
    actividad_fira INTEGER NOT NULL,
    subrama INTEGER NOT NULL,
    cadena INTEGER NOT NULL,
    concepto_fira INTEGER NOT NULL,
    etapa etapa_cartera_pasiva NOT NULL,
    monto REAL NOT NULL,
    meses_recuperacion INTEGER NOT NULL,
    seguro INTEGER NOT NULL,
    clave INTEGER NOT NULL,
    garantia_fija REAL NOT NULL,
    tipo_garantia INTEGER NOT NULL,
    clave_garantia INTEGER NOT NULL,
    procampo BOOLEAN NOT NULL,
    garantia_acreditado REAL NOT NULL,
    garantia_otros REAL NOT NULL,
    clave_autorizacion INTEGER NOT NULL,
    analisis_riesgo VARCHAR(50) NOT NULL,
    exito_esperado REAL NOT NULL,
    solicitante VARCHAR(70) NOT NULL,
    fira INTEGER NOT NULL,
    ubicacion_predio VARCHAR(70) NOT NULL,
    clave_predio INTEGER NOT NULL,
    condiciones_especificas VARCHAR(70) NOT NULL,
    tipo_de_sujeto_de_credito tipo_de_sujeto_de_credito_cartera_pasiva NOT NULL,
    tipo_avio INTEGER NOT NULL,
    tipo_tenencia tipo_tenencia_cartera_pasiva NOT NULL, 
    necesidad_anual INTEGER NOT NULL,
    denominacion denominacion_cartera_pasiva NOT NULL,
    activo_total REAL NOT NULL,
    activo_fijo REAL NOT NULL,
    pasivo_total REAL NOT NULL,
    pasivo_fijo REAL NOT NULL,
    tasa tasa_cartera_pasiva NOT NULL,
    puntos REAL NOT NULL,
    tasa_moratoria REAL NOT NULL,
    FOREIGN KEY (persona) REFERENCES personas(id_persona) ON DELETE RESTRICT,
    FOREIGN KEY (sucursal) REFERENCES sucursales(id_sucursal) ON DELETE RESTRICT,
    FOREIGN KEY (servicio) REFERENCES servicios(id_servicio) ON DELETE RESTRICT,
    FOREIGN KEY (plazo) REFERENCES plazos(id_plazo) ON DELETE RESTRICT,
    FOREIGN KEY (concepto_fira) REFERENCES conceptos_fira(id_concepto_fira) ON DELETE RESTRICT
    
);


