CREATE TYPE iva_detalle_poliza AS ENUM (
    'no_aplica',
    'iva0',
    'iva8',
    'iva11',
    'iva16',
    'excento',
    'retenido'
);

CREATE TABLE detalles_poliza (
    id_detalle_poliza SERIAL PRIMARY KEY,
    poliza INTEGER NOT NULL,
    cuenta INTEGER NOT NULL,
    sucursal INTEGER NOT NULL,
    cargo REAL NOT NULL,
    abono REAL NOT NULL,
    proveedor INTEGER NOT NULL,
    concepto VARCHAR(100) NOT NULL,
    iva iva_detalle_poliza NOT NULL,
    FOREIGN KEY (poliza) REFERENCES polizas(id_poliza) ON DELETE RESTRICT,
    FOREIGN KEY (cuenta) REFERENCES cuentas(id_cuenta) ON DELETE RESTRICT,
    FOREIGN KEY (sucursal) REFERENCES sucursales(id_sucursal) ON DELETE RESTRICT,
    FOREIGN KEY (proveedor) REFERENCES proveedores(id_proveedor) ON DELETE RESTRICT
)
