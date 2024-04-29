CREATE TYPE tipo_proveedor AS ENUM ('nacional', 'extranjero', 'global');

CREATE TYPE operacion_proveedor AS ENUM (
    'servicios_profesionales',
    'arrendamiento',
    'otros'
);

CREATE TABLE proveedores (
    id_proveedor SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    domicilio INTEGER NOT NULL,
    rfc VARCHAR(15) NOT NULL,
    curp VARCHAR(18) NOT NULL,
    telefono VARCHAR(10) NOT NULL,
    tipo tipo_proveedor NOT NULL,
    operacion operacion_proveedor NOT NULL,
    regimen VARCHAR(50) NOT NULL,
    nombre_extranjero VARCHAR(255),
    pais_residencia INTEGER NOT NULL,
    pais_nacimiento INTEGER NOT NULL,
    banco INTEGER NOT NULL,
    cuenta_clabe VARCHAR(20) NOT NULL,
    FOREIGN KEY (domicilio) REFERENCES domicilios(id_domicilio) ON DELETE RESTRICT,
    FOREIGN KEY (pais_residencia) REFERENCES paises(id_pais) ON DELETE RESTRICT,
    FOREIGN KEY (pais_nacimiento) REFERENCES paises(id_pais) ON DELETE RESTRICT,
    FOREIGN KEY (banco) REFERENCES bancos(id_banco) ON DELETE RESTRICT
);