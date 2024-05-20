CREATE TABLE frecuencias_empresa (
    id_frecuencias_empresa SERIAL PRIMARY KEY,
    nombre VARCHAR(75) NOT NULL,
    dias INTEGER NOT NULL,
    meses INTEGER NOT NULL
);

CREATE TABLE empresas (
    id_empresa SERIAL NOT NULL,
    nombre VARCHAR(75) NOT NULL,
    domicilio INTEGER NOT NULL,
    telefono VARCHAR(10) NOT NULL,
    empleos_fijos INTEGER NOT NULL,
    frecuencia INTEGER NOT NULL,
    FOREIGN KEY (domicilio) REFERENCES domicilios(id_domicilio) ON DELETE RESTRICT,
    FOREIGN KEY (frecuencia) REFERENCES frecuencias_empresa(id_frecuencias_empresa) ON DELETE RESTRICT
);
