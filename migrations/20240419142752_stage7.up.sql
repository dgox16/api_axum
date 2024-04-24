CREATE TYPE tipo_poliza AS ENUM ('egreso', 'ingreso', 'diario');

CREATE TYPE aplicacion_poliza AS ENUM (
    'normal',
    'condonacion',
    'cheque_orden',
    'cierre_diario',
    'cierre_mensual',
    'cierre_anual'
);

CREATE TYPE fuente_poliza AS ENUM (
    'operacion',
    'activos',
    'nomina',
    'gastos',
    'pasiva',
    'traslados',
    'traspasos'
);

CREATE TABLE polizas (
    id_poliza SERIAL PRIMARY KEY,
    tipo tipo_poliza NOT NULL,
    numero INTEGER DEFAULT 1,
    sucursal INTEGER NOT NULL,
    fecha_poliza TIMESTAMP WITH TIME ZONE DEFAULT NOW (),
    fecha_registro_poliza TIMESTAMP WITH TIME ZONE DEFAULT NOW (),
    concepto VARCHAR(150) NOT NULL,
    usuario_autoriza INTEGER DEFAULT 1,
    usuario_elabora INTEGER NOT NULL,
    aplicacion aplicacion_poliza DEFAULT 'normal',
    fuente fuente_poliza DEFAULT 'operacion',
    automatico BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (sucursal) REFERENCES sucursales(id_sucursal) ON DELETE RESTRICT,
    FOREIGN KEY (usuario_elabora) REFERENCES usuarios(id) ON DELETE RESTRICT,
    FOREIGN KEY (usuario_autoriza) REFERENCES usuarios(id) ON DELETE RESTRICT
)