CREATE TYPE tipo_contrato_captacion AS ENUM (
    'persona_fisica',
    'grupo'
);

CREATE TABLE contratos_captacion (
    id_contrato_captacion SERIAL PRIMARY KEY,
    id_persona INTEGER NOT NULL,
    cuenta VARCHAR(16) NOT NULL,
    servicio INTEGER NOT NULL,
    fecha DATE NOT NULL,
    no_dejar_retirar_antes_de DATE NOT NULL,
    fecha_contrato DATE NOT NULL,
    monto_autorizado REAL NOT NULL,
    numero_sesion VARCHAR(20) NOT NULL,
    tipo_sesion VARCHAR(20) NOT NULL,
    nombre VARCHAR(50) NOT NULL,
    tipo tipo_contrato_captacion NOT NULL,
    contrato_migrado BIGINT NOT NULL,
    fecha_desbloqueo TIMESTAMP NOT NULL,
    usuario_desbloqueo INTEGER NOT NULL,
    fecha_libera_garantia TIMESTAMP NOT NULL,
    usuario_libera_garantia INTEGER NOT NULL,
    monto_libera_garantia REAL NOT NULL,
    tercero_autorizado BOOLEAN NOT NULL,
    tasa_pactada REAL NOT NULL,
    bloqueada BOOLEAN NOT NULL,
    monto_bloqueado_adicional REAL NOT NULL,
    usuario INTEGER NOT NULL,
    fecha_interes DATE NOT NULL,
    autoriza_cancelacion BOOLEAN NOT NULL,
    usuario_autoriza_cancelacion INTEGER NOT NULL,
    FOREIGN KEY (servicio) REFERENCES servicios(id_servicio) ON DELETE RESTRICT,
    FOREIGN KEY (usuario_desbloqueo) REFERENCES usuarios(id) ON DELETE RESTRICT,
    FOREIGN KEY (usuario_libera_garantia) REFERENCES usuarios(id) ON DELETE RESTRICT,
    FOREIGN KEY (usuario) REFERENCES usuarios(id) ON DELETE RESTRICT,
    FOREIGN KEY (usuario_autoriza_cancelacion) REFERENCES usuarios(id) ON DELETE RESTRICT
);
