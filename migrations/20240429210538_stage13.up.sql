CREATE TYPE iva_detalle_poliza AS ENUM (
    'no_aplica',
    '0',
    '8',
    '11',
    '16',
    'excento',
    'retenido'
);

CREATE TABLE detalles_polizas (
    id_detalle_poliza SERIAL PRIMARY KEY,
    poliza INTEGER NOT NULL,
    cuenta INTEGER NOT NULL,
    sucursal INTEGER NOT NULL,
    cargo DECIMAL(14, 2) NOT NULL,
    abono DECIMAL(14, 2) NOT NULL,
    proveedor INTEGER NOT NULL,
    concepto VARCHAR(100) NOT NULL,
    iva iva_detalle_poliza NOT NULL,
    FOREIGN KEY (poliza) REFERENCES polizas(id_poliza) ON DELETE RESTRICT,
    FOREIGN KEY (cuenta) REFERENCES cuentas(id_cuenta) ON DELETE RESTRICT,
    FOREIGN KEY (sucursal) REFERENCES sucursales(id_sucursal) ON DELETE RESTRICT,
    FOREIGN KEY (proveedor) REFERENCES proveedores(id_proveedor) ON DELETE RESTRICT
)