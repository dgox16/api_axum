CREATE TYPE clasificacion_persona AS ENUM (
    'fisica',
    'moral'
);

CREATE TYPE periodo_persona AS ENUM (
    'catorcenal',
    'semanal',
    'quincenal',
    'mensual'
);

CREATE TABLE aspirantes_persona (
    id_persona_aspirante SERIAL PRIMARY KEY,
    id_persona INTEGER NOT NULL,
    clasificacion clasificacion_persona NOT NULL,
    ocupacion_pld INTEGER NOT NULL,
    especificacion_pld INTEGER NOT NULL,
    actividad_pld INTEGER NOT NULL,
    antiguedad INTEGER NOT NULL,
    periodo periodo_persona NOT NULL,
    frecuencia_captacion INTEGER NOT NULL,
    operacion_maxima_captacion REAL NOT NULL,
    perfil_frecuencia_prestamo INTEGER NOT NULL,
    operacion_maxima_prestamo REAL NOT NULL,
    ingresos_mensual REAL NOT NULL,
    egresos_mensual REAL NOT NULL,
    grado_afectacion INTEGER NOT NULL,
    afectacion REAL NOT NULL,
    entre_calle BIGINT NOT NULL,
    y_calle BIGINT NOT NULL,
    fecha_residencia DATE NOT NULL,
    lugar_nacimiento VARCHAR(75) NOT NULL,
    estado_nacimiento INTEGER NOT NULL,
    profesion INTEGER NOT NULL,
    escolaridad INTEGER NOT NULL,
    autorizo_compartir_informacion_ifai BOOLEAN NOT NULL,
    autorizo_publicidad BOOLEAN NOT NULL,
    FOREIGN KEY (id_persona) REFERENCES personas (id_persona) ON DELETE RESTRICT,
    FOREIGN KEY (ocupacion_pld) REFERENCES ocupaciones_pld (id_ocupacion_pld) ON DELETE RESTRICT,
    FOREIGN KEY (especificacion_pld) REFERENCES especificaciones_pld (id_especificacion_pld) ON DELETE RESTRICT,
    FOREIGN KEY (actividad_pld) REFERENCES actividades_pld (id_actividad_pld) ON DELETE RESTRICT,
    FOREIGN KEY (antiguedad) REFERENCES lavado_antiguedad (id_lavado_antiguedad) ON DELETE RESTRICT,
    FOREIGN KEY (entre_calle) REFERENCES calles (id_calle) ON DELETE RESTRICT,
    FOREIGN KEY (y_calle) REFERENCES calles (id_calle) ON DELETE RESTRICT,
    FOREIGN KEY (profesion) REFERENCES profesiones (id_profesion) ON DELETE RESTRICT,
    FOREIGN KEY (escolaridad) REFERENCES escolaridades (id_escolaridad) ON DELETE RESTRICT
);
