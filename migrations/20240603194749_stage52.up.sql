CREATE TABLE servicios_subgrupo (
    id_servicio_subgrupo SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL
);

CREATE TABLE plazos (
    id_plazo SERIAL PRIMARY KEY,
    nombre VARCHAR(25) NOT NULL,
    nombre_2 VARCHAR(50) NOT NULL,
    dias INTEGER NOT NULL,
    meses INTEGER NOT NULL,
    prestamo INTEGER NOT NULL,
    inversiones INTEGER NOT NULL,
    ponderacion_5c_abonos REAL NOT NULL,
    ponderacion_5c_plazo REAL NOT NULL
);

CREATE TABLE grupos_finalidad (
    id_grupo_finalidad SERIAL PRIMARY KEY,
    nombre VARCHAR(20) NOT NULL
);

INSERT INTO grupos_finalidad (nombre) VALUES 
('PERSONALES'),
('FAMILIARES'),
('TRABAJO'),
('AGRICULTURA'),
('GANADERIA'),
('PESCA'),
('MAQUILA'),
('OTROS');
