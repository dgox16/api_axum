CREATE TYPE clasificacion_cuenta AS ENUM ('capitulo', 'subcapitulo', 'mayor', 'menor');

CREATE TYPE grupo_cuenta AS ENUM (
    'activo',
    'pasivo',
    'capital',
    'resultado_acreedor',
    'resultado_deudor',
    'orden',
    'puente'
);

CREATE TYPE finalidad_cuenta AS ENUM ('caja', 'banco', 'otros');

CREATE TYPE naturaleza_cuenta AS ENUM ('deudora', 'acreedora');

CREATE TABLE cuentas (
    id_cuenta SERIAL PRIMARY KEY,
    cuenta VARCHAR(30) NOT NULL,
    cuenta_siti VARCHAR(15) NOT NULL,
    nombre VARCHAR(60) NOT NULL,
    clasificacion clasificacion_cuenta NOT NULL,
    grupo grupo_cuenta NOT NULL,
    finalidad finalidad_cuenta NOT NULL,
    naturaleza naturaleza_cuenta NOT NULL,
    afectable BOOLEAN NOT NULL,
    padre VARCHAR(30) NOT NULL,
    nivel INTEGER NOT NULL,
    en_balance BOOLEAN NOT NULL,
    en_catalogo_minimo BOOLEAN NOT NULL,
    nombre_balance VARCHAR(100) NOT NULL,
    nombre_siti VARCHAR(70) NOT NULL,
    cuenta_padre_siti VARCHAR(20) NOT NULL,
    cuenta_agrupar VARCHAR(30) NOT NULL,
    orden_siti INTEGER NOT NULL,
    subcuenta_siti BOOLEAN NOT NULL,
    prorrateo BOOLEAN NOT NULL
);