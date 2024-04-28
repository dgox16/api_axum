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
    numero INTEGER NOT NULL,
    sucursal INTEGER NOT NULL,
    fecha_poliza TIMESTAMP WITH TIME ZONE DEFAULT NOW (),
    fecha_registro_poliza TIMESTAMP WITH TIME ZONE DEFAULT NOW (),
    concepto VARCHAR(150) NOT NULL,
    usuario_autoriza INTEGER NOT NULL,
    usuario_elabora INTEGER NOT NULL,
    aplicacion aplicacion_poliza NOT NULL,
    fuente fuente_poliza NOT NULL,
    automatico BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (sucursal) REFERENCES sucursales(id_sucursal) ON DELETE RESTRICT,
    FOREIGN KEY (usuario_elabora) REFERENCES usuarios(id) ON DELETE RESTRICT,
    FOREIGN KEY (usuario_autoriza) REFERENCES usuarios(id) ON DELETE RESTRICT
)