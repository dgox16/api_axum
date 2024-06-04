CREATE TYPE estado_grupo_prestamo AS ENUM (
    'solicitud',
    'activo',
    'pagado'
);

CREATE TYPE ministrado_grupo_prestamo AS ENUM (
    'efectivo',
    'cheque'
);

CREATE TABLE grupos_prestamo (
    id_grupo_prestamo SERIAL PRIMARY KEY,
    grupo_persona INTEGER NOT NULL,
    contrato INTEGER NOT NULL,
    fecha DATE NOT NULL,
    estado estado_grupo_prestamo NOT NULL,
    servicio INTEGER NOT NULL,
    servicio_subgrupo INTEGER NOT NULL,
    finalidad INTEGER NOT NULL,
    destino VARCHAR(200) NOT NULL,
    numero_abonos INTEGER NOT NULL,
    plazo INTEGER NOT NULL,
    ministrado ministrado_grupo_prestamo NOT NULL,
    FOREIGN KEY (servicio) REFERENCES servicios(id_servicio) ON DELETE RESTRICT,
    FOREIGN KEY (servicio_subgrupo) REFERENCES servicios_subgrupo(id_servicio_subgrupo) ON DELETE RESTRICT,
    FOREIGN KEY (finalidad) REFERENCES finalidades(id_finalidad) ON DELETE RESTRICT,
    FOREIGN KEY (plazo) REFERENCES plazos(id_plazo) ON DELETE RESTRICT
);

CREATE TABLE prestamos_migrado (
    id_prestamo_migrado SERIAL PRIMARY KEY,
    solicitud VARCHAR(15) NOT NULL,
    cuenta VARCHAR(16) NOT NULL,
    nombre_servicio_migrado VARCHAR(35) NOT NULL,
    persona INTEGER NOT NULL,
    sucursal INTEGER NOT NULL,
    grupo_prestamo INTEGER,
    servicio_subgrupo INTEGER,
    usuario INTEGER NOT NULL,
    FOREIGN KEY (persona) REFERENCES personas(id_persona) ON DELETE RESTRICT,
    FOREIGN KEY (usuario) REFERENCES usuarios(id) ON DELETE RESTRICT,
    FOREIGN KEY (sucursal) REFERENCES sucursales(id_sucursal) ON DELETE RESTRICT,
    FOREIGN KEY (grupo_prestamo) REFERENCES grupos_prestamo(id_grupo_prestamo) ON DELETE RESTRICT,
    FOREIGN KEY (servicio_subgrupo) REFERENCES servicios_subgrupo(id_servicio_subgrupo) ON DELETE RESTRICT
);
