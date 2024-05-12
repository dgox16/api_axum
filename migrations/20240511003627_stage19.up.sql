CREATE TABLE tipos_persona (
    id_tipo_persona SERIAL PRIMARY KEY,
    nombre VARCHAR(25) NOT NULL
);

INSERT INTO tipos_persona (id_tipo_persona, nombre) VALUES
(1,	'Aspirante'),
(2,	'Socio'),
(3,	'Aval Externo'),
(4,	'Menor'),
(5,	'Conyuge/Concubino'),
(6,	'Cliente'),
(7,	'Sucursal'),
(8,	'Tercero autorizado'),
(9, 'Propietario Real'),
(10, 'Proveedor de recursos'),
(11, 'Tutor'),
(12, 'Referencia'),
(13, 'Beneficiario');


CREATE TYPE sexo_persona AS ENUM (
    'masculino',
    'femenino',
    'persona_moral'
);

CREATE TYPE vivienda_persona AS ENUM (
    'propio',
    'rentada',
    'familiares',
    'prestada',
    'otros'
);

CREATE TYPE estado_civil_persona AS ENUM (
    'soltero',
    'casado',
    'union_libre',
    'viudo',
    'otros',
    'no_aplica'
);


CREATE TABLE personas (
    id_persona SERIAL PRIMARY KEY,
    nombre VARCHAR(40) NOT NULL,
    apellido_paterno VARCHAR(30) NOT NULL,
    apellido_materno VARCHAR(30) NOT NULL,
    tipo INTEGER NOT NULL,
    sexo sexo_persona NOT NULL,
    fecha_actualizacion TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    usuario_actualizo INTEGER NOT NULL,
    cp VARCHAR(5) NOT NULL,
    barrio INTEGER NOT NULL,
    ciudad INTEGER NOT NULL,
    calle BIGINT NOT NULL,
    numero_exterior VARCHAR(20) NOT NULL,
    numero_interior VARCHAR(20),
    vivienda vivienda_persona NOT NULL,
    geolocalizacion VARCHAR(50) NOT NULL,
    observaciones_geolocalizacion VARCHAR(100) NOT NULL,
    fecha_nacimiento DATE NOT NULL,
    pais_nacimiento INTEGER NOT NULL,
    estado_civil estado_civil_persona NOT NULL,
    persona_conyuge INTEGER,
    es_embargo_precautorio BOOLEAN NOT NULL,
    bloqueado_autoridad BOOLEAN NOT NULL,
    tercero_autorizado INTEGER NOT NULL,
    FOREIGN KEY (tipo) REFERENCES tipos_persona (id_tipo_persona),
    FOREIGN KEY (usuario_actualizo) REFERENCES usuarios (id),
    FOREIGN KEY (barrio) REFERENCES barrios (id_barrio),
    FOREIGN KEY (ciudad) REFERENCES ciudades (id_ciudad),
    FOREIGN KEY (calle) REFERENCES calles (id_calle),
    FOREIGN KEY (pais_nacimiento) REFERENCES paises (id_pais),
    FOREIGN KEY (persona_conyuge) REFERENCES personas (id_persona) 
);

CREATE INDEX idx_nombre_persona ON personas (nombre);
