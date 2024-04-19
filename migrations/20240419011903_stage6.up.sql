CREATE TABLE sucursales (
    id_sucursal SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL,
    encargado INTEGER NOT NULL,
    domicilio INTEGER NOT NULL,
    FOREIGN KEY (encargado) REFERENCES usuarios(id) ON DELETE RESTRICT,
    FOREIGN KEY (domicilio) REFERENCES domicilios(id_domicilio) ON DELETE RESTRICT
);