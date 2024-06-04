CREATE TYPE al_vencimiento_capital_inversion AS ENUM (
    'renovacion',
    'ahorro'
);

CREATE TYPE al_vencimiento_intereses_inversion AS ENUM (
    'renovacion',
    'ahorro',
    'cuenta_pago_mensual'
);

CREATE TYPE estado_inversion AS ENUM (
    'solicitud',
    'activa',
    'pagada',
    'cancelada'
);

CREATE TABLE inversiones (
    id_inversion SERIAL PRIMARY KEY,
    cuenta VARCHAR(16) NOT NULL,
    servicio INTEGER NOT NULL,
    persona INTEGER NOT NULL,
    tercero_autorizado INTEGER NOT NULL,
    monto REAL NOT NULL,
    monto_garantia REAL NOT NULL,
    fecha_inversion DATE NOT NULL,
    fecha_interes DATE NOT NULL,
    dias INTEGER NOT NULL,
    tasa_interes REAL NOT NULL,
    ide REAL NOT NULL,
    isr REAL NOT NULL,
    al_vencimiento_capital al_vencimiento_capital_inversion NOT NULL,
    al_vencimiento_intereses al_vencimiento_intereses_inversion NOT NULL,
    captacion INTEGER NOT NULL,
    sucursal INTEGER NOT NULL,
    usuario INTEGER NOT NULL,
    ficha_inversion INTEGER NOT NULL,
    ficha_pago INTEGER NOT NULL,
    pagada BOOLEAN NOT NULL,
    estado estado_inversion NOT NULL,
    inversion_anterior INTEGER NOT NULL,
    contrato_migrado VARCHAR(15) NOT NULL,
    caratula_impresa BOOLEAN NOT NULL,
    contrato_impreso BOOLEAN NOT NULL,
    vencida BOOLEAN NOT NULL,
    FOREIGN KEY (servicio) REFERENCES servicios(id_servicio) ON DELETE RESTRICT,
    FOREIGN KEY (persona) REFERENCES personas(id_persona) ON DELETE RESTRICT,
    FOREIGN KEY (sucursal) REFERENCES sucursales(id_sucursal) ON DELETE RESTRICT,
    FOREIGN KEY (usuario) REFERENCES usuarios(id) ON DELETE RESTRICT
)
